use fuse::Filesystem;
use std::{
    ffi::c_int,
    fs::OpenOptions,
    io::{BufReader, BufWriter},
};

use super::{bitmap::Bitmap, inode::Inode};

pub struct TinyFS;

impl Filesystem for TinyFS {
    // TODO: Test that this actually works
    fn init(&mut self, _req: &fuse::Request) -> Result<(), c_int> {
        let root_dir_inode = 0;
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .open("./tiny.img")
            .expect("file to have been opened");

        let mut buf = BufReader::new(&file);
        let write_buf = BufWriter::new(&file);

        // TODO: encapsulate serialization in buffer
        // Pass file in the buffer constructor
        let mut bm = Bitmap::deserialize_from(&mut buf).unwrap();

        if bm.is_inode_allocated(root_dir_inode) {
            return Ok(());
        }

        // TODO:
        // Save directory entries in data blocks
        let mut inode = Inode::default();
        inode.id = 0;
        inode.file_type = 1;
        inode
            .save_at(0, write_buf)
            .expect("inode was saved successfully");

        Ok(())
    }
}
