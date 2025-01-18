## TinyFS: A Little File System

TinyFS is a little filesystem with no big plans.

## Setup

1. Run `mkdir /tmp/tiny`
2. Run `sudo addgroup <username> fuse`

## Running

1. To start **tinyFS** run make run in this project's root dir
2. To interact with tinyFS, open another terminal and perform file operations on `/tmp/tiny` ie `stat /tmp/tiny`

## Resources

- [Filesystem Implementation](https://pages.cs.wisc.edu/~remzi/OSTEP/file-implementation.pdf)
- [To FUSE or Not To Fuse](https://libfuse.github.io/doxygen/fast17-vangoor.pdf)
- [Fuse Filesystems](https://zsiciarz.github.io/24daysofrust/book/vol1/day15.html)

## Design Docs

- [tinyfs intro](./design/tinyfs.md)
- [format disk](./design/mkfs.md)
- [initialize filesystem](./design/fs_init.md)
- [stat](./design/stat.md)
- [list directory contents]() Not Implemented
- [mkdir]() Not Implemented
- [rmdir]() Not Implemented
- [create]() Not Implemented
- [read]() Not Implemented
- [write]() Not Implemented
- [unlink]() Not Implemented
