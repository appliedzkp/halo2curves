use core::convert::TryInto;
use halo2derive::impl_field;
use rand::RngCore;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

impl_field!(
    secp256r1_base,
    Fp,
    modulus = "ffffffff00000001000000000000000000000000ffffffffffffffffffffffff",
    mul_gen = "6",
    zeta = "4d6ea8928adb86cf62388a8e0ef623312e68c59bdef3e53fd964598eb819acce",
    from_uniform = [48, 64],
    endian = "little",
);

crate::extend_field_legendre!(Fp);
crate::impl_binops_calls!(Fp);
crate::impl_binops_additive!(Fp, Fp);
crate::impl_binops_multiplicative!(Fp, Fp);
crate::field_bits!(Fp);
crate::serialize_deserialize_primefield!(Fp);
crate::impl_from_u64!(Fp);
crate::impl_from_bool!(Fp);

#[cfg(test)]
mod test {
    use super::Fp;
    use crate::{arith_test, constants_test, legendre_test, serde_test, test, test_uniform_bytes};

    constants_test!(Fp);

    arith_test!(Fp);
    legendre_test!(Fp);
    test!(arith, Fp, sqrt_test, 1000);

    serde_test!(Fp PrimeFieldBits);
    test_uniform_bytes!(Fp, 1000, L 64, L 48);
}
