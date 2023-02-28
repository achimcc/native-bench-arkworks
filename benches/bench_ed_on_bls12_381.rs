use criterion::Criterion;

pub fn bench_ed_on_bls12_381(c: &mut Criterion) {
    // let mut c = Criterion::default();
    let mut group = c.benchmark_group("ed_on_bls12_381");
    group.bench_function("mul affine sw", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::ed_on_bls12_381::do_mul_affine_sw();
        });
    });
    group.bench_function("mul projective sw", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::ed_on_bls12_381::do_mul_projective_sw();
        });
    });
    group.bench_function("mul affine te", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::ed_on_bls12_381::do_mul_affine_te();
        });
    });
    group.bench_function("mul projective te", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::ed_on_bls12_381::do_mul_projective_te();
        });
    });
    group.bench_function("msm sw, 10 arguments", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::ed_on_bls12_381::do_msm_sw(10);
        });
    });
    group.bench_function("msm sw, 1000 arguments", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::ed_on_bls12_381::do_msm_sw(1000);
        });
    });
    group.bench_function("mwm te, 10 arguments", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::ed_on_bls12_381::do_msm_te(10);
        });
    });
    group.bench_function("mwm te, 1000 arguments", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::ed_on_bls12_381::do_msm_te(1000);
        });
    });
    group.finish();
}
