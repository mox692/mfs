use std::fmt;

#[derive(PartialEq, Eq, Debug)]
pub struct MyError {
    pub(crate) msg: String,
}
impl MyError {
    pub(crate) fn with_msg(msg: String) -> Self {
        Self { msg: msg }
    }
}
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("Error: {}", self.msg))
    }
}
impl std::error::Error for MyError {}
