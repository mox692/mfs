use mfs::{FlatFS, MemStorage, MyError, FS};

#[test]
fn test_init() {
    // storageを初期化
    let s: MemStorage<usize, usize, String, MyError> =
        MemStorage::new();

    // fsを初期化
    let mut fs = FlatFS::new(s);
    let a = fs.write("fads", 2);
    let got = fs.read("fasd");
}
