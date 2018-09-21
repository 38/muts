pub mod boxed;
pub mod allocator;

pub type AllocResult<T> = ::std::result::Result<T, ()>;

pub trait Allocator {
    type MetaDataType;
    type AllocParam;
    fn alloc(sz  : usize, param : Self::AllocParam) -> AllocResult<(Self::MetaDataType, *mut u8)>;
    fn free(addr : *mut u8, metadata : &mut Self::MetaDataType);
}

pub trait Wrapped {
    type InnerType;
}

#[cfg(test)]
mod tests {
    include!("../test/test.rs");
}
