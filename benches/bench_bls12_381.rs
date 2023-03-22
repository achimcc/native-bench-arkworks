use ark_ec::CurveGroup;
use criterion::Criterion;
use native_bench_arkworks::utils::{
    generate_msm_args, generate_pairing_args, generate_scalar_args,
};

pub fn bench_bls12_381(c: &mut Criterion) {
    let (bases_g1_10, scalars_g1_10) =
        generate_msm_args::<ark_ec::short_weierstrass::Projective<ark_bls12_381::g1::Config>>(10);
    let bases_g1_10 = bases_g1_10
        .iter()
        .map(|base| base.into_affine())
        .collect::<Vec<_>>();
    let (bases_g1_1000, scalars_g1_1000) =
        generate_msm_args::<ark_ec::short_weierstrass::Projective<ark_bls12_381::g1::Config>>(1000);
    let bases_g1_1000 = bases_g1_1000
        .iter()
        .map(|base| base.into_affine())
        .collect::<Vec<_>>();
    let (bases_g2_10, scalars_g2_10) =
        generate_msm_args::<ark_ec::short_weierstrass::Projective<ark_bls12_381::g2::Config>>(10);
    let bases_g2_10 = bases_g2_10
        .iter()
        .map(|base| base.into_affine())
        .collect::<Vec<_>>();
    let (bases_g2_1000, scalars_g2_1000) =
        generate_msm_args::<ark_ec::short_weierstrass::Projective<ark_bls12_381::g2::Config>>(1000);
    let bases_g2_1000 = bases_g2_1000
        .iter()
        .map(|base| base.into_affine())
        .collect::<Vec<_>>();
    let mut group = c.benchmark_group("bls12_381");

    let (a, b) = generate_pairing_args::<ark_bls12_381::G1Affine, ark_bls12_381::G2Affine>();
    group.bench_function("normal", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bls12_381::do_pairing(a, b);
        });
    });
    group.bench_function("msm g1, 10 arguments", |b| {
        b.iter(|| {
            let _ =
                native_bench_arkworks::bls12_381::do_msm_g1(&bases_g1_10[..], &scalars_g1_10[..]);
        });
    });
    group.bench_function("msm g1, 1000 arguments", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bls12_381::do_msm_g1(
                &bases_g1_1000[..],
                &scalars_g1_1000[..],
            );
        });
    });
    group.bench_function("msm g2, 10 arguments", |b| {
        b.iter(|| {
            let _ =
                native_bench_arkworks::bls12_381::do_msm_g2(&bases_g2_10[..], &scalars_g2_10[..]);
        });
    });
    group.bench_function("msm g2, 1000 arguments", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bls12_381::do_msm_g2(
                &bases_g2_1000[..],
                &scalars_g2_1000[..],
            );
        });
    });
    let (base, scalar) = generate_scalar_args::<ark_bls12_381::G1Affine>();
    group.bench_function("mul affine g1", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bls12_381::do_mul_affine_g1(&base, &scalar);
        });
    });
    let (base, scalar) = generate_scalar_args::<ark_bls12_381::G1Projective>();
    group.bench_function("mul projective g1", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bls12_381::do_mul_projective_g1(&base, &scalar);
        });
    });
    let (base, scalar) = generate_scalar_args::<ark_bls12_381::G2Affine>();
    group.bench_function("mul affine g2", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bls12_381::do_mul_affine_g2(&base, &scalar);
        });
    });
    let (base, scalar) = generate_scalar_args::<ark_bls12_381::G2Projective>();
    group.bench_function("mul projective g2", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bls12_381::do_mul_projective_g2(&base, &scalar);
        });
    });
    group.bench_function("groth16", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::bls12_381::do_verify_groth16();
        });
    });
    group.finish();
}
