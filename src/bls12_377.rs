use ark_ec::{pairing::Pairing, AffineRepr, Group};
use ark_std::io::Error;

pub fn do_pairing(a: ark_bls12_377::G1Affine, b: ark_bls12_377::G2Affine) -> Result<(), Error> {
    let _out = ark_bls12_377::Bls12_377::multi_pairing([a], [b]);
    Ok(())
}

pub fn do_msm_g1(
    bases: &[ark_ec::short_weierstrass::Affine<ark_bls12_377::g1::Config>],
    scalars: &[<ark_bls12_377::g1::Config as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
    let _out = <ark_bls12_377::g1::Config as ark_ec::short_weierstrass::SWCurveConfig>::msm(
        bases, scalars,
    );
    Ok(())
}

pub fn do_msm_g2(
    bases: &[ark_ec::short_weierstrass::Affine<ark_bls12_377::g2::Config>],
    scalars: &[<ark_bls12_377::g2::Config as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
    let _out = <ark_bls12_377::g2::Config as ark_ec::short_weierstrass::SWCurveConfig>::msm(
        bases, scalars,
    );
    Ok(())
}

pub fn do_mul_projective_g1(base: &ark_bls12_377::G1Projective, scalar: u64) -> Result<(), Error> {
    let _out =
        <ark_bls12_377::g1::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_projective(
            base,
            &[scalar],
        );
    Ok(())
}

pub fn do_mul_affine_g1(base: &ark_bls12_377::G1Affine, scalar: u64) -> Result<(), Error> {
    let _out = <ark_bls12_377::g1::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_affine(
        base,
        &[scalar],
    );
    Ok(())
}

pub fn do_mul_projective_g2(base: &ark_bls12_377::G2Projective, scalar: u64) -> Result<(), Error> {
    let _out =
        <ark_bls12_377::g2::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_projective(
            base,
            &[scalar],
        );
    Ok(())
}

pub fn do_mul_affine_g2(base: &ark_bls12_377::G2Affine, scalar: u64) -> Result<(), Error> {
    let _out = <ark_bls12_377::g2::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_affine(
        base,
        &[scalar],
    );
    Ok(())
}
