use ark_ec::{twisted_edwards::TECurveConfig, AffineRepr, Group};
use ark_std::{io::Error, vec::Vec};

pub fn do_msm(samples: u32) -> Result<(), Error> {
    let g = ark_ed_on_bls12_377::EdwardsAffine::generator();
    let v: Vec<_> = (0..samples).map(|_| g).collect();
    let scalars: Vec<_> = (0..samples).map(|_| ark_ff::Fp::from(2u64)).collect();
    let _out =
        <ark_ed_on_bls12_377::EdwardsConfig as ark_ec::models::twisted_edwards::TECurveConfig>::msm(
            &v[..],
            &scalars[..],
        );
    Ok(())
}

pub fn do_mul_affine() -> Result<(), Error> {
    let _out =
        <ark_ed_on_bls12_377::EdwardsConfig as ark_ec::models::twisted_edwards::TECurveConfig>::msm(
            &[ark_ed_on_bls12_377::EdwardsAffine::generator()],
            &[2u64.into()],
        );
    Ok(())
}

pub fn do_mul_projective() -> Result<(), Error> {
    let _out = <ark_ed_on_bls12_377::EdwardsConfig as TECurveConfig>::mul_projective(
        &ark_ed_on_bls12_377::EdwardsProjective::generator(),
        &[2u64],
    );
    Ok(())
}
