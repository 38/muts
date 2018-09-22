use std::marker::Copy;
pub mod ptr;
pub mod allocator;

pub type AllocResult<T> = ::std::result::Result<T, ()>;

pub trait Allocator {
    type MetaDataType : Copy;
    type AllocParam;
    fn alloc(sz  : usize, param : Self::AllocParam) -> AllocResult<(Self::MetaDataType, *mut u8)>;
    fn free(addr : *mut u8, metadata : &mut Self::MetaDataType);
    fn translate<T>(addr : *const T, _mt: &Self::MetaDataType) -> *const T { addr }
    fn translate_mut<T>(addr : *mut T, _mt : &Self::MetaDataType) -> *mut T { addr }
}

pub struct AllocateWith<T:Allocator> {
    param : T::AllocParam
}

impl <T:Allocator> AllocateWith<T> {
    pub fn with_args(p : T::AllocParam) -> Self 
    {
        Self {
            param : p
        }
    }
}

pub trait Wrapped {
    type InnerType;
}

#[cfg(test)]
mod tests {
    include!("../test/test.rs");
}
