use ark_ec::CurveGroup;
use criterion::Criterion;
use native_bench_arkworks::utils::{generate_msm_args, generate_scalar_args};

pub fn bench_ed_on_bls12_377(c: &mut Criterion) {
    let (bases_10, scalars_10) = generate_msm_args::<ark_ed_on_bls12_377::EdwardsProjective>(10);
    let bases_10 = bases_10
        .iter()
        .map(|base| base.into_affine())
        .collect::<Vec<_>>();
    let (bases_1000, scalars_1000) =
        generate_msm_args::<ark_ed_on_bls12_377::EdwardsProjective>(1000);
    let bases_1000 = bases_1000
        .iter()
        .map(|base| base.into_affine())
        .collect::<Vec<_>>();
    let mut group = c.benchmark_group("ed_on_bls12_377");
    group.bench_function("msm, 10 arguments", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::ed_on_bls12_377::do_msm(&bases_10[..], &scalars_10[..]);
        });
    });
    group.bench_function("msm, 1000 arguments", |b| {
        b.iter(|| {
            let _ =
                native_bench_arkworks::ed_on_bls12_377::do_msm(&bases_1000[..], &scalars_1000[..]);
        });
    });
    let (base, scalar) = generate_scalar_args::<ark_ed_on_bls12_377::EdwardsAffine>();
    group.bench_function("mul affine", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::ed_on_bls12_377::do_mul_affine(&base, scalar);
        });
    });
    let (base, scalar) = generate_scalar_args::<ark_ed_on_bls12_377::EdwardsProjective>();
    group.bench_function("mul projective", |b| {
        b.iter(|| {
            let _ = native_bench_arkworks::ed_on_bls12_377::do_mul_projective(&base, scalar);
        });
    });
    group.finish();
}
