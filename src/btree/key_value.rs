extern crate serde;
use serde::Serialize;

pub trait KeyType: Ord + Clone + Serialize {}
pub trait ValueType: Ord + Clone + Serialize {}

impl<T> KeyType for T where T: Ord + Clone + Serialize {}

impl<T> ValueType for T where T: Ord + Clone + Serialize {}

#[derive(Serialize)]
pub struct KeyValuePair<K: KeyType, V: ValueType> {
    key: K,
    value: V,
}

impl<K: KeyType, V: ValueType> KeyValuePair<K, V> {
    pub fn new(key: K, value: V) -> Self {
        KeyValuePair { key, value }
    }
}
