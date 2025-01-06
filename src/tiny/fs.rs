use fuse::Filesystem;
use std::{ffi::c_int, fs::OpenOptions};

use super::{bitmap::Bitmap, inode::Inode};

pub struct TinyFS;

// TODO: Test that this actually works
impl Filesystem for TinyFS {
    fn init(&mut self, _req: &fuse::Request) -> Result<(), c_int> {
        let root_dir_inode = 0;
        let disk = OpenOptions::new()
            .write(true)
            .read(true)
            .open("./tiny.img")
            .expect("file to have been opened");

        let mut bm = Bitmap::from(&disk);
        if bm.is_inode_allocated(root_dir_inode) {
            return Ok(());
        }

        // TODO:
        // Save directory entries in data blocks
        let mut inode = Inode::default();
        inode.id = 0;
        inode.file_type = 1;
        inode
            .save_at(0, &disk)
            .expect("inode was saved successfully");

        Ok(())
    }
}
