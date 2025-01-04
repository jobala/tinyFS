use fuse::Filesystem;
use std::ffi::c_int;

pub struct TinyFS;

impl Filesystem for TinyFS {
    fn init(&mut self, _req: &fuse::Request) -> Result<(), c_int> {
        Ok(())
    }
}
