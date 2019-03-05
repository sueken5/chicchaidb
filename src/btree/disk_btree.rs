extern crate serde;
use serde::{Serialize, Deserialize};
extern crate bincode;
use bincode::{serialize, deserialize};

use crate::btree::key_value::{KeyType, ValueType};
use std::error::Error;
use std::marker::PhantomData;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::cmp::Ordering;

const DB_NAME: &'static str = "chicchaidb";
const VERSION: u8 = 0x01;
// FIXME:
const BLOCK_SIZE: usize = 4096;

///
/// file layout
///
/// + 0 -----------------------------------------------+
///  metadata
/// + 4095 --------------------------------------------+
/// + 4096 --------------------------------------------+
/// ...data
/// +--------------------------------------------------+
///

struct BTree<'d, K: KeyType<'d>, V: ValueType<'d>> {
    root: Node<'d, K, V>,
    count: usize,
    file: File,
}

impl <'d, K: KeyType<'d>, V: ValueType<'d>> BTree<'d, K, V> {
    fn new(file_path: String) -> Result<BTree<'d, K, V>, Box<Error>> {
        //FIXME: not fix width.
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;

        let n = Node::<K, V>::new(&mut file, NodeID(1), 3, Vec::<Elm<'d, K, V>>::new())?;

        Ok(BTree {
            root: n,
            count: 1,
            file,
        })
    }

    fn insert(&mut self, key: K, value: V) -> Result<(), Box<Error>> {
        let elm = Elm::<K, V>::new(key, value, Edge::Empty, Edge::Empty);
        let op_e = self.root.insert(&mut self.file, &mut self.count, elm)?;
        match op_e {
            None => Ok(()),
            Some(e) => {
                let elms = Vec::<Elm<'d, K, V>>::new();
                elms.push(e);

                self.count += 1;

                let n = Node::<K, V>::new(&mut self.file, NodeID(self.count), 3, elms)?;
                self.root = n;

                Ok(())
            },
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
struct NodeID(usize);

impl NodeID {
    fn block_position(&self) -> u64 {
        let NodeID(u) = self;
        (u * BLOCK_SIZE) as u64
    }
}

#[derive(Serialize, Deserialize)]
struct Node<'d, K: KeyType<'d>, V: ValueType<'d>> {
    id: NodeID,
    width: usize,
    elms: Vec<Elm<'d, K, V>>,
    _key_marker: PhantomData<&'d K>,
    _value_marker: PhantomData<&'d V>,
}

impl <'d, K: KeyType<'d>, V: ValueType<'d>> Node<'d, K, V> {
    fn new(file: &mut File, id: NodeID, width: usize, elms: Vec<Elm<'d, K, V>>) -> Result<Node<'d, K, V>, Box<Error>> {
        let n = Node {
            id,
            width,
            elms,
            _key_marker: PhantomData,
            _value_marker: PhantomData,
        };

        Node::save(file, &n)?;

        Ok(n)
    }

    fn insert(&mut self, file: &mut File, count: &mut usize, elm: Elm<'d, K, V>) -> Result<Option<Elm<'d, K, V>>, Box<Error>> {
        let (edge, index) = self.search(&elm);
        match edge {
            Edge::Empty => {
                self.elms.push(elm);
                self.elms.sort();
                return Ok(self.balance(file, count)?);
            },
            Edge::NotEmpty(id) => {
                let mut node = Node::<'d, K, V>::open(file, id)?;

                match node.insert(file, count, elm) {
                    Ok(op_e) => {
                        match op_e {
                            None => Ok(None),
                            Some(e) => {
                                self.elms.insert(index, e);

                                if let Some(left_e) = self.elms.iter_mut().nth(index-1) {
                                    left_e.right = self.elms[index].left;
                                }

                                if let Some(right_e) = self.elms.iter_mut().nth(index+1) {
                                    right_e.left = self.elms[index].right;
                                }

                                return Ok(self.balance(file, count)?);
                            },
                        }
                    }
                }
            },
        }
    }

    fn balance(&mut self, file: &mut File, count: &mut usize) -> Result<Option<Elm<'d, K, V>>, Box<Error>>{
        if self.elms.len() == self.width {
            let m = self.elms.len() / 2;
            let mut elm = self.elms.remove(m);

            let left_elms = self.elms.split_off(m);
            *count += 1;
            let mut left = Node::<'d, K, V>::new(file, NodeID(*count), self.width, left_elms)?;

            elm.left = Edge::NotEmpty(left.id);
            elm.right = Edge::NotEmpty(self.id);

            return Ok(Some(elm));
        }

        Ok(None)
    }

    fn save(file: &mut File, node: &Node<'d, K, V>) -> Result<(), Box<Error>> {
        let mut buff = serialize(&node)?;

        if buff.len() > BLOCK_SIZE {
            return Err(From::from("invalid node size"));
        } else {
            let diff = BLOCK_SIZE - buff.len();
            buff.extend(vec![0; diff]);
        }

        file.seek(SeekFrom::Start(node.id.block_position()));
        file.write_all(&buff)?;

        Ok(())
    }

    fn open(file: &mut File, id: NodeID) -> Result<Node<'d, K, V>, Box<Error>> {
        let mut buff = vec![0; BLOCK_SIZE];
        file.seek(SeekFrom::Start(id.block_position()));
        file.read_exact(&mut buff)?;
        match deserialize(&buff) {
            Ok(v) => Ok(v),
            Err(e) => Err(From::from(e)),
        }
    }

    fn search(&self, elm: &Elm<'d, K, V>) -> (Edge, usize) {
        for i in 0..self.elms.len() {
            let e = &self.elms[i];
            if elm < e {
                return (e.left, i);
            }
        }

        let Some(e) = self.elms.last();
        (e.right, self.elms.len() - 1)
    }
}

#[derive(Serialize, Deserialize, Copy, Clone)]
enum Edge {
    Empty,
    NotEmpty(NodeID),
}

#[derive(Serialize, Deserialize, Clone)]
struct Elm<'d, K: KeyType<'d>, V: ValueType<'d>> {
    key: &'d K,
    value: &'d V,
    right: Edge,
    left: Edge,
}

impl <'d, K: KeyType<'d>, V: ValueType<'d>> Elm<'d, K, V> {
    fn new(key: K, value: V, right: Edge, left: Edge) -> Elm<'d, K, V> {
        Elm {
            key,
            value,
            right,
            left,
        }
    }
}

