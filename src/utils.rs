use ark_std::{test_rng, vec::Vec, UniformRand};

pub fn generate_msm_args<Group: ark_ec::VariableBaseMSM>(
    size: u32,
) -> (Vec<Group>, Vec<Group::ScalarField>) {
    let rng = &mut test_rng();
    let scalars = (0..size)
        .map(|_| Group::ScalarField::rand(rng))
        .collect::<Vec<_>>();
    let bases = (0..size).map(|_| Group::rand(rng)).collect::<Vec<_>>();
    (bases, scalars)
}

fn generate_scalar() -> u64 {
    let rng = &mut test_rng();
    let scalar = u64::rand(rng);
    scalar
}

fn generate_base<Group: UniformRand>() -> Group {
    let rng = &mut test_rng();
    let base = Group::rand(rng);
    base
}

pub fn generate_scalar_args<Group: UniformRand>() -> (Group, u64) {
    let serialized_scalar = generate_scalar();
    let serialized_base = generate_base::<Group>();
    (serialized_base, serialized_scalar)
}

pub fn generate_pairing_args<GroupA: UniformRand, GroupB: UniformRand>() -> (GroupA, GroupB) {
    (generate_base::<GroupA>(), generate_base::<GroupB>())
}
