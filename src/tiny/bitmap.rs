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
        buf.flush()?;
        Ok(())
    }

    pub fn deserialize_from<R: Read + Seek>(mut r: R) -> Result<Bitmap, io::Error> {
        let offset = BLOCK_SIZE as u64;
        let mut bitmap = Bitmap::new();

        let mut buf = Vec::with_capacity(BLOCK_SIZE);
        r.seek(SeekFrom::Start(offset))?;
        r.read_exact(&mut buf)?;
        bitmap.inode = BitVec::from_slice(&buf);

        r.read_exact(&mut buf)?;
        bitmap.data = BitVec::from_slice(&buf);

        Ok(bitmap)
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

    pub fn is_inode_allocated(&mut self, index: usize) -> bool {
        self.inode[index]
    }
}
