mod mkfs;
mod tiny;

use std::path::Path;

use tiny::TinyFS;

fn main() {
    let path = Path::new("./tiny.img");
    if !path.exists() {
        mkfs::make("./tiny.img");
    }

    let mount_path = "/tmp/tiny";

    fuse::mount(TinyFS, &mount_path, &[]).expect("expected filesytem to mount");
}
