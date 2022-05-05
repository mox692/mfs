use super::super::FS;
use crate::{MemStorage, MyError, Storage, Stack};
use std::{
    collections::HashMap, error::Error, fmt::Debug, hash::Hash, marker::PhantomData,
};

/// ひとまず、StorageのkeyもvalueもString型で実装してみる
pub struct FlatFS<K, V, I, E, S> {
    pub storage: S,
    // file名をキーとして、そのファイルのメタ情報を保持しているHash Table.
    pub file_map: HashMap<K, I>,
    // blockのindex - nodeInfo のHashTable
    pub free_storage_stack: Stack<I>,
    // さしあたり今の実装では、newするタイミングで固定のblock sizeを与える
    _marker_k: PhantomData<K>,
    _marker_v: PhantomData<V>,
    _marker_e: PhantomData<E>,
}
#[derive(Clone, Copy)]
pub struct FileNode<P, Q> {
    offset: P,
    size: Q,
}

//
// 以下は具体実装
//

// Iの具体実装.
// offsetはblock indexを表す.
impl FileNode<usize, usize> {
    pub fn new(offset: usize, size: usize) -> Self {
        Self {
            offset: offset,
            size: size,
        }
    }
    pub fn new_vec_from_block_size(block_size: usize, block_num: usize) -> Vec<Self> {
        let mut vec:Vec<Self> = vec![Self::default();block_num];
        let mut cur_off = 0;
        for i in 0..block_num {
            let n = FileNode::new(cur_off, block_size);
            vec[i] = n;
            cur_off += block_size;
        }
        vec
    }
}

impl Default for FileNode<usize, usize> {
    fn default() -> Self {
        Self { offset: 0, size: 0}
    }
}

impl<K, E> FlatFS<K, Vec<u8>, FileNode<usize, usize>, E, MemStorage<usize, usize, Vec<u8>, MyError>> {
    // MEMO: ひとまず固定.
    const BLOCK_SIZE: usize = 1000;
    pub fn new(s: MemStorage<usize, usize, Vec<u8>, MyError>) -> Self {
        let block_num = MemStorage::STORAGE_SIZE / Self::BLOCK_SIZE;
        let fnodes = FileNode::new_vec_from_block_size(Self::BLOCK_SIZE, block_num);
        Self {
            storage: s,
            // ファイル名 - Filenode
            file_map: HashMap::new(),
            free_storage_stack: Stack::new_with_init_data(fnodes),
            _marker_e: PhantomData,
            _marker_k: PhantomData::<K>,
            _marker_v: PhantomData::<Vec<u8>>,
        }
    }
}

impl<K> FS<K, Vec<u8>, MyError>
    for FlatFS<
        K,
        Vec<u8>,
        FileNode<usize, usize>,
        MyError,
        MemStorage<usize, usize, Vec<u8>, MyError>,
    >
where
    K: Hash + Eq + Debug,
{
    // file_nameが存在しなかったら、そのファイルを新規で作成
    fn write(&mut self, file_name: K, contents: Vec<u8>) -> Result<(), Box<dyn Error>> {
        // let a = self.file_map.get()
        // match a {
        //
        //        }
        let info = match self.file_map.get(&file_name) {
            // TODO: hashmapから取得するときcloneすることをどこかに明記する
            | Some(i) => i.clone(),
            | None => {
                let (s, n) = self.free_storage_stack.clone().pop();
                self.free_storage_stack = s;
                match n {
                    Some(v) => v,
                    None => return Err(Box::<dyn Error>::from(MyError::with_msg("no free disk stace found!!".to_string()))),
                }
            },
        };
        self.storage.write(info.offset, contents)
    }

    fn read(&self, file_name: K) -> Option<Vec<u8>> {
        let info = match self.file_map.get(&file_name) {
            | Some(i) => i,
            | None => return None,
        };
        Some(self.storage.read(info.offset, info.size))
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
        let content = vec![21, 32];
        let _ = fs.write(file_name, content.clone());
        // let a = fs.read("aaa".to_string()).unwrap();
        // assert_eq!(a.clone(), content)
    }
}
