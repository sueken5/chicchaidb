use crate::btree::key_value::{KeyType, KeyValuePair, ValueType};
use crate::btree::record_file::RecordFile;
use std::error::Error;

pub struct WAL<K: KeyType, V: ValueType> {
    file: RecordFile<K, V>,
}

impl<K: KeyType, V: ValueType> WAL<K, V> {
    pub fn new(
        file_path: &String,
        key_size: usize,
        value_size: usize,
    ) -> Result<WAL<K, V>, Box<Error>> {
        let file = RecordFile::<K, V>::new(file_path, key_size, value_size)?;
        Ok(WAL { file })
    }

    pub fn insert(&mut self, key: &K, value: &V) -> Result<(), Box<Error>> {
        let kv = KeyValuePair::new(key.clone(), value.clone());
        return self.file.insert(&kv);
    }
}
