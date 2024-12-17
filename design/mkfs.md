## mkfs

TinyFS uses a file as a disk, `mkfs` formats this file and lays out the different parts like the superblock,
bitmap so and so forth.

## Goals
    - Format disk with Superblock
    - Format disk with Bitmap table

## Non Goals
    - mkfs doesn't intend to format disk with an inode table
    - mkfs doesn't intend to format disk with a data blocks table

Formatting the disk requires to first have the data structure in memory, for example TinyFS' data blocks table
is 56 blocks wide -- that is 56 * 4kb in size -- which will be wasteful to load into memory then write to disk.
So how do we know a blocks location then? Simple, we can calculate it based on the bitmap index and the data
block's table starting address.

For example, the location of block 35 is;

block_loc = data_table_start + (35 * sizeof(block))

We can then read and write to that location without loading the whole datablock table -- about 224mb -- into memory.

## Design

**mkfs** takes some path which is where it will create its disk.

### Superblock

The superblock holds metadata about the filesystem and knows how to serialize and deserialize itself into a binary format
which `mkfs` then writes to disk.

### Bitmaps

Bitmaps are used to determine whether an inode or block is free or allocated. TinyFS has two bitmap tables; inode and 
block bitmaps. However, for TinyFS we have more bits than we can use because each bitmap table uses a block (each block
is 4096 bytes) giving us 4096 * 8 -- 32k-- bits. In reality, TinyFS needs around 80 bits for inodes.

Bit 0 in the bitmap corresponds to inode 0, Bit 1 to inode 1 so and so forth. Bitmap also knows how to serialize and 
deserialize itself to a binary format which `mkfs` uses to write to disk.

## Implementation

```
class Superblock:
    function serialize_into()
    function deserialize_from()

class Bitmap:
    function serialize_into()
    function deserialize_from()

function mkfs(path: string)
    file = open(path, create)

    sb = Superblock()
    sb.serialize_into(file)

    bm = Bitmap()
    bm.serialize_into(file)

    file.flush()
```
