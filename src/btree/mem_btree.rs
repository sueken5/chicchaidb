use crate::btree::key_value::KeyType;
use crate::btree::key_value::ValueType;
use std::collections::BTreeMap;
use std::error::Error;

pub struct MemBtree<'d, K: KeyType<'d>, V: ValueType<'d>> {
    map: BTreeMap<&'d K, &'d V>,
    count: usize,
}

impl<'d, K: KeyType<'d>, V: ValueType<'d>> MemBtree<'d, K, V> {
    pub fn new() -> Result<MemBtree<'d, K, V>, Box<Error>> {
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