impl <'d, K: KeyType<'d>, V: ValueType<'d>> Ord for Elm<'d, K, V> {
    fn cmp(&self, other: &Elm<'d, K, V>) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl <'d, K: KeyType<'d>, V: ValueType<'d>> PartialOrd for Elm<'d, K, V> {
    fn partial_cmp(&self, other: &Elm<'d, K, V>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl <'d, K: KeyType<'d>, V: ValueType<'d>> PartialEq for Elm<'d, K, V> {
    fn eq(&self, other: &Elm<'d, K, V>) -> bool {
        self.key == other.key
    }
}

impl <'d, K: KeyType<'d>, V: ValueType<'d>> Eq for Elm<'d, K, V> {}

pub struct DiskBtree<'d, K: KeyType<'d>, V: ValueType<'d>> {
    file: File,
    key_size: usize,
    value_size: usize,
    _key_marker: PhantomData<&'d K>,
    _value_marker: PhantomData<&'d V>,
}

#[derive(Serialize, Deserialize)]
struct MetaData {
    name: String,
    version: u8,
}

impl<'d, K: KeyType<'d>, V: ValueType<'d>> DiskBtree<'d, K, V> {
    pub fn new(
        file_path: &String,
        key_size: usize,
        value_size: usize,
    ) -> Result<DiskBtree<'d, K, V>, Box<Error>> {
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

        let meta= b.metadata()?;
        println!("db name: {}", meta.name);

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

    fn metadata(&mut self) -> Result<MetaData, Box<Error>> {
        let mut buff = vec![0; BLOCK_SIZE];

        self.file.seek(SeekFrom::Start(0));
        match self.file.read_exact(&mut buff) {
            Ok(_) => {
                match deserialize(&buff) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e),
                }
            },
            Err(e) => {
                Err(Box::new(e))
            }
        }
    }

    fn mapping(&self) -> Result<(), Box<Error>> {
        Ok(())
    }
}
