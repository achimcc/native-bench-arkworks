use ark_std::io::Error;

pub fn do_msm_sw(
    bases: &[ark_ec::short_weierstrass::Affine<ark_ed_on_bls12_381::SWConfig>],
    scalars: &[<ark_ed_on_bls12_381::SWConfig as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
    let _out =
        <ark_ed_on_bls12_381::EdwardsConfig as ark_ec::short_weierstrass::SWCurveConfig>::msm(
            bases, scalars,
        );
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

pub fn do_mul_affine_sw(base: &ark_ed_on_bls12_381::SWAffine, scalar: u64) -> Result<(), Error> {
    let _out = <ark_ed_on_bls12_381::EdwardsConfig as ark_ec::short_weierstrass::SWCurveConfig>::mul_affine(
        base,
        &[scalar],
    );
    Ok(())
}

pub fn do_mul_affine_te(base: &ark_ed_on_bls12_381::SWAffine, scalar: u64) -> Result<(), Error> {
    let _out =
		<ark_ed_on_bls12_381::EdwardsConfig as ark_ec::short_weierstrass::SWCurveConfig>::mul_affine(
			base,
			&[scalar],
		);
    Ok(())
}

pub fn do_mul_projective_sw(
    base: &ark_ed_on_bls12_381::SWProjective,
    scalar: u64,
) -> Result<(), Error> {
    let _out =
		<ark_ed_on_bls12_381::EdwardsConfig as ark_ec::short_weierstrass::SWCurveConfig>::mul_projective(
			base,
			&[scalar],
		);
    Ok(())
}

pub fn do_mul_projective_te(
    base: &ark_ed_on_bls12_381::EdwardsProjective,
    scalar: u64,
) -> Result<(), Error> {
    let _out = <ark_ed_on_bls12_381::EdwardsConfig as ark_ec::twisted_edwards::TECurveConfig>::mul_projective(
		base,
		&[scalar],
	);
    Ok(())
}
