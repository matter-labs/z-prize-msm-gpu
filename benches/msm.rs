use ark_bls12_377::G1Affine;
use ark_ff::BigInteger256;
use criterion::{criterion_group, criterion_main, Criterion};
use std::str::FromStr;
use z_prize_msm_gpu::*;

fn criterion_benchmark(c: &mut Criterion) {
    let bench_npow = std::env::var("BENCH_NPOW").unwrap_or("26".to_string());
    let npoints_npow = i32::from_str(&bench_npow).unwrap();

    let batches = 4;
    let (points, scalars) =
        util::generate_points_scalars::<G1Affine>(1usize << npoints_npow, batches);
    let scalars = unsafe { std::mem::transmute::<&[_], &[BigInteger256]>(scalars.as_slice()) };
    let mut context = create_msm_context(points.as_slice());
    let mut group = c.benchmark_group("CUDA");
    group.sample_size(10);
    let name = format!("2**{}x{}", npoints_npow, batches);
    group.bench_function(name, |b| {
        b.iter(|| {
            let _ = execute_batch_msm(&mut context, scalars);
        })
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
