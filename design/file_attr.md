## Get File Attributes

File attributes will enable the `stat` command to work on tinyFS, the command returns information about a file/directory.

```
File: .
Size: 4096            Blocks: 8          IO Block: 4096   directory
Device: 179,2   Inode: 1278537     Links: 7
Access: (0775/drwxrwxr-x)  Uid: ( 1000/  jobala)   Gid: ( 1003/  jobala)
Access: 2025-01-14 16:05:26.789054116 +0300
Modify: 2025-01-14 16:05:26.463056875 +0300
Change: 2025-01-14 16:05:26.463056875 +0300
Birth: 2024-12-14 15:22:49.207461152 +0300
```

## Goals

- Return accurate information

## Non Goals

- N/A

## Design

This fs functionality is handled by the `getattr` function, which has the parameters below.

```rust
 getattr(&mut self, _req: &fuse::Request, ino: u64, reply: fuse::ReplyAttr)
```

The `ino` param holds the inode number for the inode which we want to get information about.
The inode number is 1 for the root directory, but in tinyFS' inode bitmap, the root directory has index 0, so we should
make the conversion.

1. Read inode from disk
2. Create a FileAttr object from inode data
3. Reply to the fuse request with inode data

### LookUp
Lookup is responsible for getting the inode number given some path. `getattr` implicitly depends on `lookup` for
the inode number.

```rust
lookup(&mut self, _req: &fuse::Request, _parent: u64, _name: &std::ffi::OsStr, reply: fuse::ReplyEntry)
```
