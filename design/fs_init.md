Intention
I want to inform the reader about what happens during filesystem initialisation

## Filesystem Initialization

Filesystem initialisation happens whenever the file system is mounted.

## Goals

- Create the filesystem's root directory the first time the filesystem is mounted

## Non Goals

- Adding default files in the default directory
- Using A BTree to store directory entries

## Design

The root directory should be created only if it wasn't created before. We will use the inode bitmap table to track what
inode blocks have been used/allocated and since the root directory's inode is the first to be created we should
check whether the inode bitmap at index zero is allocated. If it is, return.

```rust
if bitmap.is_inode_allocated(0) {
    return
}

...
```

Both files and directories are represented by an Inode, for directories, the Inode's **file_type** attribute is
set to directory.

```rust
let inode = Inode::new()
inode.file_type = 1 // 1 == directory
```

Directories have entries which could be files or other directories, we'll use a hashmap that maps a path to some
file or directory inode number.

```rust
struct DirData {
    entries HashMap<string, i32>
}
```

Each inode tracks where in disk its data is written. The method `allocate_blocks` returns the write location
addresses, which are then stored in the inode's **block_ptrs** property.

```rust
let dir_data = DirData::default()
inode.block_ptrs = self.allocate_blocks(dir_data)
```

Finally, we need to save the inode to disk at the first inode block(zero indexed) and make sure the inode bitmap
at index zero is marked as allocated.

```rust
inode.save_at(0, &disk)
bitmap.allocate_inode(0)
```
