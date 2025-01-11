mod mkfs;
mod tiny;

use std::{fs::OpenOptions, path::Path};

use tiny::{constants::Disk, TinyFS};

fn main() {
    let path = Path::new("./tiny.img");
    if !path.exists() {
        mkfs::make("./tiny.img");
    }

    let mount_path = "/tmp/tiny";

    fuse::mount(
        TinyFS {
            disk: load_disk(path),
        },
        &mount_path,
        &[],
    )
    .expect("expected filesytem to mount");
}

fn load_disk(path: &Path) -> Disk {
    OpenOptions::new()
        .write(true)
        .read(true)
        .open(path)
        .expect("file to have been opened")
}
