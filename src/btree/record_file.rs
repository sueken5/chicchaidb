use crate::btree::key_value::{KeyType, KeyValuePair, ValueType};
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
extern crate bincode;
use bincode::serialize;
use std::marker::PhantomData;

pub struct RecordFile<'d, K: KeyType<'d>, V: ValueType<'d>> {
    file: File,
    key_size: usize,
    value_size: usize,
    _key_marker: PhantomData<&'d K>,
    _value_marker: PhantomData<&'d V>,
}

impl<'d, K: KeyType<'d>, V: ValueType<'d>> RecordFile<'d, K, V> {
    pub fn new(
        file_path: &String,
        key_size: usize,
        value_size: usize,
    ) -> Result<RecordFile<'d, K, V>, Box<Error>> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;
        Ok(RecordFile {
            file,
            key_size,
            value_size,
            _key_marker: PhantomData,
            _value_marker: PhantomData,
        })
    }

    pub fn is_new(&self) -> Result<bool, Box<Error>> {
        let l = self.file.metadata()?;
        if l.len() == 0 {
           return Ok(true);
        }

        Ok(false)
    }

    pub fn insert(&mut self, kv: &KeyValuePair<'d, K, V>) -> Result<(), Box<Error>> {
        let record_size = self.key_size + self.value_size;
        let mut buff = serialize(&kv)?;

        if buff.len() > record_size {
            return Err(From::from("invalid key and value"));
        } else {
            let diff = (self.key_size + self.value_size) - buff.len();
            buff.extend(vec![0; diff]);
        }

        match self.file.write_all(&buff) {
            Ok(_) => Ok(()),
            Err(e) => Err(From::from(e)),
        }
    }
}
