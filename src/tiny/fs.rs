use fuse::Filesystem;
use std::{
    ffi::c_int,
    io::{BufWriter, Write},
};

use super::{bitmap::Bitmap, constants::Disk, directory::Directory, inode::Inode};

pub struct TinyFS {
    pub disk: Disk,
}

impl Filesystem for TinyFS {
    fn init(&mut self, _req: &fuse::Request) -> Result<(), c_int> {
        let root_dir_inode = 0;

        let mut bm = Bitmap::from(&self.disk);
        if bm.is_inode_allocated(root_dir_inode) {
            return Ok(());
        }

        let mut inode = Inode::default();
        let mut inode_data = Directory::default();

        let data_buf = Vec::new();
        let mut write_buf = BufWriter::new(data_buf);
        let _ = inode_data.serialize_into(&mut write_buf);
        let _ = write_buf.flush();
        let data_buf = write_buf.into_inner().expect("error getting inner buffer");

        inode.block_pointers = self.save_data_blocks(&mut bm, data_buf);
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
