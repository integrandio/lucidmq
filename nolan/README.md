# Nolan

## What is Nolan?

Nolan is the base library that wraps and handles commitlog/WAL logic into a simplified API. To those who are not familiar, this is an append only data structure that supports random reads via an offset. This library is created the underlying persistence unit behind the LucidMQ project. It is somewhat generic and can be used in other projects that need to utilize a commitlog/WAL.

To prevent commitlog curruption, only a few commitlog methods are exposed to be used as client code.

## Basic Usage

```rust
pub fn main() {
    let commit_log = Commitlog::new("test_dir".to_string(), 1000, 10000);

    // Let's append data to our commitlog
    commit_log.append("hello please".as_bytes());

    //Lets lookup by offset the message we just appended
    commit_log.read(0);
}

```
