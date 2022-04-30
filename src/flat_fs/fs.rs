use super::super::FS;
use crate::{MemStorage, MyError };
use std::{
    collections::{HashMap},
    error::Error,
    fmt::Debug,
    hash::{Hash},
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

impl<K, V, E> FlatFS<K, V, E, MemStorage<usize, usize, Vec<u8>, MyError>> {
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

impl<K, V> FS<K, V, MyError>
    for FlatFS<K, V, MyError, MemStorage<usize, usize, Vec<u8>, MyError>>
where
    K: Hash + Eq + Debug,
    V: Debug,
{
    fn write(&mut self, file_name: K, contents: V) -> Result<(), Box<dyn Error>> {
        // TODO: valueに、storageの位置情報をいれる、storageへの書き込み処理
        if let Err(e) = self.file_map.try_insert(file_name, contents) {
            // TODO: eをそのまま渡そうとするとErrorになる.
            return Err(Box::<dyn Error>::from(e.to_string()));
        }
        Ok(())
    }
    fn read(&self, file_name: K) -> Option<&V> {
        self.file_map.get(&file_name) 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let s: MemStorage<usize, usize, Vec<u8>, MyError> = MemStorage::new();
        let mut fs = FlatFS::new(s);
        let file_name = "aaa".to_string();
        let content = "yey".to_string();
        let _ = fs.write(file_name, content.clone());
        let a = fs.read("aaa".to_string()).unwrap();
        assert_eq!(a.clone(), content)
    }
}
