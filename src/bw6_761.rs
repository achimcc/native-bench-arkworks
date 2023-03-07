use ark_std::{io::Error, vec::Vec};

pub fn do_msm_g1(
    bases: &[ark_ec::short_weierstrass::Affine<ark_bw6_761::g1::Config>],
    scalars: &[<ark_bw6_761::g1::Config as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
    let _out = <ark_bw6_761::g1::Config as SWCurveConfig>::msm(bases, scalars);

    Ok(())
}

pub fn do_msm_g2(
    bases: &[ark_ec::models::short_weierstrass::Affine<ark_bw6_761::g2::Config>],
    scalars: &[<ark_bw6_761::g2::Config as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
    let _out = <ark_bw6_761::g2::Config as SWCurveConfig>::msm(bases, scalars);
    Ok(())
}

pub fn do_mul_affine_g1() -> Result<(), Error> {
    let _out = <ark_bw6_761::g1::Config as SWCurveConfig>::mul_affine(
        &ark_bw6_761::G1Affine::generator(),
        &[2u64],
    );
    Ok(())
}

pub fn do_mul_projective_g1() -> Result<(), Error> {
    let _out = <ark_bw6_761::g1::Config as SWCurveConfig>::mul_projective(
        &ark_bw6_761::G1Projective::generator(),
        &[2u64],
    );
    Ok(())
}

pub fn do_mul_affine_g2() -> Result<(), Error> {
    let _out = <ark_bw6_761::g2::Config as SWCurveConfig>::mul_affine(
        &ark_bw6_761::G2Affine::generator(),
        &[2u64],
    );
    Ok(())
}

pub fn do_mul_projective_g2() -> Result<(), Error> {
    let _out = <ark_bw6_761::g2::Config as SWCurveConfig>::mul_projective(
        &ark_bw6_761::G2Projective::generator(),
        &[2u64],
    );
    Ok(())
}

pub fn do_pairing() -> Result<(), Error> {
    let _out = ark_bw6_761::BW6_761::multi_pairing(
        [ark_bw6_761::G1Affine::generator()],
        [ark_bw6_761::G2Affine::generator()],
    );
    Ok(())
}
