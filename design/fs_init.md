## Filesystem Initialization 

Filesystem initialisation happens whenever the file system is mounted. 

## Goals

- If it is the first time the filesystem is mounted
    - create root directory
- if the filesystem had been mounted before
    - update the last_mounted_at property


## Non Goals

- Adding default files in the default directory
- Using A BTree to store directory entries

## Design

## Implementation

```rust
struct Directory {
    entries HashMap<string, i32>
}
```

```
1. Check if the first inode bitmap has been allocated
2. If it has, that means that a root directory exists.
    1. Just update the last_mounted_at property
3. If there's no root directory
    1. Create a directory inode entry
    2. Mark the inode's bitmap as allocated
    3. Save the directory
```

