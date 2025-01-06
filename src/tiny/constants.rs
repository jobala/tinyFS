use std::fs::File;

pub const BLOCK_SIZE: usize = 4096;
pub const INODE_BLOCK_BASE: u64 = 3u64 * BLOCK_SIZE as u64;

pub type Disk = File;
