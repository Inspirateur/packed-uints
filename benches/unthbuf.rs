use std::num::NonZeroU8;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{Rng, seq::SliceRandom};
use unthbuf::{UnthBuf, aligned::AlignedLayout};
const N: usize = 1_000_000;

fn rand_read(packed: &UnthBuf<AlignedLayout>, indices: &Vec<usize>) {
    for i in indices.into_iter() {
        let _ = packed.get(*i).unwrap();
    }
}

fn rand_write(packed: &mut UnthBuf<AlignedLayout>, indices: &Vec<usize>) {
    for i in indices.into_iter() {
        packed.set(*i, 1).unwrap();
    }
}

fn from_vec(values: Vec<usize>)-> UnthBuf<AlignedLayout> {
    let bits = values.iter().max().unwrap_or(&2).ilog2();
    UnthBuf::new_from_capacity_and_iter(
        NonZeroU8::try_from(bits as u8).unwrap(), N, values.into_iter()
    )
}

fn read_benchmark<const BITSIZE: u32>(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    // generate a random packed array
    let mut values = vec![0; N];
    values.fill_with(|| rng.gen_range(0..2_usize.pow(BITSIZE)));
    let packed = UnthBuf::new_from_capacity_and_iter(
        NonZeroU8::try_from(BITSIZE as u8).unwrap(), N, values.into_iter()
    );
    // generate random indices
    let mut indices: Vec<usize> = (0..N).collect();
    indices.shuffle(&mut rng);
    c.bench_function(&format!("unthbuf_rand_read-1m-{}", BITSIZE), |b| b.iter(|| rand_read(
        black_box(&packed), black_box(&indices)
    )));
}

fn write_benchmark<const BITSIZE: u32>(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    // generate random indices
    let mut indices: Vec<usize> = (0..N).collect();
    indices.shuffle(&mut rng);
    // generate a random packed array to write to
    let mut values = vec![0; N];
    values.fill_with(|| rng.gen_range(0..2_usize.pow(BITSIZE)));
    let mut packed = UnthBuf::new_from_capacity_and_iter(
        NonZeroU8::try_from(BITSIZE as u8).unwrap(), N, values.into_iter()
    );
    c.bench_function(&format!("unthbuf_rand_write-1m-{}", BITSIZE), |b| b.iter(|| rand_write(
        black_box(&mut packed), black_box(&indices)
    )));
}

fn from_vec_benchmark(c: &mut Criterion) {
    c.bench_function(&format!("unthbuf_from_vec-1m"), |b| {
        b.iter(|| from_vec(black_box((0..N).collect())))
    });
}

criterion_group!(
    unthbuf, 
    read_benchmark<4>, 
    write_benchmark<4>, 
    from_vec_benchmark
);
criterion_main!(unthbuf);