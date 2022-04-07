use mfs::{FlatFS, FS};

#[test]
fn test_display() {
    let mut fs = FlatFS::new(100);
    fs.display();
    let _ = fs.write("aa", "aaa");
    let a: Option<usize> = fs.read("aa");
}
