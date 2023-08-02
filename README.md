# packed-uints
Array of uints that scales from u4 to u32 to minimize wasted space while still being fast. Credits to [Martin Janin](https://github.com/Involture) for working on the implementation!

This is a very specialized structure with niche applications, in my case it's for storing Bloc data in Chunks for a voxel game.

## Usage
```rust
use packed_uints::PackedUints;

// initialize an empty array of length 100 (filled with 0)
let arr = PackedUints::new(100);
// or initialize it from an existing one
let values: Vec<usize> = (0..=15).collect();
let arr = PackedUints::from(values.as_slice());
// set a value at index 3
arr.set(3, 42);
// get the value at index 3
assert_eq!(arr.get(3), 42);
```

in our example, we initialized the array with values ranging from 0 to 15, PackedUints will thus chose to represent each value on 4 bits (packing 2 values on an u8) 
because 4 bits is enough to represent values up to 15. 
However the moment we set the value at index 3 to 42, PackedUints switches to 8 bit representation (reallocating the array in the process) to accomodate this new value.

*Note: PackedUints only does upscaling, for performance reasons.*

## Benchmark
I compared PackedUints to the performance of a regular `Vec<u32>` as well as [Unthbuf](https://github.com/Longor1996/unthbuf), a crate with similar purposes except it gives the bitsize decision to the user instead of upscaling automatically like PackedUints does. Unthbuf also has the benefit of allowing non aligned values like u5 whereas PackedUints only supports u4, u8, u16 and u32.

The 3 structures are benchmarked on: 
- random read performance (1 million "u4" values)
- random write performance (1 million "u4" values)
- initialisation from an existing Vec (1 million u32 values)

Here are the results on an Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz CPU:
```
vec_read                time:   379.39 µs
vec_write               time:   2.0634 ms
vec_from_vec            time:   1.7448 ms
---
packed_uints_read       time:   438.08 µs
packed_uints_write      time:   3.3806 ms
packed_uints_from_vec   time:   1.9934 ms
---
unthbuf_read            time:   533.75 µs
unthbuf_write           time:   3.1632 ms
unthbuf_vec             time:   6.2827 ms
```

As you can see both crates have similar performance, not far from `Vec<u32>`, with packed_uints being slightly faster on random read significantly faster on initialization from a Vec.
