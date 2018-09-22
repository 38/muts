use std::mem::size_of;
use std::ptr::replace;
use std::ops::{Deref, DerefMut};
use std::fmt::{Display, Formatter, Debug, Error};
use std::result::Result;
use std::marker::{PhantomData};

use crate::{Allocator, Wrapped, AllocResult, AllocateWith};

pub trait Ownership {
    fn get() -> bool;
}
pub trait Mutable { } 
pub struct Owned;
pub struct Borrowed<'a> { m : PhantomData<&'a i32> }
pub struct BorrowedMut<'a> { m : PhantomData<&'a i32> }

impl Ownership for Owned {
    fn get()->bool { true }
}
impl <'a> Ownership for Borrowed<'a> {
    fn get()->bool { false }
}

impl <'a> Ownership for BorrowedMut<'a> {
    fn get()->bool { false }
}

impl Mutable for Owned { }
impl <'a> Mutable for BorrowedMut<'a> { }

pub type My<T, A> = Ptr<T,A,Owned>;

pub type Your<'a, T, A> = Ptr<T,A,Borrowed<'a>>;

pub type YourMut<'a, T, A> = Ptr<T,A,BorrowedMut<'a>>;

pub struct Ptr<T:Sized, A : Allocator, O : Ownership> {
    meta : A::MetaDataType,
    addr : *mut T,
    mark : PhantomData<O>
}

impl <T:Sized, A:Allocator> My<T,A> {
    pub fn try_new(obj : T, alloc_ctx : AllocateWith<A>) -> AllocResult<My<T, A>>
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
            replace(A::translate_mut(mem.0, &mem.1), obj);
        }

        return Ok(My {
            addr : mem.0,
            meta : mem.1,
            mark : PhantomData
        });
    }

    pub fn new(obj : T, _alloc : AllocateWith<A>) -> My<T, A>
    {
        if let Ok(ret) = Self::try_new(obj, _alloc) 
        {
            return ret;
        }
        panic!("Memory Allocation Failure");
    }

    pub fn borrow<'a>(&'a self) -> Your<'a, T, A> 
    {
        return Your {
            addr : self.addr,
            meta : self.meta,
            mark : PhantomData
        };
    }
    
    pub fn borrow_mut<'a>(&'a mut self) -> YourMut<'a, T, A> 
    {
        return YourMut {
            addr : self.addr,
            meta : self.meta,
            mark : PhantomData
        };
    }
}

impl <T:Sized, A:Allocator, O:Ownership> Wrapped for Ptr<T, A, O> {
    type InnerType = T;
}

impl <T:Sized, A:Allocator, O:Ownership> AsRef<T> for Ptr<T, A, O> {
    fn as_ref(&self) -> &T 
    {
        unsafe { A::translate(self.addr, &self.meta).as_ref().unwrap() }
    }
}

impl <T:Sized, A:Allocator, O:Ownership + Mutable> AsMut<T> for Ptr<T, A, O> {
    fn as_mut(&mut self) -> &mut T
    {
        unsafe { A::translate_mut(self.addr, &self.meta).as_mut().unwrap() } 
    }
}

impl <T:Sized, A:Allocator, O : Ownership> Drop for Ptr<T,A,O> {
    fn drop(&mut self)
    {
        if O::get()
        {
            let mem = self.addr as *mut u8;
            A::free(mem, &mut self.meta);
        }
    }
}


impl <T: Sized, A:Allocator, O:Ownership> Deref for Ptr<T, A, O> {
    type Target = T;
    fn deref(&self) -> &T 
    {
        return self.as_ref();
    }
}

impl <T: Sized, A:Allocator, O:Ownership + Mutable> DerefMut for Ptr<T, A, O> {
    fn deref_mut(&mut self) -> &mut T 
    {
        return self.as_mut();
    }
}

impl <T: Sized + Display, A:Allocator, O:Ownership> Display for Ptr<T, A, O> {
    fn fmt(&self, f : &mut Formatter) -> Result<(), Error>
    {
        Display::fmt(self.deref(), f)
    }
}

impl <T: Sized + Debug, A:Allocator, O:Ownership> Debug for Ptr<T, A, O> {
    fn fmt(&self, f : &mut Formatter) -> Result<(), Error>
    {
        Debug::fmt(self.deref(), f)
    }
}

