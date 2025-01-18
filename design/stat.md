## stat 

`stat` is a linux command for showing file information and its sample output is shown below.

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

- Support `stat` for root directory

## Non Goals

- Supporting `lookup`for non-root directories

## Design

For tinyFS to support `stat` it needs to implement two methods `getattr` & `lookup` their signatures are shown below.

```rust
 lookup(&mut self, _req: &fuse::Request, _parent: u64, _name: &std::ffi::OsStr, reply: fuse::ReplyEntry);

 getattr(&mut self, _req: &fuse::Request, ino: u64, reply: fuse::ReplyAttr);
```

### Lookup

`lookup` finds an inode given the parent's inode number and the name of the file/directory. 

#### Steps

1. Check if parent is 1
    1. If it is, 
        - load the root inode from disk
        - convert the inode to `FileAttr`
        - send a FUSE reply with the `FileAttr` object
    2. If not,
        - send a FUSE error reply with `libc::ENOENT`
    

### getattr

`getattr` has `ino` as one of its params, it holds the inode number. `lookup` provides `getattr` the inode number which
getattr uses to load an inode from disk.

#### Steps

1. Read inode from disk
2. Convert `Inode` to a `FileAttr` object
3. Send a FUSE reply with the  `FileAttr` object 
``
