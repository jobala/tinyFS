use std::fs::File;

pub const BLOCK_SIZE: usize = 4096;
pub const INODE_BLOCK_COUNT: u64 = 5u64;
pub const INODE_BLOCK_BASE: u64 = 3u64 * BLOCK_SIZE as u64;
pub const DATA_BLOCK_BASE: u64 = INODE_BLOCK_BASE + (INODE_BLOCK_COUNT * BLOCK_SIZE as u64);

pub const FILE: u8 = 0;
pub const DIR: u8 = 1;

pub type Disk = File;
