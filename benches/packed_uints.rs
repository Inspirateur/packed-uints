use packed_uints::PackedUints;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{seq::SliceRandom, Rng};
const N: usize = 1_000_000;

fn rand_read(packed: &PackedUints, indices: &Vec<usize>) {
    for i in indices.into_iter() {
        let _ = packed.get(*i);
    }
}

fn rand_write(packed: &mut PackedUints, indices: &Vec<usize>) {
    for i in indices.into_iter() {
        packed.set(*i, 1);
    }
}

fn from_vec(values: &Vec<usize>)-> PackedUints {
    PackedUints::from(values.as_slice())
}

fn read_benchmark<const BITSIZE: u32>(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    // generate a random packed array
    let mut values = vec![0; N];
    values.fill_with(|| rng.gen_range(0..2_usize.pow(BITSIZE)));
    let packed = PackedUints::from(values.as_slice());
    // generate random indices
    let mut indices: Vec<usize> = (0..N).collect();
    indices.shuffle(&mut rng);
    // array for reading into
    c.bench_function(&format!("packed_uints_rand_read-1m-{}", BITSIZE), |b| {
        b.iter(|| rand_read(black_box(&packed), black_box(&indices)))
    });
}

fn write_benchmark<const BITSIZE: u32>(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    // generate random indices
    let mut indices: Vec<usize> = (0..N).collect();
    indices.shuffle(&mut rng);
    // generate a random packed array to write to
    let mut values = vec![0; N];
    values.fill_with(|| rng.gen_range(0..2_usize.pow(BITSIZE)));
    let mut packed = PackedUints::from(values.as_slice());
    c.bench_function(&format!("packed_uints_rand_write-1m-{}", BITSIZE), |b| {
        b.iter(|| rand_write(black_box(&mut packed), black_box(&indices)))
    });
}

fn from_vec_benchmark(c: &mut Criterion) {
    // values to be initialized from
    let values: Vec<usize> = (0..N).collect();
    c.bench_function(&format!("packed_uints_from_vec-1m"), |b| {
        b.iter(|| from_vec(black_box(&values)))
    });
}

criterion_group!(
    packed_uints, 
    read_benchmark<4>, 
    write_benchmark<4>,
    from_vec_benchmark
);
criterion_main!(packed_uints);
