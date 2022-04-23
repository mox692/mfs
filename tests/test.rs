use mfs::{FS, FlatFS, MemStorage, MyError};

#[test]
fn test_init() {
    // storageを初期化
    let s :MemStorage<usize, usize, String, MyError> = MemStorage::new(1000, String::from("aaa"));
    // fsを初期化
    let mut fs = FlatFS::new(s);
    let a = fs.write(213, "cont").unwrap();
    let got = fs.read(2);
}
