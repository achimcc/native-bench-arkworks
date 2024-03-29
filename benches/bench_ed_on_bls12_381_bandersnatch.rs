use ark_ec::CurveGroup;
use criterion::Criterion;
use native_bench_arkworks::utils::{generate_msm_args, generate_scalar_args};

pub fn bench_ed_on_bls12_381_bandersnatch(c: &mut Criterion) {
    let (bases_sw_10, scalars_sw_10) = generate_msm_args::<
        ark_ec::short_weierstrass::Projective<ark_ed_on_bls12_381_bandersnatch::EdwardsConfig>,
    >(10);
    let bases_sw_10 = bases_sw_10
        .iter()
        .map(|base| base.into_affine())
        .collect::<Vec<_>>();
    let (bases_sw_1000, scalars_sw_1000) = generate_msm_args::<
        ark_ec::short_weierstrass::Projective<ark_ed_on_bls12_381_bandersnatch::EdwardsConfig>,
    >(1000);
    let bases_sw_1000 = bases_sw_1000
        .iter()
        .map(|base| base.into_affine())
        .collect::<Vec<_>>();
    let (bases_te_10, scalars_te_10) =
        generate_msm_args::<ark_ed_on_bls12_381_bandersnatch::EdwardsProjective>(10);
    let bases_te_10 = bases_te_10
        .iter()
        .map(|base| base.into_affine())
        .collect::<Vec<_>>();
    let (bases_te_1000, scalars_te_1000) =
        generate_msm_args::<ark_ed_on_bls12_381_bandersnatch::EdwardsProjective>(1000);
    let bases_te_1000 = bases_te_1000
        .iter()
        .map(|base| base.into_affine())
        .collect::<Vec<_>>();
    let mut group = c.benchmark_group("ed_on_bls12_381_bandersnatch");

    let (base, scalar) = generate_scalar_args::<ark_ed_on_bls12_381_bandersnatch::SWAffine>();
    group.bench_function("mul affine sw", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::ed_on_bls12_381_bandersnatch::do_mul_affine_sw(
                &base, scalar,
            );
        });
    });
    let (base, scalar) = generate_scalar_args::<ark_ed_on_bls12_381_bandersnatch::SWProjective>();
    group.bench_function("mul projective sw", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::ed_on_bls12_381_bandersnatch::do_mul_projective_sw(
                &base, scalar,
            );
        });
    });
    let (base, scalar) = generate_scalar_args::<ark_ed_on_bls12_381_bandersnatch::SWAffine>();
    group.bench_function("mul affine te", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::ed_on_bls12_381_bandersnatch::do_mul_affine_te(
                &base, scalar,
            );
        });
    });
    let (base, scalar) =
        generate_scalar_args::<ark_ed_on_bls12_381_bandersnatch::EdwardsProjective>();
    group.bench_function("mul projective te", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::ed_on_bls12_381_bandersnatch::do_mul_projective_te(
                &base, scalar,
            );
        });
    });
    group.bench_function("msm sw, 10 arguments", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::ed_on_bls12_381_bandersnatch::do_msm_sw(
                &bases_sw_10[..],
                &scalars_sw_10[..],
            );
        });
    });
    group.bench_function("msm sw, 1000 arguments", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::ed_on_bls12_381_bandersnatch::do_msm_sw(
                &bases_sw_1000[..],
                &scalars_sw_1000[..],
            );
        });
    });
    group.bench_function("mwm te, 10 arguments", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::ed_on_bls12_381_bandersnatch::do_msm_te(
                &bases_te_10[..],
                &scalars_te_10[..],
            );
        });
    });
    group.bench_function("mwm te, 1000 arguments", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::ed_on_bls12_381_bandersnatch::do_msm_te(
                &bases_te_1000[..],
                &scalars_te_1000[..],
            );
        });
    });
    group.finish();
}
