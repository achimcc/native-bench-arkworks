use ark_ec::CurveGroup;
use criterion::Criterion;
use native_bench_arkworks::utils::{
    generate_msm_args, generate_pairing_args, generate_scalar_args,
};

pub fn bench_bw6_761(c: &mut Criterion) {
    let (bases_g1_10, scalars_g1_10) =
        generate_msm_args::<ark_ec::short_weierstrass::Projective<ark_bw6_761::g1::Config>>(10);
    let bases_g1_10 = bases_g1_10
        .iter()
        .map(|base| base.into_affine())
        .collect::<Vec<_>>();
    let (bases_g1_1000, scalars_g1_1000) =
        generate_msm_args::<ark_ec::short_weierstrass::Projective<ark_bw6_761::g1::Config>>(1000);
    let bases_g1_1000 = bases_g1_1000
        .iter()
        .map(|base| base.into_affine())
        .collect::<Vec<_>>();
    let (bases_g2_10, scalars_g2_10) =
        generate_msm_args::<ark_ec::short_weierstrass::Projective<ark_bw6_761::g2::Config>>(10);
    let bases_g2_10 = bases_g2_10
        .iter()
        .map(|base| base.into_affine())
        .collect::<Vec<_>>();
    let (bases_g2_1000, scalars_g2_1000) =
        generate_msm_args::<ark_ec::short_weierstrass::Projective<ark_bw6_761::g2::Config>>(1000);
    let bases_g2_1000 = bases_g2_1000
        .iter()
        .map(|base| base.into_affine())
        .collect::<Vec<_>>();
    let mut group = c.benchmark_group("bw6_761");
    group.bench_function("msm g1, 10 arguments", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bw6_761::do_msm_g1(&bases_g1_10[..], &scalars_g1_10[..]);
        });
    });
    group.bench_function("normal, 1000 arguments", |b| {
        b.iter(|| {
            let _ =
                native_bench_arkworks::bw6_761::do_msm_g1(&bases_g1_1000[..], &scalars_g1_1000[..]);
        });
    });
    group.bench_function("msm g2, 10 arguments", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bw6_761::do_msm_g2(&bases_g2_10[..], &scalars_g2_10[..]);
        });
    });
    group.bench_function("msm g2, 1000 arguments", |b| {
        b.iter(|| {
            let _ =
                native_bench_arkworks::bw6_761::do_msm_g2(&bases_g2_1000[..], &scalars_g2_1000[..]);
        });
    });
    let (base, scalar) = generate_scalar_args::<ark_bw6_761::G1Affine>();
    group.bench_function("mul affine g1", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bw6_761::do_mul_affine_g1(&base, &scalar);
        });
    });
    let (base, scalar) = generate_scalar_args::<ark_bw6_761::G1Projective>();
    group.bench_function("mul projective g1", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bw6_761::do_mul_projective_g1(&base, &scalar);
        });
    });
    let (base, scalar) = generate_scalar_args::<ark_bw6_761::G2Affine>();
    group.bench_function("mul affine g2", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bw6_761::do_mul_affine_g2(&base, &scalar);
        });
    });
    let (base, scalar) = generate_scalar_args::<ark_bw6_761::G2Projective>();
    group.bench_function("mul projective g2", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bw6_761::do_mul_projective_g2(&base, &scalar);
        });
    });
    let (a, b) = generate_pairing_args::<ark_bw6_761::G1Affine, ark_bw6_761::G2Affine>();
    group.bench_function("pairing", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bw6_761::do_pairing(a, b);
        });
    });
    group.finish();
}
