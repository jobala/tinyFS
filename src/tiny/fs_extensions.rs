use std::io::{BufWriter, Cursor, Read, Seek, SeekFrom, Write};

use super::{
    bitmap::Bitmap,
    constants::{BLOCK_SIZE, DATA_BLOCK_BASE},
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
}
