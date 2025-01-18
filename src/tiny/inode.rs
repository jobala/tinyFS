use std::{
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    time::SystemTime,
};

use super::{
    constants::{Disk, BLOCK_SIZE, INODE_BLOCK_BASE},
    type_extensions::TinyTimespec,
};
use fuse::{FileAttr, FileType};
use serde::{Deserialize, Serialize};

impl Inode {
    pub fn save_at(&mut self, ino: u64, disk: &Disk) -> Result<(), bincode::Error> {
        let location = Self::get_location(ino);
        let mut buf = BufWriter::new(disk);

        let _ = buf.seek(SeekFrom::Start(location));
        self.serialize_into(&mut buf)?;
        let _ = buf.flush();
        Ok(())
    }

    pub fn load_from(disk: &Disk, ino: u64) -> Result<Inode, bincode::Error> {
        let location = Self::get_location(ino);
        let mut buf = BufReader::new(disk);
        let _ = buf.seek(SeekFrom::Start(location));

        let mut read_buf = [0; size_of::<Inode>()];
        buf.read_exact(&mut read_buf);
        bincode::deserialize_from(&mut read_buf.as_slice())
    }

    pub fn to_file_attr(&mut self) -> FileAttr {
        let mut kind = FileType::RegularFile;
        if self.kind == 1 {
            kind = FileType::Directory;
        }

        let now = SystemTime::now();

        FileAttr {
            ino: self.id,
            size: self.block_count * BLOCK_SIZE as u64,
            blocks: self.block_count,
            atime: now.to_timespec(),
            mtime: now.to_timespec(),
            crtime: now.to_timespec(),
            ctime: now.to_timespec(),
            kind,
            perm: 0o755,
            nlink: self.hard_links,
            uid: 1000,
            gid: 1000,
            rdev: 0,
            flags: 0,
        }
    }

    fn serialize_into<W: Write>(&mut self, buf: W) -> Result<(), bincode::Error> {
        bincode::serialize_into(buf, self)
    }

    fn get_location(ino: u64) -> u64 {
        INODE_BLOCK_BASE + (ino * size_of::<Inode>() as u64)
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Inode {
    pub id: u64,
    pub kind: u8,
    pub block_count: u64,
    pub accessed_at: u64,
    pub modified_at: u64,
    pub created_at: u64,
    pub hard_links: u32,
    pub block_pointers: [u64; 12],
}
