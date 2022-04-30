use super::super::FS;
use crate::{MemStorage, MyError, Storage};
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    fmt::Debug,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

/// ひとまず、StorageのkeyもvalueもString型で実装してみる
pub struct FlatFS<K, V, E, S> {
    pub storage: S,
    pub file_map: HashMap<K, V>,
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
            file_map: HashMap::new(),
            _marker_e: PhantomData,
            _marker_k: PhantomData::<K>,
            _marker_v: PhantomData::<V>,
        }
    }
}

impl<'a, K, V> FS<K, V, MyError>
    for FlatFS<K, V, MyError, MemStorage<usize, usize, Vec<u8>, MyError>>
where
    K: Hash + Eq + Debug,
    V: ToString + Debug,
{
    fn write(&mut self, file_name: K, contents: V) -> Result<(), MyError> {
        if let Err(e) = self.file_map.try_insert(file_name, contents) {
            return Err(MyError::with_msg(String::from(format!("{}", e))));
        }

        // let mut f = DefaultHasher::new();
        // file_name.hash(&mut f);
        // let mut pos = 0;
        // f.write_usize(pos);
        // let str = contents.to_string();
        // let a = str.as_bytes().to_vec();
        // self.storage.write(pos, a);
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
