// use ckb_opt_fips202::{absorb, squeeze};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use sha3::{
    digest::{ExtendableOutput, Update, XofReader},
    Shake256,
};

fn bench_shake256(c: &mut Criterion) {
    let mut group = c.benchmark_group("shake256");

    let size_list = [32usize, 256, 1024, 4096];
    for size in size_list {
        let m = vec![0xA5u8; size];
        group.bench_with_input(BenchmarkId::new("ckb_opt_fips202", size), &m, |b, data| {
            b.iter(|| {
                let mut state = ckb_opt_fips202::Shake256::new();
                state.absorb(black_box(data));
                state.finalize();
                let mut out = [0u8; 64];
                state.squeeze(black_box(&mut out));
                black_box(out)
            });
        });
        group.bench_with_input(BenchmarkId::new("sha3", size), &m, |b, data| {
            b.iter(|| {
                let mut hasher = Shake256::default();
                hasher.update(black_box(data));
                let mut reader = hasher.finalize_xof();
                let mut out = [0u8; 64];
                reader.read(black_box(&mut out));
                black_box(out)
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_shake256);
criterion_main!(benches);
