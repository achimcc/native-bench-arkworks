use criterion::{criterion_group, criterion_main, Criterion};
mod bench_bls12_381;
use bench_bls12_381::bench_bls12_381;
mod bench_bls12_377;
use bench_bls12_377::bench_bls12_377;
mod bench_bw6_761;
use bench_bw6_761::bench_bw6_761;
mod bench_ed_on_bls12_381;
use bench_ed_on_bls12_381::bench_ed_on_bls12_381;
mod bench_ed_on_bls12_377;
use bench_ed_on_bls12_377::bench_ed_on_bls12_377;

criterion_group! {
    name = arkworks;
    config = Criterion::default();
    targets =
        bench_bls12_377, bench_bls12_381, bench_bw6_761,
        bench_ed_on_bls12_377, bench_ed_on_bls12_381
}

criterion_main!(arkworks);
