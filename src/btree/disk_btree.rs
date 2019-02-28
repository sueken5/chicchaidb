use crate::btree::key_value::{KeyType, ValueType};
use crate::btree::record_file::RecordFile;
use std::error::Error;

pub struct DiskBtree<K: KeyType, V: ValueType> {
    file: RecordFile<K, V>,
}

impl<K: KeyType, V: ValueType> DiskBtree<K, V> {
    pub fn new(
        file_path: &String,
        key_size: usize,
        value_size: usize,
    ) -> Result<DiskBtree<K, V>, Box<Error>> {
        let file = RecordFile::<K, V>::new(&file_path, key_size, value_size)?;
        Ok(DiskBtree { file })
    }
}
