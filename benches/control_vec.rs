use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{Rng, seq::SliceRandom};
const N: usize = 1_000_000;

fn rand_read(packed: &Vec<u32>, indices: &Vec<usize>) {
    for i in indices.into_iter() {
        let _ = packed[*i];
    }
}

fn rand_write(packed: &mut Vec<u32>, indices: &Vec<usize>) {
    for i in indices.into_iter() {
        packed[*i] = 1;
    }
}

fn from_vec(values: &Vec<usize>)-> Vec<usize> {
    values.clone()
}

fn read_benchmark<const BITSIZE: u32>(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    // generate a random packed array
    let mut packed = vec![0; N];
    packed.fill_with(|| rng.gen_range(0..2u32.pow(BITSIZE)));
    let packed = packed.clone();
    // generate random indices
    let mut indices: Vec<usize> = (0..N).collect();
    indices.shuffle(&mut rng);
    c.bench_function(&format!("vec_rand_read-1m-{}", BITSIZE), |b| b.iter(|| rand_read(
        black_box(&packed), black_box(&indices)
    )));
}

fn write_benchmark<const BITSIZE: u32>(c: &mut Criterion) {
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    // generate random indices
    let mut indices: Vec<usize> = (0..N).collect();
    indices.shuffle(&mut rng);
    // generate a random packed array to write to
    let mut packed = vec![0; N];
    packed.fill_with(|| rng.gen_range(0..2u32.pow(BITSIZE)));
    c.bench_function(&format!("vec_rand_write-1m-{}", BITSIZE), |b| b.iter(|| rand_write(
        black_box(&mut packed), black_box(&indices)
    )));
}

fn from_vec_benchmark(c: &mut Criterion) {
    // values to be initialized from
    let values: Vec<usize> = (0..N).collect();
    c.bench_function(&format!("vec_from_vec-1m"), |b| {
        b.iter(|| from_vec(black_box(&values)))
    });
}

criterion_group!(control_vec, read_benchmark<4>, write_benchmark<4>, from_vec_benchmark);
criterion_main!(control_vec);