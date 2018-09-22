extern crate muts;
use muts::{Allocator, AllocateWith, AllocResult};
use muts::ptr::My;
use muts::allocator::cmalloc::CMalloc;

struct AA {}

impl Allocator for AA {
    type MetaDataType = ();
    type AllocParam   = ();
    
    fn alloc(sz  : usize, _p : Self::MetaDataType) -> AllocResult<(Self::MetaDataType, *mut u8)> 
    {
        if let Ok((_, ptr)) = CMalloc::alloc(sz, ())
        {
            return Ok(((), unsafe{ptr.offset(0x1000000)}));
        }
        return Err(());
    }
    fn free(addr : *mut u8, _mt : &mut Self::AllocParam)
    {
        CMalloc::free(unsafe{addr.offset(-0x1000000)}, &mut ());
    } 

    fn translate<T>(addr : *const T, _mt: &Self::MetaDataType) -> *const T 
    {
        let p = addr as *const u8;
        return unsafe{p.offset(-0x1000000)} as *const T
    }
    fn translate_mut<T>(addr : *mut T, _mt : &Self::MetaDataType) -> *mut T
    {
        let p = addr as *mut u8;
        return unsafe{p.offset(-0x1000000)} as *mut T
    }
}

struct TreeNode<A : Allocator> {
    val  : i32,
    left : Option<My<TreeNode<A>, A>>,
    right: Option<My<TreeNode<A>, A>>
}

fn main() {
    let node = My::new(TreeNode::<AA> {
        val : 2,
        left: None,
        right:None
    }, AllocateWith::<AA>::with_args(()));

    let right = My::new(TreeNode::<AA> {
        val : 3,
        left: None,
        right:None
    }, AllocateWith::<AA>::with_args(()));


    let tree = My::new(TreeNode::<AA> {
        val : 1,
        left : Some(node),
        right : Some(right)
    }, AllocateWith::<AA>::with_args(()));

    println!("{}", tree.val);
    println!("{}", tree.left.as_ref().unwrap().val);
    println!("{}", tree.right.as_ref().unwrap().val);
}
