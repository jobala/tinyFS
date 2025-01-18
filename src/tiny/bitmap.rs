use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write};

use bitvec::prelude::*;

use super::constants::{Disk, BLOCK_SIZE};

impl Bitmap {
    pub fn new() -> Self {
        Bitmap {
            inode: bitvec![u8, Lsb0; 0; BLOCK_SIZE * 8],
            data: bitvec![u8, Lsb0; 0; BLOCK_SIZE * 8 ],
        }
    }

    pub fn from(disk: &Disk) -> Bitmap {
        let buf = BufReader::new(disk);
        Self::deserialize_from(buf).expect("failed to load bitmap")
    }

    pub fn save_to(&mut self, disk: &Disk) -> Result<(), io::Error> {
        let buf = BufWriter::new(disk);
        self.serialize_into(buf)
    }

    pub fn allocate_inode(&mut self, index: usize) {
        self.inode.set(index, true)
    }

    //pub fn free_inode(&mut self, index: usize) {
    //    self.inode.set(index, false);
    //}
    //
    pub fn allocate_data_block(&mut self, index: usize) {
        self.data.set(index, true);
    }

    // TODO: use correct error handling
    // https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/define_error_type.html
    pub fn find_free_data_block(&mut self) -> usize {
        for i in 0..self.data.len() {
            if self.data[i] == false {
                return i;
            }
        }

        return 0;
    }
    //
    //pub fn free_data_block(&mut self, index: usize) {
    //    self.inode.set(index, false);
    //}
    //

    pub fn is_inode_allocated(&mut self, index: usize) -> bool {
        self.inode[index]
    }

    fn serialize_into<W: Write + Seek>(&mut self, mut buf: W) -> Result<(), io::Error> {
        let offset = BLOCK_SIZE as u64;

        buf.seek(SeekFrom::Start(offset))?;
        buf.write_all(self.inode.as_raw_slice())?;
        buf.write_all(self.data.as_raw_slice())?;
        buf.flush()?;
        Ok(())
    }

    fn deserialize_from<R: Read + Seek>(mut r: R) -> Result<Bitmap, io::Error> {
        let offset = BLOCK_SIZE as u64;
        let mut bitmap = Bitmap::new();

        let mut buf = [0u8; BLOCK_SIZE];
        r.seek(SeekFrom::Start(offset))?;
        r.read_exact(&mut buf)?;
        bitmap.inode = BitVec::from_slice(&buf);

        r.read_exact(&mut buf)?;
        bitmap.data = BitVec::from_slice(&buf);

        Ok(bitmap)
    }
}

pub struct Bitmap {
    inode: BitVec<u8>,
    data: BitVec<u8>,
}
