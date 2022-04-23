use std::fmt;
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MyError {
    msg: &'static str,
}
impl MyError {
    pub fn with_msg(msg: &'static str) -> Self {
        Self { msg: msg }
    }
}
