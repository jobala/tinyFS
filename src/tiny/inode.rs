use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Inode {
    pub id: u32,
    pub size: u64,
    pub mode: u16,
    pub block_pointers: [u32; 12],
    pub indirect_pointer: u32,
    pub double_indirect_pointer: u32,
}

impl Inode {}
