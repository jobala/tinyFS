use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
};

use crate::tiny::{
    bitmap::Bitmap,
    superblock::{Superblock, BLOCK_SIZE},
};

pub fn make(path: &str) {
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .expect("file to have been opened");

    let mut buf = BufWriter::new(&file);

    let mut super_block = Superblock::new(BLOCK_SIZE, 64);
    super_block.serialize_into(&mut buf);

    let mut bitmap = Bitmap::new();
    bitmap.serialize_into(&mut buf);

    buf.flush().expect("buffer to have been flushed");
    file.set_len(64 * BLOCK_SIZE as u64)
        .expect("to have set file size");
}
