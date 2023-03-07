use criterion::Criterion;

pub fn bench_bw6_761(c: &mut Criterion) {
    let (bases_g1_10, scalars_g1_10) =
        generate_arguments::<ark_ec::short_weierstrass::Projective<ark_bw6_761::g1::Config>>(10);
    let (bases_g1_1000, scalars_g1_1000) =
        generate_arguments::<ark_ec::short_weierstrass::Projective<ark_bw6_761::g1::Config>>(1000);
    let (bases_g2_10, scalars_g2_10) =
        generate_arguments::<ark_ec::short_weierstrass::Projective<ark_bw6_761::g2::Config>>(10);
    let (bases_g2_1000, scalars_g2_1000) =
        generate_arguments::<ark_ec::short_weierstrass::Projective<ark_bw6_761::g2::Config>>(1000);
    let mut group = c.benchmark_group("bw6_761");
    group.bench_function("msm g1, 10 arguments", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bw6_761::do_msm_g1(bases_g1_10, scalars_g1_10);
        });
    });
    group.bench_function("normal, 1000 arguments", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bw6_761::do_msm_g1(bases_g1_1000, scalars_g1_1000);
        });
    });
    group.bench_function("msm g2, 10 arguments", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bw6_761::do_msm_g2(bases_g2_10, scalars_g2_10);
        });
    });
    group.bench_function("msm g2, 1000 arguments", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bw6_761::do_msm_g2(bases_g2_1000, scalars_g2_1000);
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
