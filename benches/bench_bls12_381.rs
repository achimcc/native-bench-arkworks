use criterion::Criterion;

pub fn bench_bls12_381(c: &mut Criterion) {
    // let mut c = Criterion::default();
    let mut group = c.benchmark_group("bls12_381");
    group.bench_function("normal", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bls12_381::do_pairing();
        });
    });
    group.bench_function("msm g1, 10 arguments", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bls12_381::do_msm_g1(10);
        });
    });
    group.bench_function("msm g1, 1000 arguments", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bls12_381::do_msm_g1(1000);
        });
    });
    group.bench_function("msm g2, 10 arguments", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bls12_381::do_msm_g2(10);
        });
    });
    group.bench_function("msm g2, 1000 arguments", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bls12_381::do_msm_g2(1000);
        });
    });
    group.bench_function("mul affine g1", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bls12_381::do_mul_affine_g1();
        });
    });
    group.bench_function("mul projective g1", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bls12_381::do_mul_projective_g1();
        });
    });
    group.bench_function("mul affine g2", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bls12_381::do_mul_affine_g2();
        });
    });
    group.bench_function("mul projective g2", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bls12_381::do_mul_projective_g2();
        });
    });
    group.bench_function("groth16", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bls12_381::do_verify_groth16();
        });
    });
    group.finish();
}
