use std::io::{Read, Seek, SeekFrom, Write};

use bitvec::prelude::*;

use super::superblock::BLOCK_SIZE;

pub struct Bitmap {
    inode: BitVec<u8>,
    data: BitVec<u8>,
}

impl Bitmap {
    pub fn new() -> Self {
        Bitmap {
            inode: bitvec![u8, Lsb0; 0; (BLOCK_SIZE * 8) as usize],
            data: bitvec![u8, Lsb0; 0; (BLOCK_SIZE * 8) as usize],
        }
    }

    pub fn serialize_into<W: Write + Seek>(&mut self, mut buf: W) {
        let offset = BLOCK_SIZE;

        buf.seek(SeekFrom::Start(offset)).unwrap();
        buf.write_all(self.inode.as_raw_slice()).unwrap();
        buf.write_all(self.data.as_raw_slice()).unwrap();
    }

    pub fn deserialize_from<R: Read + Seek>(&mut self, mut r: R) {
        let offset = BLOCK_SIZE;
        let mut buf = Vec::with_capacity(BLOCK_SIZE as usize);

        r.seek(SeekFrom::Start(offset)).unwrap();
        r.read_exact(&mut buf).unwrap();
        self.inode = BitVec::from_slice(&buf);

        r.read_exact(&mut buf).unwrap();
        self.data = BitVec::from_slice(&buf);
    }

    pub fn allocate_inode(&mut self, index: usize) {
        self.inode.set(index, true)
    }

    pub fn free_inode(&mut self, index: usize) {
        self.inode.set(index, false);
    }

    pub fn allocate_data_block(&mut self, index: usize) {
        self.data.set(index, true);
    }

    pub fn free_data_block(&mut self, index: usize) {
        self.inode.set(index, false);
    }
}
