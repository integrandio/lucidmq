# Nolan

## What is Nolan?

Nolan is the base library that wraps and handles commitlog/WAL(write ahead log) logic into a simplified API. To those who are not familiar, a commitlog or WAL is an append only data structure that supports random reads via an offset. This library is the underlying persistence unit behind the LucidMQ project. It is somewhat generic and can be used in other projects that need to utilize a commitlog/WAL.

To prevent commitlog curruption, only a few commitlog methods are exposed to be used as client code.

## Terminology

### Commitlog
A directory containing all of the segments that make up the log. It is an append only data structure that supports random reads.

### Segment
A sement is made up of 2 files that contain all of the data stored in our commitlog. These two files share the same name(except for the file extension) so nolan knows that they're the same segment.

### Log
The log file is the file that actually persists the commitlog input to disk.

### Index
The index is a a file that contains metadata about the offsets for fast lookups for each piece of data. This allows for data from the log file to have to be loaded into memory. Instead, entries from the index are loaded from the index files and allow for fast lookups.

### Cleaner
The cleaner is a mechanism for cleaning up data that is no longer required. This requirement is based on the defined policy passed in as a configuration.

## Basic Usage

```rust
pub fn main() {
    // Create a commitlog, by providing the data directory, max size of the segment
    let commit_log = Commitlog::new("test_dir".to_string(), 1000, 10000);

    // Let's append data to our commitlog
    commit_log.append("hello please".as_bytes());

    //Lets lookup by offset the message we just appended
    commit_log.read(0);
}
```


