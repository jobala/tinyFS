mod mkfs;
mod tiny;

use fuse::Filesystem;

fn main() {
    mkfs::make("./tiny.img");

    let mount_path = "/tmp/tiny";
    fuse::mount(TinyFS, &mount_path, &[]).expect("expected filesytem to mount");
}

struct TinyFS;

impl Filesystem for TinyFS {}
