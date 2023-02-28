use criterion::Criterion;

pub fn bench_ed_on_bls12_377(c: &mut Criterion) {
    // let mut c = Criterion::default();
    let mut group = c.benchmark_group("ed_on_bls12_377");
    group.bench_function("msm, 10 arguments", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::ed_on_bls12_377::do_msm(10);
        });
    });
    group.bench_function("msm, 1000 arguments", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::ed_on_bls12_377::do_msm(1000);
        });
    });
    group.bench_function("mul affine", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::ed_on_bls12_377::do_mul_affine();
        });
    });
    group.bench_function("mul projective", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::ed_on_bls12_377::do_mul_projective();
        });
    });
    group.finish();
}
