use crate::arithmetic::{adc, bigint_geq, mac, macx, sbb};
use crate::ff::{Field, FromUniformBytes, PrimeField, WithSmallOrderMulGroup};
use crate::serde::SerdeObject;
use core::convert::TryInto;
use core::fmt;
use core::ops::{Add, Mul, Neg, Sub};
use rand::RngCore;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

// Number of 64 bit limbs to represent the field element
pub(crate) const NUM_BITS: u32 = 256;

// Inverter constant
const BYIL: usize = 6;

// Jabobi constant
const JACOBI_L: usize = 5;

/// Constant representing the modulus
/// p = 0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f
const MODULUS: Fp = Fp([
    0xfffffffefffffc2f,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
]);

/// Constant representing the multiplicative generator of the modulus.
/// It's derived with SageMath with: `GF(MODULUS).primitive_element()`.
const MULTIPLICATIVE_GENERATOR: Fp = Fp::from_raw([0x03, 0x00, 0x00, 0x00]);

/// Size of the 2-adic sub-group of the field.
const S: u32 = 1;

/// The modulus as u32 limbs.
#[cfg(not(target_pointer_width = "64"))]
const MODULUS_LIMBS_32: [u32; 8] = [
    0xffff_fc2f,
    0xffff_fffe,
    0xffff_ffff,
    0xffff_ffff,
    0xffff_ffff,
    0xffff_ffff,
    0xffff_ffff,
    0xffff_ffff,
];

/// Constant representing the modulus as static str
const MODULUS_STR: &str = "0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f";

/// INV = -(p^{-1} mod 2^64) mod 2^64
const INV: u64 = 0xd838091dd2253531;

/// R = 2^256 mod p
/// 0x1000003d1
const R: Fp = Fp([0x1000003d1, 0, 0, 0]);

/// R^2 = 2^512 mod p
/// 0x1000007a2000e90a1
const R2: Fp = Fp([0x000007a2000e90a1, 0x1, 0, 0]);

/// R^3 = 2^768 mod p
/// 0x100000b73002bb1e33795f671
const R3: Fp = Fp([0x002bb1e33795f671, 0x100000b73, 0, 0]);

/// 1 / 2 mod p
const TWO_INV: Fp = Fp::from_raw([
    0xffffffff7ffffe18,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0x7fffffffffffffff,
]);

const ZETA: Fp = Fp::from_raw([
    0xc1396c28719501ee,
    0x9cf0497512f58995,
    0x6e64479eac3434e9,
    0x7ae96a2b657c0710,
]);

/// Generator of the t-order multiplicative subgroup.
/// Computed by exponentiating Self::MULTIPLICATIVE_GENERATOR by 2^s, where s is Self::S.
/// `0x0000000000000000000000000000000000000000000000000000000000000009`.
const DELTA: Fp = Fp([0x900002259u64, 0, 0, 0]);

/// Implementations of this trait MUST ensure that this is the generator used to derive Self::ROOT_OF_UNITY.
/// Derived from:
/// ```ignore
/// Zp(Zp(mul_generator)^t) where t = (modulus - 1 )/ 2
/// 115792089237316195423570985008687907853269984665640564039457584007908834671662
/// ```
const ROOT_OF_UNITY: Fp = Fp([
    0xfffffffdfffff85eu64,
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
]);

/// Inverse of [`ROOT_OF_UNITY`].
const ROOT_OF_UNITY_INV: Fp = Fp([
    0xfffffffdfffff85eu64,
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
]);

use crate::{
    const_montgomery_4, extend_field_legendre, field_arithmetic_4, field_bits, field_specific_4,
    impl_add_binop_specify_impl, impl_add_binop_specify_output, impl_binops_additive,
    impl_binops_additive_specify_output, impl_binops_multiplicative,
    impl_binops_multiplicative_mixed, impl_field, impl_from_u64, impl_from_uniform_bytes,
    impl_prime_field, impl_serde_object, impl_sub_binop_specify_output, impl_sum_prod, pow_vartime,
};
impl_binops_additive!(Fp, Fp);
impl_binops_multiplicative!(Fp, Fp);
impl_add_binop_specify_impl!(Fp);
impl_field!(Fp, dense);
impl_serde_object!(Fp);
impl_prime_field!(Fp, [u8; 32], le);
impl_sum_prod!(Fp);
extend_field_legendre!(Fp);
impl_from_uniform_bytes!(Fp, 64);
impl_from_uniform_bytes!(Fp, 48);
impl_from_u64!(Fp);

const_montgomery_4!(Fp);
field_arithmetic_4!(Fp, dense);

#[cfg(target_pointer_width = "64")]
field_bits!(Fp);
#[cfg(not(target_pointer_width = "64"))]
field_bits!(Fp);

#[cfg(feature = "derive_serde")]
crate::serialize_deserialize_primefield!(Fp, [u8; 32]);

impl Fp {
    /// Computes the square root of this element, if it exists.
    fn sqrt(&self) -> CtOption<Self> {
        let tmp = self.pow([
            0xffffffffbfffff0c,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0x3fffffffffffffff,
        ]);

        CtOption::new(tmp, tmp.square().ct_eq(self))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    crate::field_testing_suite!(Fp, "field_arithmetic");
    crate::field_testing_suite!(Fp, "conversion");
    crate::field_testing_suite!(Fp, "serialization");
    crate::field_testing_suite!(Fp, "quadratic_residue");
    crate::field_testing_suite!(Fp, "bits");
    crate::field_testing_suite!(Fp, "serialization_check");
    crate::field_testing_suite!(Fp, "constants", MODULUS_STR);
    crate::field_testing_suite!(Fp, "sqrt");
    crate::field_testing_suite!(Fp, "zeta");
    crate::field_testing_suite!(Fp, "from_uniform_bytes", 48, 64);
}
