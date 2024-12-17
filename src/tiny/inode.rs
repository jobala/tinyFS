use super::superblock::BLOCK_SIZE;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Seek, SeekFrom, Write},
    vec,
};

const INODES_PER_BLOCK: usize = 16;

#[derive(Serialize, Deserialize)]
pub struct InodeTable {
    blocks: Vec<[Inode; INODES_PER_BLOCK]>,
    block_count: usize,
}

impl InodeTable {
    pub fn new(block_count: usize) -> InodeTable {
        Self {
            blocks: vec![[Inode::default(); INODES_PER_BLOCK]; block_count],
            block_count,
        }
    }

    pub fn serialize_into<W: Write + Seek>(&mut self, mut w: W) {
        let offset = BLOCK_SIZE * 3;
        let data = bincode::serialize(self).unwrap();

        w.seek(SeekFrom::Start(offset)).unwrap();
        w.write(&data).unwrap();
    }

    pub fn deserialize_from<R: Read + Seek>(&mut self, mut r: R) -> Self {
        let offset = BLOCK_SIZE * 3;
        r.seek(SeekFrom::Start(offset)).unwrap();

        let mut buf: Vec<u8> = Vec::with_capacity((BLOCK_SIZE as usize) * self.block_count);
        r.read_exact(&mut buf).unwrap();

        let inode_table: Self = bincode::deserialize(&buf).unwrap();
        inode_table
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Inode {
    pub id: u32,
    pub size: u64,
    pub mode: u16,
    pub block_pointers: [u32; 12],
    pub indirect_pointer: u32,
    pub double_indirect_pointer: u32,
}

impl Inode {}
