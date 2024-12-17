mod mkfs;
mod tiny;

use tiny::TinyFS;

fn main() {
    mkfs::make("./tiny.img");

    let mount_path = "/tmp/tiny";

    fuse::mount(TinyFS, &mount_path, &[]).expect("expected filesytem to mount");
}
