use ark_std::{io::Error, vec::Vec};

pub fn do_msm_sw(
    bases: &[sp_ark_models::short_weierstrass::Affine<ark_ed_on_bls12_381::SWConfig>],
    scalars: &[<ark_ed_on_bls12_381::SWConfig as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
    let _out = <ark_ed_on_bls12_381::EdwardsConfig as SWCurveConfig>::msm(bases, scalars);
    Ok(())
}

pub fn do_msm_te(
    bases: &[ark_ec::twisted_edwards::Affine<ark_ed_on_bls12_381::JubjubConfig>],
    scalars: &[<ark_ed_on_bls12_381::EdwardsConfig as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
    let _out = <ark_ed_on_bls12_381::JubjubConfig as ark_ec::twisted_edwards::TECurveConfig>::msm(
        bases, scalars,
    );
    Ok(())
}

pub fn do_mul_affine_sw() -> Result<(), Error> {
    let _out = <ark_ed_on_bls12_381::EdwardsConfig as SWCurveConfig>::mul_affine(
        &ark_ed_on_bls12_381::SWAffine::generator(),
        &[2u64],
    );
    Ok(())
}

pub fn do_mul_affine_te() -> Result<(), Error> {
    let _out =
		<ark_ed_on_bls12_381::EdwardsConfig as sp_ark_models::short_weierstrass::SWCurveConfig>::mul_affine(
			&ark_ed_on_bls12_381::SWAffine::generator(),
			&[2u64],
		);
    Ok(())
}

pub fn do_mul_projective_sw() -> Result<(), Error> {
    let _out =
		<ark_ed_on_bls12_381::EdwardsConfig as sp_ark_models::short_weierstrass::SWCurveConfig>::mul_projective(
			&ark_ed_on_bls12_381::SWProjective::generator(),
			&[2u64],
		);
    Ok(())
}

pub fn do_mul_projective_te() -> Result<(), Error> {
    let _out = <ark_ed_on_bls12_381::EdwardsConfig as ark_ec::twisted_edwards::TECurveConfig>::mul_projective(
		&ark_ed_on_bls12_381::EdwardsProjective::generator(),
		&[2u64],
	);
    Ok(())
}
