use std::error::Error;
mod key_value;
use key_value::{KeyType, ValueType};
mod disk_btree;
use disk_btree::DiskBtree;
mod mem_btree;
use mem_btree::MemBtree;
mod wal;
use wal::WAL;
mod record_file;

pub struct Btree<K: KeyType, V: ValueType> {
    key_size: usize,
    value_size: usize,
    disk_btree: DiskBtree<K, V>,
    mem_btree: MemBtree<K, V>,
    wal: WAL<K, V>,
}

impl<K: KeyType, V: ValueType> Btree<K, V> {
    pub fn new(
        btree_file_path: &String,
        wal_file_path: &String,
        key_size: usize,
        value_size: usize,
    ) -> Result<Btree<K, V>, Box<Error>> {
        let dt = DiskBtree::<K, V>::new(&btree_file_path, key_size, value_size)?;
        let mt = MemBtree::<K, V>::new()?;
        let wal = WAL::<K, V>::new(&wal_file_path, key_size, value_size)?;

        Ok(Btree {
            key_size,
            value_size,
            disk_btree: dt,
            mem_btree: mt,
            wal,
        })
    }

    pub fn insert(&mut self, key: K, value: V) -> Result<(), Box<Error>> {
        if let Err(e) = self.wal.insert(&key, &value) {
            return Err(e);
        }

        self.mem_btree.insert(key, value);
        Ok(())
    }

    pub fn get(&self, key: K) -> Option<&V> {
        self.mem_btree.get(key)
    }

    fn merge(&mut self) -> Result<(), Box<Error>> {
        Ok(())
    }

    pub fn close(&mut self) -> Result<(), Box<Error>> {
        if let Err(e) = self.merge() {
            return Err(e);
        }

        Ok(())
    }
}
