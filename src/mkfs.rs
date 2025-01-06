use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
};

use crate::tiny::{bitmap::Bitmap, constants::BLOCK_SIZE, superblock::Superblock};

pub fn make(path: &str) {
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .expect("file to have been opened");

    let mut buf = BufWriter::new(&file);

    let mut super_block = Superblock::new(BLOCK_SIZE, 64);
    super_block
        .serialize_into(&mut buf)
        .expect("superblock to have been serialized");

    let mut bitmap = Bitmap::new();
    bitmap.save(&file).expect("bitmap to have been serialized");

    buf.flush().expect("buffer to have been flushed");
    file.set_len(64 * BLOCK_SIZE as u64)
        .expect("to have set file size");
}
