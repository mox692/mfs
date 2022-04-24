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
    mem: [u8; 100000],
    start_addr: P,
    size: Q,
    _e: PhantomData<E>,
    _p: PhantomData<P>,
    _q: PhantomData<Q>,
    _v: D,
}

//
// 以下は具体実装
//
impl Storage<usize, usize, Vec<u8>, MyError>
    for MemStorage<usize, usize, String, MyError>
{
    // TODO
    fn write(&mut self, offset: usize, data: Vec<u8>) -> Result<(), MyError> {
        // self.mem[3] = data[1];
        let pos = self.start_addr + offset;
        Ok(())
    }
    // TODO
    fn read(&self, offset: usize, size: usize) -> Option<Vec<u8>> {
        None
    }
}

impl<'a> MemStorage<usize, usize, String, MyError> {
    // new use c_malloc to allocate memory.
    // valueの情報だけ、初期化の際にUserに入れてもらう. (もっといい方法がありそうだが...)
    pub fn new() -> Self {
        unsafe {
            Self {
                start_addr: 0,
                size: 0,
                mem: [0; 100000],
                _e: PhantomData::<MyError>,
                _p: PhantomData::<usize>,
                _q: PhantomData::<usize>,
                _v: String::from(""),
            }
        }
    }
    // new_with_allocator use provided allocator to allocate memory.
    //　使わない
    fn new_with_allocator<K>(
        allocator: K,
        size: usize,
        align: usize,
        value_info: String,
    ) -> Result<Self, MyError>
    where
        K: Allocator,
    {
        let l = Layout::from_size_align(size, align).unwrap();
        match allocator.allocate(l) {
            | Ok(block) => Ok(Self {
                mem: [0; 100000],
                start_addr: block.as_mut_ptr() as usize,
                size: size,
                _e: PhantomData::<MyError>,
                _p: PhantomData::<usize>,
                _q: PhantomData::<usize>,
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
