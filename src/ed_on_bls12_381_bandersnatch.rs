use ark_std::io::Error;

pub fn do_msm_sw(
    bases: &[ark_ec::short_weierstrass::Affine<ark_ed_on_bls12_381_bandersnatch::SWConfig>],
    scalars: &[<ark_ed_on_bls12_381_bandersnatch::SWConfig as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
    let _out =
        <ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as ark_ec::short_weierstrass::SWCurveConfig>::msm(
            bases, scalars,
        );
    Ok(())
}

pub fn do_msm_te(
    bases: &[ark_ec::twisted_edwards::Affine<
        ark_ed_on_bls12_381_bandersnatch::BandersnatchConfig,
    >],
    scalars: &[<ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
    let _out = <ark_ed_on_bls12_381_bandersnatch::BandersnatchConfig as ark_ec::twisted_edwards::TECurveConfig>::msm(
        bases, scalars,
    );
    Ok(())
}

pub fn do_mul_affine_sw(
    base: &ark_ed_on_bls12_381_bandersnatch::SWAffine,
    scalar: u64,
) -> Result<(), Error> {
    let _out = <ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as ark_ec::short_weierstrass::SWCurveConfig>::mul_affine(
        base,
        &[scalar],
    );
    Ok(())
}

pub fn do_mul_affine_te(
    base: &ark_ed_on_bls12_381_bandersnatch::SWAffine,
    scalar: u64,
) -> Result<(), Error> {
    let _out =
		<ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as ark_ec::short_weierstrass::SWCurveConfig>::mul_affine(
			base,
			&[scalar],
		);
    Ok(())
}

pub fn do_mul_projective_sw(
    base: &ark_ed_on_bls12_381_bandersnatch::SWProjective,
    scalar: u64,
) -> Result<(), Error> {
    let _out =
		<ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as ark_ec::short_weierstrass::SWCurveConfig>::mul_projective(
			base,
			&[scalar],
		);
    Ok(())
}

pub fn do_mul_projective_te(
    base: &ark_ed_on_bls12_381_bandersnatch::EdwardsProjective,
    scalar: u64,
) -> Result<(), Error> {
    let _out = <ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as ark_ec::twisted_edwards::TECurveConfig>::mul_projective(
		base,
		&[scalar],
	);
    Ok(())
}
