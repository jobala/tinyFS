use std::{
    collections::HashMap,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Directory {
    entries: HashMap<String, i32>,
}

impl Directory {
    fn serialize_into<W: Write>(&mut self, buf: W) -> Result<(), bincode::Error> {
        bincode::serialize_into(buf, self)
    }

    fn deserialize_from<R: Read>(&mut self, buf: R) -> Result<Directory, bincode::Error> {
        let directory: Self = bincode::deserialize_from(buf)?;
        Ok(directory)
    }
}
