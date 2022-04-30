use std::{error::Error, hash::Hash};

// File systemのinterface
// ファイル名(に、よらないかもしれないが、)をとって、contentsを返す.
pub trait FS<K, V, E> {
    fn write(&mut self, file_name: K, contents: V) -> Result<(), Box<dyn Error>>;
    fn read(&self, file_name: K) -> Option<&V>;
}
