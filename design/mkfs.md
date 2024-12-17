## mkfs

TinyFS uses a file as a disk, `mkfs` formats this file and lays out the different parts like the superblock,
bitmap so and so forth.

`mkfs` serializes the **superblock** and **bitmap** into a binary format and then writes them to disk.
The inode table and blocks don't need to be explicitly configured because we can compute the address
of an inode or datablock from their bitmap indices and their section's starting address.

For example, if we want to read from inode 45:
    1. Find the starting address of inodes table 
    2. location = inode_table_start + (45 * sizeof(inode))
    3. read(location, sizeof(inode))
The same formular applies for the calculation of a data block's location

### Superblock

The superblock holds metadata about the filesystem and knows how to serialize and deserialize itself into a binary format
which `mkfs` then writes to disk.

### Bitmaps

Bitmaps are used to determine whether an inode or block is free or allocated. TinyFS has two bitmap tables; inode and 
block bitmaps. However, for TinyFS we have more bits than we can use because each bitmap table uses a block (each block
is 4096 bytes) giving us 4096 * 8 -- 32k-- bits. In reality, TinyFS needs around 80 bits for inodes.

Bit 0 in the bitmap corresponds to inode 0, Bit 1 to inode 1 so and so forth. Bitmap also knows how to serialize and 
deserialize itself to a binary format which `mkfs` uses to write to disk.
