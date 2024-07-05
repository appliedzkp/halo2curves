use core::convert::TryInto;
use halo2derive::impl_field;
use rand::RngCore;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

impl_field!(
    secp256k1_scalar,
    Fq,
    modulus = "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
    mul_gen = "7",
    zeta = "5363ad4cc05c30e0a5261c028812645a122e22ea20816678df02967c1b23bd72",
    from_uniform = [48, 64],
    endian = "little",
);

crate::extend_field_legendre!(Fq);
crate::impl_binops_calls!(Fq);
crate::impl_binops_additive!(Fq, Fq);
crate::impl_binops_multiplicative!(Fq, Fq);
crate::field_bits!(Fq);
crate::serialize_deserialize_primefield!(Fq);
crate::impl_from_u64!(Fq);

#[cfg(test)]
mod test {
    use super::*;
    crate::field_testing_suite!(Fq, "field_arithmetic");
    crate::field_testing_suite!(Fq, "conversion");
    crate::field_testing_suite!(Fq, "serialization");
    crate::field_testing_suite!(Fq, "quadratic_residue");
    crate::field_testing_suite!(Fq, "bits");
    crate::field_testing_suite!(Fq, "serialization_check");
    crate::field_testing_suite!(Fq, "constants");
    crate::field_testing_suite!(Fq, "sqrt");
    crate::field_testing_suite!(Fq, "zeta");
    crate::field_testing_suite!(Fq, "from_uniform_bytes", 48, 64);
}
