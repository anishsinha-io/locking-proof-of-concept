### Locking Proof of Concept

This repository contains a rough proof of concept for how I will implement shared, thread-safe in-memory, internal data structures for managing the concurrency of the Lehman and Yao index. 

This small program creates a thread pool and spawns two writers and four readers, all working on the same shared data structure (a simple HashMap) although this can be generalized to any data structure. The HashMap is wrapped in a `RwLock` which is in turn wrapped in an `Arc`.

To run this program, make sure you have the Rust toolchain installed. If the following commands work, you have it installed:
<ul>
    <li><code>cargo --version</code></li>
    <li><code>rustc --version</code></li>
</ul>

You can download the Rust toolchain from here: [Rust Official Page](https://www.rust-lang.org/tools/install)