use fuse::{FileType, Filesystem};
use std::{
    ffi::c_int,
    io::{BufWriter, Write},
};

use super::{
    bitmap::Bitmap,
    constants::{Disk, DIR},
    directory::DirData,
    inode::Inode,
};

impl Filesystem for TinyFS {
    fn init(&mut self, _req: &fuse::Request) -> Result<(), c_int> {
        let root_inode = 1;

        let mut bm = Bitmap::from(&self.disk);
        if bm.is_inode_allocated(root_inode) {
            return Ok(());
        }

        let mut inode = Inode::new();
        let mut inode_data = DirData::default();

        let data_buf = Vec::new();
        let mut write_buf = BufWriter::new(data_buf);
        let _ = inode_data.serialize_into(&mut write_buf);
        let _ = write_buf.flush();
        let data_buf = write_buf.into_inner().expect("error getting inner buffer");
        inode.size = data_buf.len();

        let (block_ptrs, block_count) = self.save_data_blocks(&mut bm, data_buf);
        inode.block_pointers = block_ptrs;
        inode.block_count = block_count as u64;
        inode.id = root_inode as u64;
        inode.kind = DIR;
        inode
            .save_at(root_inode as u64, &self.disk)
            .expect("error saving root directory inode");

        bm.allocate_inode(root_inode);
        bm.save_to(&self.disk).expect("error saving root directory");
        Ok(())
    }

    fn lookup(&mut self, _req: &fuse::Request, parent: u64, _name: &std::ffi::OsStr, reply: fuse::ReplyEntry) {
        if parent == 1 {
            let mut inode = Inode::load_from(&self.disk, parent).expect("error loading inode");
            reply.entry(&self.ttl(), &inode.to_file_attr(), 0);
        } else {
            reply.error(libc::ENOENT);
        }
    }

    fn getattr(&mut self, _req: &fuse::Request, ino: u64, reply: fuse::ReplyAttr) {
        let mut inode = Inode::load_from(&self.disk, ino).expect("error loading inode");
        reply.attr(&self.ttl(), &inode.to_file_attr());
    }

    fn readdir(&mut self, _req: &fuse::Request, ino: u64, _fh: u64, offset: i64, mut reply: fuse::ReplyDirectory) {
        let inode = Inode::load_from(&self.disk, ino).expect("error loading inode");
        let dir_data = self.get_dir_data(inode);

        for (i, (name, entry)) in dir_data.entries.iter().enumerate().skip(offset as usize) {
            let mut kind = FileType::RegularFile;
            if entry.kind == 1 {
                kind = FileType::Directory;
            }

            reply.add(entry.ino, i as i64 + 1, kind, name);
        }

        reply.ok();
    }
}

pub struct TinyFS {
    pub disk: Disk,
}

// Look up inode based on name and parent
