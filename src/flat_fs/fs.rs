use super::super::FS;
use std::marker::PhantomData;

pub struct FlatFS<T> {
    mem_size: usize,
    _marker: PhantomData<T>,
}

impl<T> FlatFS<T> {
    pub fn new(mem_size: usize) -> Self {
        return Self {
            mem_size: mem_size,
            _marker: PhantomData,
        };
    }
    pub fn display(&self) {
        for i in 0..self.mem_size {
            print!("| {: ^4} ", i);
            if i % 10 == 9 {
                println!("|");
            }
        }
        println!("|");
    }
}

impl<S> FS<S, usize> for FlatFS<S> {
    fn write(&mut self, file_name: S, contents: S) -> Option<usize> {
        Some(self.mem_size)
    }
    fn read(&self, file_name: S) -> Option<usize> {
        Some(self.mem_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {}
}
