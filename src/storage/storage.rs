use crate::MyError;
use libc::malloc;
use std::{
    alloc::{Allocator, Layout},
    marker::PhantomData,
};

/// storage trait defines the behavior of storage.
pub trait Storage<P, Q, D, E> {
    // 一応仕様的にはwriteするデータのサイズはwriteする側で調整することにしている
    fn write(&mut self, offset: P, data: D) -> Result<(), E>;
    fn read(&self, offset: P, size: Q) -> Option<D>;
}

pub struct MemStorage<P, Q, D, E> {
    start_addr: *mut u8,
    size: usize,
    _e: PhantomData<E>,
    _p: PhantomData<P>,
    _q: PhantomData<Q>,
    _v: D,
}

// ErrorはMyErrorで実装している
impl<P, Q, D> Storage<P, Q, D, MyError>  for MemStorage<P, Q, D, MyError> {
    // TODO
    fn write(&mut self, offset: P, data: D) -> Result<(), MyError> {
        Ok(())
    }
    // TODO
    fn read(&self, offset: P, size: Q) -> Option<D> {
        None
    }
}

impl<P, Q, D> MemStorage<P, Q, D, MyError> {
    // new use c_malloc to allocate memory.
    // valueの情報だけ、初期化の際にUserに入れてもらう. (もっといい方法がありそうだが...)
    pub fn new(size: usize, value_info: D) -> Self {
        unsafe {
            let ptr = malloc(size);
            Self {
                start_addr: ptr as *mut u8,
                size: size,
                _e: PhantomData::<MyError>,
                _p: PhantomData::<P>,
                _q: PhantomData::<Q>,
                _v: value_info,
            }
        }
    }
    // new_with_allocator use provided allocator to allocate memory.
    fn new_with_allocator<K>(
        allocator: K,
        size: usize,
        align: usize,
        value_info: D,
    ) -> Result<Self, MyError>
    where
        K: Allocator,
    {
        let l = Layout::from_size_align(size, align).unwrap();
        match allocator.allocate(l) {
            | Ok(block) => Ok(Self {
                start_addr: block.as_mut_ptr(),
                size: size,
                _e: PhantomData::<MyError>,
                _p: PhantomData::<P>,
                _q: PhantomData::<Q>,
                _v: value_info,
            }),
            | Err(e) => Err(MyError::with_msg("alloc: error")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {}
}
