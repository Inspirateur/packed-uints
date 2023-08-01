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
I compared PackedUints to the performance of a regular `Vec<u32>` as well as [Unthbuf](https://github.com/Longor1996/unthbuf), a crate with similar purposes except it gives the bitsize decision to the user instead of upscaling automatically like PackedUints does.

The 3 structures are benchmarked on: 
- random read performance (1 million "u4" values)
- random write performance (1 million "u4" values)
- initialisation from an existing Vec (1 million u32 values)

Here are the results on my Intel(R) Xeon(R) CPU E5-1650 v3 @ 3.50GHz CPU (from 2014 so quite old):
```
vec_rand_read-1m-4             time:   [331.06 µs 332.79 µs 334.85 µs]
vec_rand_write-1m-4            time:   [2.8150 ms 2.8530 ms 2.8980 ms]
vec_from_vec-1m                time:   [2.1652 ms 2.1812 ms 2.1973 ms]
---
packed_uints_rand_read-1m-4    time:   [436.69 µs 437.80 µs 438.78 µs]
packed_uints_rand_write-1m-4   time:   [5.0530 ms 5.1564 ms 5.2979 ms]
packed_uints_from_vec-1m       time:   [2.4395 ms 2.4574 ms 2.4771 ms]
---
unthbuf_rand_read-1m-4         time:   [469.39 µs 480.58 µs 494.96 µs]
unthbuf_rand_write-1m-4        time:   [4.4435 ms 4.4535 ms 4.4659 ms]
unthbuf_from_vec-1m            time:   [9.2428 ms 9.3750 ms 9.5319 ms]
```

As you can see both crates have similar performance, not far from `Vec<u32>`, packed_uints is slightly slower on random writes but faster on initialization from a Vec.
