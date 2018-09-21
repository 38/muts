use std::ptr::null_mut;
use crate::{Allocator, AllocResult};

pub struct CMalloc;

extern {
    fn malloc(size : usize) -> *mut u8;
    fn free(ptr : *mut u8);
}

impl Allocator for CMalloc {
    type MetaDataType = ();
    type AllocParam = ();
    fn alloc(sz  : usize, _p : ()) -> AllocResult<((), *mut u8)>
    {
        let ret = unsafe { malloc(sz) } ;
        if null_mut() == ret { Err(()) } else { Ok(((),ret)) }
    }
    fn free(addr : *mut u8, _mt : &mut ())
    {
        unsafe { free(addr) }
    }
}
