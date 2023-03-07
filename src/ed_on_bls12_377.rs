use ark_ec::{AffineRepr, Group};
use ark_std::io::Error;

pub fn do_msm(
    bases: &[ark_ec::twisted_edwards::Affine<ark_ed_on_bls12_377::EdwardsConfig>],
    scalars: &[<ark_ed_on_bls12_377::EdwardsConfig as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
    let _out =
        <ark_ed_on_bls12_377::EdwardsConfig as ark_ec::models::twisted_edwards::TECurveConfig>::msm(
            bases, scalars,
        );
    Ok(())
}

pub fn do_mul_affine() -> Result<(), Error> {
    let _out =
		<ark_ed_on_bls12_377::EdwardsConfig as ark_ec::models::twisted_edwards::TECurveConfig>::mul_affine(
			&ark_ed_on_bls12_377::EdwardsAffine::generator(),
			&[2u64],
		);
    Ok(())
}

pub fn do_mul_projective() -> Result<(), Error> {
    let _out = <ark_ed_on_bls12_377::EdwardsConfig as ark_ec::twisted_edwards::TECurveConfig>::mul_projective(
        &ark_ed_on_bls12_377::EdwardsProjective::generator(),
        &[2u64],
    );
    Ok(())
}
