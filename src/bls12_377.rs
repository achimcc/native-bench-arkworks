use ark_ec::{pairing::Pairing, AffineRepr, Group};
use ark_std::io::Error;

pub fn do_pairing() -> Result<(), Error> {
    let _out = ark_bls12_377::Bls12_377::multi_pairing(
        [ark_bls12_377::G1Affine::generator()],
        [ark_bls12_377::G2Affine::generator()],
    );
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

pub fn do_mul_projective_g1() -> Result<(), Error> {
    let _out =
        <ark_bls12_377::g1::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_projective(
            &ark_bls12_377::G1Projective::generator(),
            &[2u64],
        );
    Ok(())
}

pub fn do_mul_affine_g1() -> Result<(), Error> {
    let _out = <ark_bls12_377::g1::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_affine(
        &ark_bls12_377::G1Affine::generator(),
        &[2u64],
    );
    Ok(())
}

pub fn do_mul_projective_g2() -> Result<(), Error> {
    let _out =
        <ark_bls12_377::g2::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_projective(
            &ark_bls12_377::G2Projective::generator(),
            &[2u64],
        );
    Ok(())
}

pub fn do_mul_affine_g2() -> Result<(), Error> {
    let _out = <ark_bls12_377::g2::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_affine(
        &ark_bls12_377::G2Affine::generator(),
        &[2u64],
    );
    Ok(())
}
