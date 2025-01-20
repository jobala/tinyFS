use std::{
    collections::HashMap,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};

impl DirData {
    pub fn serialize_into<W: Write>(&mut self, buf: W) -> Result<(), bincode::Error> {
        bincode::serialize_into(buf, self)
    }

    pub fn deserialize_from<R: Read>(buf: R) -> Result<DirData, bincode::Error> {
        let directory: Self = bincode::deserialize_from(buf)?;
        Ok(directory)
    }

    pub fn insert(&mut self, path: &str, entry: DirEntry) {
        self.entries.insert(String::from(path), entry);
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct DirData {
    pub entries: HashMap<String, DirEntry>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct DirEntry {
    pub ino: u64,
    pub name: String,
    pub kind: u8,
}
