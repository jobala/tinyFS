use std::{
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use super::{
    constants::{Disk, BLOCK_SIZE, INODE_BLOCK_BASE},
    type_extensions::TinyTimespec,
};
use fuse::{FileAttr, FileType};
use serde::{Deserialize, Serialize};
use time::Timespec;

impl Inode {
    pub fn new() -> Inode {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("error creating time");

        Inode {
            id: 0,
            kind: 0,
            block_count: 0,
            accessed_at: now.as_millis() as u64,
            modified_at: now.as_millis() as u64,
            created_at: now.as_millis() as u64,
            hard_links: 0,
            block_pointers: [0; 12],
        }
    }

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
        let _ = buf.read_exact(&mut read_buf);
        bincode::deserialize_from(&mut read_buf.as_slice())
    }

    pub fn to_file_attr(&mut self) -> FileAttr {
        let mut kind = FileType::RegularFile;
        if self.kind == 1 {
            kind = FileType::Directory;
        }

        FileAttr {
            ino: self.id,
            size: self.block_count * BLOCK_SIZE as u64,
            blocks: self.block_count,
            atime: self.to_time(self.accessed_at),
            mtime: self.to_time(self.modified_at),
            crtime: self.to_time(self.created_at),
            ctime: self.to_time(self.created_at),
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

    fn to_time(&mut self, millis: u64) -> Timespec {
        let sys_time = UNIX_EPOCH + Duration::from_millis(millis);
        sys_time.to_timespec()
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
