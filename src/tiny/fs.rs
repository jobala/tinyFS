use fuse::Filesystem;
use std::{
    ffi::c_int,
    fs::OpenOptions,
    io::{BufWriter, Cursor, Read, Seek, SeekFrom, Write},
};

use crate::tiny::constants::DATA_BLOCK_BASE;

use super::{bitmap::Bitmap, constants::BLOCK_SIZE, directory::Directory, inode::Inode};

pub struct TinyFS;

// TODO:  throw error for sizes greater than 48KB
// TODO: clean up
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

        let mut inode = Inode::default();
        let mut root_dir = Directory::default();

        let root_buf = Vec::new();
        let mut buf = BufWriter::new(root_buf);
        let _ = root_dir.serialize_into(&mut buf);
        let _ = buf.flush(); // make sure content is written to the underlying writer.

        let inner_buf = buf.into_inner().expect("error getting inner buffer");

        let mut cursor = Cursor::new(inner_buf);
        let mut chunk = [0u8; BLOCK_SIZE];
        while let Ok(n) = cursor.read(&mut chunk) {
            if n == 0 {
                println!("nothing more to read");
                break;
            }

            let mut block_buf = BufWriter::new(&disk);
            let index = bm.find_free_data_block();
            let offset = DATA_BLOCK_BASE + (index * BLOCK_SIZE) as u64;
            let _ = block_buf.seek(SeekFrom::Start(offset));
            let _ = block_buf.write_all(&chunk);
            let _ = block_buf.flush();

            for i in 0..inode.block_pointers.len() {
                if inode.block_pointers[i] == 0 {
                    inode.block_pointers[0] = offset;
                    break;
                }
            }

            bm.allocate_data_block(index);
        }

        inode.id = 0;
        inode.file_type = 1;
        inode
            .save_at(0, &disk)
            .expect("error saving root directory inode");

        bm.allocate_inode(0);
        bm.save_to(&disk).expect("error saving root directory");
        Ok(())
    }
}
