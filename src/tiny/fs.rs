use fuse::Filesystem;
use std::{
    ffi::c_int,
    fs::OpenOptions,
    io::{BufReader, BufWriter},
};

use super::bitmap::Bitmap;

pub struct TinyFS;

impl Filesystem for TinyFS {
    fn init(&mut self, _req: &fuse::Request) -> Result<(), c_int> {
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .open("./tiny.img")
            .expect("file to have been opened");

        let mut buf = BufReader::new(&file);
        let mut rbuf = BufWriter::new(&file);

        let mut bm = Bitmap::deserialize_from(&mut buf).unwrap();
        bm.allocate_inode(0);
        bm.serialize_into(&mut rbuf).unwrap();

        // load bitmap
        // check if the first inode bitmap is allocated
        // If not, create a directory entry
        // Assign blocks of data for the entry
        Ok(())
    }
}
