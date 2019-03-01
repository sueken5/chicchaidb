extern crate serde;
use serde::Serialize;
extern crate bincode;
use bincode::serialize;

use crate::btree::key_value::{KeyType, ValueType};
use std::error::Error;
use std::marker::PhantomData;
use std::fs::File;
use std::io::Write;
use std::fs::OpenOptions;

const DB_NAME: &'static str = "chicchaidb";
const VERSION: u8 = 0x01;
// FIXME:
const BLOCK_SIZE: usize = 4096;

///
/// file layout
///
/// + 0 -----------------------------------------------+
///  metadata
/// + 4096 --------------------------------------------+
/// ...data
/// +--------------------------------------------------+
///

pub struct DiskBtree<K: KeyType, V: ValueType> {
    file: File,
    key_size: usize,
    value_size: usize,
    _key_marker: PhantomData<K>,
    _value_marker: PhantomData<V>,
}

#[derive(Serialize)]
struct MetaData {
    name: String,
    version: u8,
}

impl<K: KeyType, V: ValueType> DiskBtree<K, V> {
    pub fn new(
        file_path: &String,
        key_size: usize,
        value_size: usize,
    ) -> Result<DiskBtree<K, V>, Box<Error>> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;

        let mut b = DiskBtree {
            file,
            key_size,
            value_size,
            _key_marker: PhantomData,
            _value_marker: PhantomData,
        };

        let is_new = match b.is_new() {
            Ok(v) => v,
            Err(e) => {
                return Err(e);
            }
        };

        if is_new {
            if let Err(e) = b.initialize() {
                return Err(e);
            }
            println!("initializing success!")
        }

        Ok(b)
    }

    fn is_new(&self) -> Result<bool, Box<Error>> {
        let l = self.file.metadata()?;
        if l.len() == 0 {
            return Ok(true);
        }

        Ok(false)
    }

    fn initialize(&mut self) -> Result<(), Box<Error>> {
        let m = MetaData {
            name: DB_NAME.to_string(),
            version: VERSION,
        };

        let mut buff = serialize(&m)?;

        if buff.len() > BLOCK_SIZE {
            return Err(From::from("invalid metadata"));
        } else {
            let diff = BLOCK_SIZE - buff.len();
            buff.extend(vec![0; diff]);
        }

        match self.file.write_all(&buff) {
            Ok(_) => Ok(()),
            Err(e) => Err(From::from(e)),
        }
    }

    fn mapping(&self) -> Result<(), Box<Error>> {
        Ok(())
    }
}
