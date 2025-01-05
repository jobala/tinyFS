use std::io::{Seek, SeekFrom, Write};

use serde::{Deserialize, Serialize};

use super::superblock::BLOCK_SIZE;

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

// TODO: move constants to a constants file
const INODE_BLOCK_BASE: u64 = 3u64 * BLOCK_SIZE as u64;

impl Inode {
    // TODO: this may either generate a bincode or io error, handle both errors correctly
    pub fn save_at<W: Write + Seek>(
        &mut self,
        index: u64,
        mut buf: W,
    ) -> Result<(), bincode::Error> {
        let location = INODE_BLOCK_BASE + (index * size_of::<Inode>() as u64);

        buf.seek(SeekFrom::Start(location))?;
        self.serialize_into(&mut buf)?;
        buf.flush()?;
        Ok(())
    }

    fn serialize_into<W: Write>(&mut self, buf: W) -> Result<(), bincode::Error> {
        bincode::serialize_into(buf, self)
    }
}
