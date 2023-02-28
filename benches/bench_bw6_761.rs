use criterion::Criterion;

pub fn bench_bw6_761(c: &mut Criterion) {
    // let mut c = Criterion::default();
    let mut group = c.benchmark_group("bw6_761");
    group.bench_function("msm g1, 10 arguments", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bw6_761::do_msm_g1(10);
        });
    });
    group.bench_function("normal, 1000 arguments", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bw6_761::do_msm_g1(1000);
        });
    });
    group.bench_function("msm g2, 10 arguments", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bw6_761::do_msm_g2(10);
        });
    });
    group.bench_function("msm g2, 1000 arguments", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bw6_761::do_msm_g2(1000);
        });
    });
    group.bench_function("mul affine g1", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bw6_761::do_mul_affine_g1();
        });
    });
    group.bench_function("mul projective g1", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bw6_761::do_mul_projective_g1();
        });
    });
    group.bench_function("mul affine g2", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bw6_761::do_mul_affine_g2();
        });
    });
    group.bench_function("mul projective g2", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bw6_761::do_mul_projective_g2();
        });
    });
    group.bench_function("pairing", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bw6_761::do_pairing();
        });
    });
    group.finish();
}
