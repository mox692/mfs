#![feature(allocator_api)]
#![feature(slice_ptr_get)]
#![feature(map_try_insert)]
#![feature(unboxed_closures)]
#![feature(never_type)]
mod common;
mod fierarchical_fs;
mod flat_fs;
mod fs;
mod storage;

pub use common::{MyError, Stack};
pub use fierarchical_fs::FierarchicalFS;
pub use flat_fs::FlatFS;
pub use fs::FS;
pub use storage::{MemStorage, Storage};
