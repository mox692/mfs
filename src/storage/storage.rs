

use libc::malloc;
use std::{
    alloc::{Allocator, GlobalAlloc, Layout},
    fmt,
};
use crate::MyError;

pub trait Storage<K, V> {
    fn write(&mut self, key: K, contents: V) -> Result<(), MyError>;
    fn read(&self, key: V) -> Result<V, MyError>;
}

pub struct MemStorage {
    start_addr: *mut u8,
    size: usize,
}
impl<K, V> Storage<K, V> for MemStorage {
    fn write(&mut self, key: K, contents: V) -> Result<(), MyError> {
        Ok(())
    }
    fn read(&self, key: V) -> Result<V, MyError> {
        Err(MyError::with_msg("some error"))
    }
}
impl MemStorage {
    unsafe fn new(size: usize) -> Self {
        let ptr = malloc(size);
        Self {
            start_addr: ptr as *mut u8,
            size: size,
        }
    }
    fn new_with_allocator<T>(
        allocator: T,
        size: usize,
        align: usize,
    ) -> Result<Self, MyError>
    where
        T: Allocator,
    {
        let l = Layout::from_size_align(size, align).unwrap();
        match allocator.allocate(l) {
            | Ok(block) => Ok(Self {
                start_addr: block.as_mut_ptr(),
                size: size,
            }),
            | Err(e) => Err(MyError::with_msg("alloc: error")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
    }
}
