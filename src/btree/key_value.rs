extern crate serde;
use serde::{Serialize, Deserialize};

pub trait KeyType<'d>: Ord + Clone + Serialize + Deserialize<'d> {}
pub trait ValueType<'d>: Ord + Clone + Serialize + Deserialize<'d> {}

impl<'d, T> KeyType<'d> for T where T: Ord + Clone + Serialize + Deserialize<'d>  {}

impl<'d, T> ValueType<'d>  for T where T: Ord + Clone + Serialize + Deserialize<'d>  {}

#[derive(Serialize, Deserialize)]
pub struct KeyValuePair<'d, K: KeyType<'d>, V: ValueType<'d>> {
    key: &'d K,
    value: &'d V,
}

impl<'d, K: KeyType<'d>, V: ValueType<'d>> KeyValuePair<'d, K, V> {
    pub fn new(key: K, value: V) -> Self {
        KeyValuePair { key, value }
    }
}
