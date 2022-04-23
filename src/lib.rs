#![feature(allocator_api)]
#![feature(slice_ptr_get)]
mod fierarchical_fs;
mod flat_fs;
mod fs;
mod storage;
mod common;


pub use fierarchical_fs::FierarchicalFS;
pub use flat_fs::FlatFS;
pub use fs::FS;
pub use storage::Storage;
pub use common::MyError;
