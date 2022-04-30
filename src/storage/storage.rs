use crate::MyError;
use std::{
    alloc::{Allocator, Layout},
    marker::PhantomData,
};

/// storage trait defines the behavior of storage.
pub trait Storage<P, Q, D, E> {
    // 一応仕様的にはwriteするデータのサイズはwriteする側で調整することにしている
    fn write(&mut self, offset: P, data: D) -> Result<(), E>;
    fn read(&self, offset: P, size: Q) -> D;
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
    for MemStorage<usize, usize, Vec<u8>, MyError>
{
    // size以上の書き込みを行おうとした場合、Errorにして返す
    fn write(&mut self, offset: usize, data: Vec<u8>) -> Result<(), MyError> {
        if offset + data.len() > 100000 {
            return Err(MyError::with_msg("out of bound error"));
        }
        for (i, v) in data.into_iter().enumerate() {
            self.mem[offset + i] = v
        }
        Ok(())
    }
    // size以上の読み込みを行おうとした場合、最後の要素まで読んで返す
    fn read(&self, offset: usize, size: usize) -> Vec<u8> {
        let mut s = size;
        if offset + s > 100000 {
            s = 100000 - offset
        }
        let mut vec: Vec<u8> = vec![0; size];
        for i in 0..size {
            vec[i] = self.mem[offset + i]
        }
        vec
    }
}

impl MemStorage<usize, usize, Vec<u8>, MyError> {
    pub fn new() -> Self {
        Self {
            start_addr: 0,
            size: 0,
            mem: [0; 100000],
            _e: PhantomData::<MyError>,
            _p: PhantomData::<usize>,
            _q: PhantomData::<usize>,
            _v: vec![],
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
                _v: vec![],
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
