use std::{
    collections::HashMap,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct DirData {
    entries: HashMap<String, i32>,
}

impl DirData {
    pub fn serialize_into<W: Write>(&mut self, buf: W) -> Result<(), bincode::Error> {
        bincode::serialize_into(buf, self)
    }

    pub fn deserialize_from<R: Read>(&mut self, buf: R) -> Result<DirData, bincode::Error> {
        let directory: Self = bincode::deserialize_from(buf)?;
        Ok(directory)
    }

    pub fn insert(&mut self, path: &str, inode_num: i32) {
        self.entries.insert(String::from(path), inode_num);
    }
}
