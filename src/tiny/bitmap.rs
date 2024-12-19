use std::io::{self, Read, Seek, SeekFrom, Write};

use bitvec::prelude::*;

use super::superblock::BLOCK_SIZE;

pub struct Bitmap {
    inode: BitVec<u8>,
    data: BitVec<u8>,
}

impl Bitmap {
    pub fn new() -> Self {
        Bitmap {
            inode: bitvec![u8, Lsb0; 0; BLOCK_SIZE * 8],
            data: bitvec![u8, Lsb0; 0; BLOCK_SIZE * 8 ],
        }
    }

    pub fn serialize_into<W: Write + Seek>(&mut self, mut buf: W) -> Result<(), io::Error> {
        let offset = BLOCK_SIZE as u64;

        buf.seek(SeekFrom::Start(offset))?;
        buf.write_all(self.inode.as_raw_slice())?;
        buf.write_all(self.data.as_raw_slice())?;
        Ok(())
    }

    pub fn deserialize_from<R: Read + Seek>(&mut self, mut r: R) -> Result<(), io::Error> {
        let offset = BLOCK_SIZE as u64;

        let mut buf = Vec::with_capacity(BLOCK_SIZE);
        r.seek(SeekFrom::Start(offset))?;
        r.read_exact(&mut buf)?;
        self.inode = BitVec::from_slice(&buf);

        r.read_exact(&mut buf)?;
        self.data = BitVec::from_slice(&buf);

        Ok(())
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
