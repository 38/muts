use crate::boxed::{Box, AllocateWith};
use crate::allocator::c_malloc::CMalloc;
use crate::{Allocator, AllocResult};

#[test]
fn test_box_with_malloc()
{
    let mut test = Box::new([1i32;1024], AllocateWith::<CMalloc>::get(()));

    for i in 0..1024
    {
        test[i] = (i * 3) as i32;
    }

    for i in 0..1024
    {
        assert_eq!(test[i], (i * 3) as i32);
    }
}

struct TestAlloc<'a> {
    cnt : std::cell::Cell<u32>,
    pd  : ::std::marker::PhantomData<&'a u32>
}

impl <'a> Allocator for TestAlloc<'a> {
    type MetaDataType = &'a TestAlloc<'a>;
    type AllocParam   = &'a TestAlloc<'a>;
    fn alloc(sz  : usize, p : Self::MetaDataType) -> AllocResult<(Self::MetaDataType, *mut u8)> 
    {
        if let Ok((_, ptr)) = CMalloc::alloc(sz, ())
        {
            p.cnt.set(p.cnt.get() + 1);
            return Ok((p, ptr));
        }
        return Err(());
    }
    fn free(addr : *mut u8, mt : &mut Self::AllocParam)
    {
        mt.cnt.set(mt.cnt.get() - 1);
        CMalloc::free(addr, &mut ());
    }
}

#[test]
fn test_box_customized_alloc()
{
    let allocator = TestAlloc {
        cnt : std::cell::Cell::new(0),
        pd  : ::std::marker::PhantomData
    };

    {

        let mut test = Box::new([1i32;1024], AllocateWith::<TestAlloc>::get(&allocator));
        
        for i in 0..1024
        {
            test[i] = (i * 3) as i32;
        }

        for i in 0..1024
        {
            assert_eq!(test[i], (i * 3) as i32);
        }

        assert_eq!(allocator.cnt.get(), 1);

        {
            let another = Box::new([1i32;1024], AllocateWith::<TestAlloc>::get(&allocator));
            assert_eq!(allocator.cnt.get(), 2);
            assert_eq!(another.iter().fold(0, |x,y| x+y), 1024);
        }
        
        assert_eq!(allocator.cnt.get(), 1);
    }

    assert_eq!(allocator.cnt.get(), 0);

}
