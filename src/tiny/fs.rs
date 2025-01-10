use fuse::Filesystem;
use std::{
    ffi::c_int,
    fs::OpenOptions,
    io::{BufWriter, Write},
};

use super::{bitmap::Bitmap, constants::Disk, directory::Directory, inode::Inode};

pub struct TinyFS {
    pub disk: Disk,
}

impl Filesystem for TinyFS {
    fn init(&mut self, _req: &fuse::Request) -> Result<(), c_int> {
        let root_dir_inode = 0;
        let disk = OpenOptions::new()
            .write(true)
            .read(true)
            .open("./tiny.img")
            .expect("file to have been opened");
        self.disk = disk;

        let mut bm = Bitmap::from(&self.disk);
        if bm.is_inode_allocated(root_dir_inode) {
            return Ok(());
        }

        let mut inode = Inode::default();
        let mut root_dir = Directory::default();

        let root_buf = Vec::new();
        let mut buf = BufWriter::new(root_buf);
        let _ = root_dir.serialize_into(&mut buf);
        let _ = buf.flush(); // make sure content is written to the underlying writer.

        let inner_buf = buf.into_inner().expect("error getting inner buffer");
        inode.block_pointers = self.save_data_blocks(&mut bm, inner_buf);
        inode.id = 0;
        inode.file_type = 1;
        inode
            .save_at(0, &self.disk)
            .expect("error saving root directory inode");

        bm.allocate_inode(0);
        bm.save_to(&self.disk).expect("error saving root directory");
        Ok(())
    }
}
