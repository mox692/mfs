use std::fmt;
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MyError {
    msg: &'static str,
}
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid parameters to Layout::from_size_align")
    }
}
impl MyError {
    pub fn with_msg(msg: &'static str) -> Self {
        Self { msg: msg }
    }
}
