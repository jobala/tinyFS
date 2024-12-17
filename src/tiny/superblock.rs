use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

pub const BLOCK_SIZE: u64 = 4096;

#[derive(Deserialize, Serialize, Debug)]
pub struct Superblock {
    block_size: u64,
    block_count: u32,
}

impl Superblock {
    pub fn new(block_size: u64, block_count: u32) -> Superblock {
        Self {
            block_size,
            block_count,
        }
    }

    pub fn serialize_into<W: Write>(&mut self, buf: W) {
        bincode::serialize_into(buf, self).unwrap();
    }

    pub fn deserialize_from<R: Read>(&mut self, buf: R) -> Self {
        let super_block: Self = bincode::deserialize_from(buf).unwrap();
        super_block
    }
}
