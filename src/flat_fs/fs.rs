use super::super::FS;
use crate::{MemStorage, MyError,  Storage};
use std::{
    collections::HashMap, error::Error, fmt::Debug, hash::Hash, marker::PhantomData,
};

pub struct Stack {}
impl Stack {
    pub fn new() -> Self{
        Self {}
    }
}
/// ひとまず、StorageのkeyもvalueもString型で実装してみる
pub struct FlatFS<K, V, I, E, S> {
    pub storage: S,
    // file名をキーとして、そのファイルのメタ情報を保持しているHash Table.
    pub file_map: HashMap<K, I>,
    // blockのindex - nodeInfo のHashTable
    pub free_storage_stack: Stack,
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

// offsetはblock indexを表す.
impl FileNode<usize, usize> {
    pub fn new(offset: usize, size: usize) -> Self {
        Self {
            offset: offset,
            size: size,
        }
    }
}

impl<K, E, I> FlatFS<K, Vec<u8>, I, E, MemStorage<usize, usize, Vec<u8>, MyError>> {
    // MEMO: ひとまず固定.
    const BLOCK_SIZE: usize = 1000;
    pub fn new(s: MemStorage<usize, usize, Vec<u8>, MyError>) -> Self {
        let block_num = MemStorage::STORAGE_SIZE / Self::BLOCK_SIZE;
        Self {
            storage: s,
            // ファイル名 - Filenode
            file_map: HashMap::new(),
            free_storage_stack: Stack::new(),
            _marker_e: PhantomData,
            _marker_k: PhantomData::<K>,
            _marker_v: PhantomData::<Vec<u8>>,
        }
    }

    // TODO: 新しく空いてる領域を探してnodeを作成
    // stackの先頭から空いてるindexを取ってくる
    fn new_file_node(&self) -> Result<FileNode<usize, usize>, Box<dyn Error>> {
        Ok(FileNode::new(0, 0))
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
        let info = match self.file_map.get(&file_name) {
            // TODO: hashmapから取得するときcloneすることをどこかに明記する
            | Some(i) => i.clone(),
            | None => match self.new_file_node() {
                | Err(e) => return Err(Box::<dyn Error>::from(e)),
                | Ok(n) => n,
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
