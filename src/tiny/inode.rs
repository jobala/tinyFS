use std::io::{BufWriter, Seek, SeekFrom, Write};

use super::constants::{Disk, INODE_BLOCK_BASE};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Inode {
    pub id: u32,
    pub size: u64,
    pub mode: u16,
    pub hard_links: u16,
    pub file_type: u8,
    pub block_pointers: [u32; 12],
}

impl Inode {
    pub fn save_at(&mut self, index: u64, disk: &Disk) -> Result<(), bincode::Error> {
        let location = INODE_BLOCK_BASE + (index * size_of::<Inode>() as u64);
        let mut buf = BufWriter::new(disk);

        let _ = buf.seek(SeekFrom::Start(location));
        self.serialize_into(&mut buf)?;
        let _ = buf.flush();
        Ok(())
    }

    fn serialize_into<W: Write>(&mut self, buf: W) -> Result<(), bincode::Error> {
        bincode::serialize_into(buf, self)
    }
}
