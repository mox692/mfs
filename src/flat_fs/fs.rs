use super::super::FS;
use crate::{MyError, Storage};
use std::marker::PhantomData;

/// ひとまず、StorageのkeyもvalueもString型で実装してみる
pub struct FlatFS<K, V, E, S, P, Q, D>
where
    S: Storage<P, Q, D, E>,
{
    pub storage: S,
    _marker_k: PhantomData<K>,
    _marker_v: PhantomData<V>,
    _marker_e: PhantomData<E>,
    _marker_p: PhantomData<P>,
    _marker_q: PhantomData<Q>,
    _marker_d: PhantomData<D>,
}

impl<K, V, E, S, P, Q, D> FlatFS<K, V, E, S, P, Q, D>
where
    S: Storage<P, Q, D, E>,
{
    pub fn new(s: S) -> Self {
        Self {
            storage: s,
            _marker_e: PhantomData,
            _marker_k: PhantomData,
            _marker_v: PhantomData,
            _marker_p: PhantomData,
            _marker_q: PhantomData,
            _marker_d: PhantomData,
        }
    }
}

impl<K, V, E, S, P, Q, D>  FS<K, V, E, P, Q> for FlatFS<K, V, E, S, P, Q, D> 
where
    S: Storage<P, Q, D, E>,
{
    // TODO:
    fn write(&mut self, file_name: K, contents: V) -> Result<(), E> {
        Ok(())
    }
    // TODO:
    fn read(&self, file_name: K) -> Option<V> {
        // self.storage.read(self.key_to_offset(file_name), )
        None
    }
    fn key_to_offset(&self, key: K) -> Option<P> {
        None
    }
    fn value_to_size(&self, value: V) -> Q {
        ()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {}
}
