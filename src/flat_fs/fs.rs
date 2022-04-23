use super::super::FS;
use std::marker::PhantomData;
use crate::MyError;

// K, V共にstringでひとまず実装してしまう
pub struct FlatFS {
}

impl FlatFS {}

impl FS<String, String> for FlatFS {
    // TODO:
    fn write(&mut self, file_name: String, contents: String) -> Result<(), MyError> {
        Ok(())
    }
    // TODO:
    fn read(&self, file_name: String) -> Option<String> {
        Some(String::from("aa"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {}
}
