use mfs::{FlatFS, MemStorage, MyError, Storage, FS};

#[test]
fn test_storage() {
    let test_data: Vec<(usize, Vec<u8>, usize, usize, Vec<u8>)> = vec![
        (22, vec![0x21, 0x33], 22, 2, vec![0x21, 0x33]),
        (1, vec![0x21, 0x33, 0x31], 3, 1, vec![0x31]),
    ];
    for (
        i,
        (
            input_write_offset,
            input_write_data,
            input_read_offset,
            input_read_size,
            expect_data,
        ),
    ) in test_data.into_iter().enumerate()
    {
        let mut s: MemStorage<usize, usize, Vec<u8>, MyError> = MemStorage::new();
        match s.write(input_write_offset, input_write_data.clone()) {
            | Err(e) => panic!("{}'th case, {:?}", i, e),
            | _ => (),
        }
        let v = s.read(input_read_offset, input_read_size);
        if !v.eq(&expect_data) {
            panic!("{}'th case, expect {:?}, but got {:?}", i, expect_data, v)
        }
    }
}

#[test]
fn test_storage_edge() {
    // size以上の書き込みを行おうとした場合、Errorにして返す
    let mut s: MemStorage<usize, usize, Vec<u8>, MyError> = MemStorage::new();
    match s.write(99999, vec![12, 34, 1, 3]) {
        | Err(_) => (),
        | _ => panic!("expect err, but no error found."),
    }
}

fn test_fs() {
    // storageを初期化
    let s: MemStorage<usize, usize, Vec<u8>, MyError> = MemStorage::new();

    // fsを初期化
    let mut fs = FlatFS::new(s);
    // fsにデータを格納
    let file_name = "test_file";
    let contents = "this is contentes";
    let _ = fs.write(file_name, contents).unwrap();
    if let Some(c) = fs.read(file_name) {
        if c != contents {
            panic!("expect {}, but got {}", contents, c)
        }
    } else {
        panic!("expect {}, but got no content", contents)
    }
}
