use crate::MyError;

// File systemのinterface
// ファイル名(に、よらないかもしれないが、)をとって、contentsを返す.
pub trait FS<K, V, E, P, Q>  {
    fn write(&mut self, file_name: K, contents: V) -> Result<(), E>;
    fn read(&self, file_name: K) -> Option<V>;
    fn key_to_offset(&self, key: K) -> Option<P>;
    fn value_to_size(&self, value: V) -> Q;
}
