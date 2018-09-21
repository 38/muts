use std::mem::size_of;
use std::ptr::copy;
use std::ops::{Deref, DerefMut};
use std::fmt::{Display, Formatter, Debug, Error};
use std::result::Result;

use crate::{Allocator, Wrapped, AllocResult};

pub struct Box<T:Sized + Copy, A : Allocator> {
    meta : A::MetaDataType,
    addr : *mut T
}

pub struct AllocateWith<T:Allocator> {
    param : T::AllocParam
}

impl <T:Allocator> AllocateWith<T> {
    pub fn get(p : T::AllocParam) -> Self 
    {
        Self {
            param : p
        }
    }
}


impl <T:Sized + Copy, A:Allocator> Box<T,A> {
    pub fn try_new(obj : T, alloc_ctx : AllocateWith<A>) -> AllocResult<Box<T, A>>
    {
        let mem = { 
            if let Ok((meta, res)) = A::alloc(size_of::<T>(), alloc_ctx.param) 
            {
                (res as *mut T, meta)
            }
            else
            {
                return Err(());
            }
        };

        unsafe {
            copy(&obj as *const T, mem.0, 1);
        }

        return Ok(Box {
            addr : mem.0,
            meta : mem.1
        });
    }

    pub fn new(obj : T, _alloc : AllocateWith<A>) -> Box<T, A>
    {
        if let Ok(ret) = Self::try_new(obj, _alloc) 
        {
            return ret;
        }
        panic!("Memory Allocation Failure");
    }
}

impl <T:Sized + Copy, A:Allocator> Wrapped for Box<T, A> {
    type InnerType = T;
}

impl <T:Sized + Copy, A:Allocator> AsRef<T> for Box<T, A> {
    fn as_ref(&self) -> &T 
    {
        unsafe { self.addr.as_ref().unwrap() }
    }
}

impl <T:Sized + Copy, A:Allocator> AsMut<T> for Box<T, A> {
    fn as_mut(&mut self) -> &mut T
    {
        unsafe { self.addr.as_mut().unwrap() } 
    }
}

impl <T:Sized + Copy, A:Allocator> Drop for Box<T,A> {
    fn drop(&mut self)
    {
        let mem = self.addr as *mut u8;
        A::free(mem, &mut self.meta);
    }
}

impl <T: Sized + Copy, A:Allocator> Deref for Box<T,A> {
    type Target = T;
    fn deref(&self) -> &T 
    {
        return self.as_ref();
    }
}

impl <T: Sized + Copy, A:Allocator> DerefMut for Box<T,A> {
    fn deref_mut(&mut self) -> &mut T 
    {
        return self.as_mut();
    }
}

impl <T: Sized + Copy + Display, A:Allocator> Display for Box<T,A> {
    fn fmt(&self, f : &mut Formatter) -> Result<(), Error>
    {
        Display::fmt(self.deref(), f)
    }
}

impl <T: Sized + Copy + Debug, A:Allocator> Debug for Box<T,A> {
    fn fmt(&self, f : &mut Formatter) -> Result<(), Error>
    {
        Debug::fmt(self.deref(), f)
    }
}

