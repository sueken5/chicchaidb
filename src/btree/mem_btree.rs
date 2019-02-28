use crate::btree::key_value::KeyType;
use crate::btree::key_value::ValueType;
use std::collections::BTreeMap;
use std::error::Error;

pub struct MemBtree<K: KeyType, V: ValueType> {
    map: BTreeMap<K, V>,
    count: usize,
}

impl<K: KeyType, V: ValueType> MemBtree<K, V> {
    pub fn new() -> Result<MemBtree<K, V>, Box<Error>> {
        Ok(MemBtree {
            map: BTreeMap::<K, V>::new(),
            count: 0,
        })
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.count += 1;
        self.map.insert(key, value);
    }

    pub fn get(&self, key: K) -> Option<&V> {
        self.map.get(&key)
    }
}
