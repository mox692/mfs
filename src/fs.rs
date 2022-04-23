use crate::MyError;

// File systemのinterface
// ファイル名(に、よらないかもしれないが、)をとって、contentsを返す.
pub trait FS<K, V> {
    fn write(&mut self, file_name: K, contents: V) -> Result<(), MyError>;
    fn read(&self, file_name: K) -> Option<V>;
}
