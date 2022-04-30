use super::super::FS;
use crate::{MemStorage, MyError, Storage};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

/// ひとまず、StorageのkeyもvalueもString型で実装してみる
pub struct FlatFS<K, V, E, S> {
    pub storage: S,
    _marker_k: PhantomData<K>,
    _marker_v: PhantomData<V>,
    _marker_e: PhantomData<E>,
}

//
// 以下は具体実装
//

impl<'a, K, V, E> FlatFS<K, V, E, MemStorage<usize, usize, Vec<u8>, MyError>> {
    pub fn new(s: MemStorage<usize, usize, Vec<u8>, MyError>) -> Self {
        Self {
            storage: s,
            _marker_e: PhantomData,
            _marker_k: PhantomData::<K>,
            _marker_v: PhantomData::<V>,
        }
    }
}

impl<'a, K, V> FS<K, V, MyError>
    for FlatFS<K, V, MyError, MemStorage<usize, usize, Vec<u8>, MyError>>
where
    K: Hash,
    V: ToString,
{
    // TODO:
    fn write(&mut self, file_name: K, contents: V) -> Result<(), MyError> {
        let mut f = DefaultHasher::new();
        file_name.hash(&mut f);
        let mut pos = 0;
        f.write_usize(pos);
        let str = contents.to_string();
        let a = str.as_bytes().to_vec();
        self.storage.write(pos, a);
        Ok(())
    }
    // TODO:
    fn read(&self, file_name: K) -> Option<V> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {}
}
