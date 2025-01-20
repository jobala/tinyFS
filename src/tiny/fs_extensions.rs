use std::{
    io::{BufReader, BufWriter, Cursor, Read, Seek, SeekFrom, Write},
    time::SystemTime,
};

use time::Timespec;

use super::{
    bitmap::Bitmap,
    constants::{Block, BLOCK_SIZE, DATA_BLOCK_BASE},
    directory::DirData,
    inode::Inode,
    type_extensions::TinyTimespec,
    TinyFS,
};

impl TinyFS {
    // TODO:  throw error for files greater than 48KB in size
    pub fn save_data_blocks(&mut self, bitmap: &mut Bitmap, buf: Vec<u8>) -> ([u64; 12], usize) {
        let mut block_ptrs = [0u64; 12];
        let mut cursor = Cursor::new(buf);
        let mut chunk = [0u8; BLOCK_SIZE];
        let mut last_allocated = 0;

        while let Ok(n) = cursor.read(&mut chunk) {
            if n == 0 {
                println!("nothing more to read");
                break;
            }

            let mut write_buf = BufWriter::new(&self.disk);
            let index = bitmap.find_free_data_block();
            let block_location = DATA_BLOCK_BASE + (index * BLOCK_SIZE) as u64;
            let _ = write_buf.seek(SeekFrom::Start(block_location));
            let _ = write_buf.write_all(&chunk);
            block_ptrs[last_allocated] = block_location;
            last_allocated += 1;
        }

        (block_ptrs, last_allocated)
    }

    pub fn ttl(&mut self) -> Timespec {
        SystemTime::now().to_timespec()
    }

    pub fn get_dir_data(&mut self, inode: Inode) -> DirData {
        let mut buf = vec![];

        for ptr in inode.block_pointers {
            if ptr == 0 {
                continue;
            }

            let block = self.load_block(ptr);
            buf.append(&mut block.to_vec());
        }

        let s = &buf[..inode.size + 1];
        DirData::deserialize_from(s).expect("error getting dir data")
    }

    fn load_block(&mut self, location: u64) -> Block {
        let mut block = [0; BLOCK_SIZE];

        let mut disk_buf = BufReader::new(&self.disk);
        let _ = disk_buf.seek(SeekFrom::Start(location));
        self.disk.read_exact(&mut block).expect("error reading block");

        block
    }
}
