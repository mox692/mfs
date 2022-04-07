// s: データ表現
// u: なんか数値 or addr (iterで回したい)
pub trait FS<S, U> {
    fn write(&mut self, file_name: S, contents: S) -> Option<U>;
    fn read(&self, file_name: S) -> Option<U>;
}
