use ff::{Field, FromUniformBytes, PrimeField};
use rand::RngCore;

use crate::serde::SerdeObject;

// Tests to_repr/ from_repr
pub(crate) fn from_to_repr_test<F: PrimeField>(mut rng: impl RngCore, n: usize) {
    // n = 1M
    for _ in 0..n {
        let a = F::random(&mut rng);
        let bytes = a.to_repr();
        let b = F::from_repr(bytes).unwrap();
        assert_eq!(a, b);
    }
}

// Tests to_raw_bytes / from_raw_bytes + read_raw /write_raw
pub(crate) fn from_to_raw_bytes_test<F: Field + SerdeObject>(mut rng: impl RngCore, n: usize) {
    for _ in 0..n {
        let a = F::random(&mut rng);
        let bytes = a.to_raw_bytes();
        let b = F::from_raw_bytes(&bytes).unwrap();
        assert_eq!(a, b);

        let mut buf = Vec::new();
        a.write_raw(&mut buf).unwrap();
        let b = F::read_raw(&mut &buf[..]).unwrap();
        assert_eq!(a, b);
    }
}

// Tests derive_serde
#[cfg(feature = "derive_serde")]
pub(crate) fn derive_serde_test<F>(mut rng: impl RngCore, n: usize)
where
    for<'de> F: Field + serde::Serialize + serde::Deserialize<'de>,
{
    for _ in 0..n {
        // byte serialization
        let a = F::random(&mut rng);
        let bytes = bincode::serialize(&a).unwrap();
        let reader = std::io::Cursor::new(bytes);
        let b = bincode::deserialize_from(reader).unwrap();
        assert_eq!(a, b);

        // json serialization
        let json = serde_json::to_string(&a).unwrap();
        let reader = std::io::Cursor::new(json);
        let b: F = serde_json::from_reader(reader).unwrap();
        assert_eq!(a, b);
    }
}

// Test from_raw_bytes / to_raw_bytes
// TODO: Is this test redundant? It uses methods that are not exposed.
// IMO this can be removed.
// fn test_serialization_check<F: Field>(mut rng: impl RngCore, n: usize) {
//     use crate::serde::SerdeObject;
//     const LIMBS: usize = F::SIZE / 8;
//     // failure check
//     for _ in 0..n {
//         let rand_word = [(); LIMBS].map(|_| rng.next_u64());
//         let a = F(rand_word);
//         let rand_bytes = a.to_raw_bytes();

//         match F::is_less_than_modulus(&rand_word) {
//             false => {
//                 assert!(F::from_raw_bytes(&rand_bytes).is_none());
//             }
//             _ => {
//                 assert_eq!(F::from_raw_bytes(&rand_bytes), Some(a));
//             }
//         }
//     }
// }

#[cfg(feature = "bits")]
pub(crate) fn test_bits<F: ff::PrimeFieldBits>(mut rng: impl RngCore, n: usize) {
    for _ in 0..n {
        let a = F::random(&mut rng);
        let bytes = a.to_repr();
        let bits = a.to_le_bits();
        for idx in 0..bits.len() {
            assert_eq!(bits[idx], ((bytes.as_ref()[idx / 8] >> (idx % 8)) & 1) == 1);
        }
    }
}

#[macro_export]
macro_rules! serde_test {
    ($field:ident) => {
        test!(serde, $field, from_to_repr_test, 100_000);
        test!(serde, $field, from_to_raw_bytes_test, 100_000);
        // test!($field, test_serialization_check, 1_000_000);
        #[cfg(feature = "derive_serde")]
        test!(serde, $field, derive_serde_test, 100_000);
    };

    ($field:ident PrimeFieldBits) => {
        serde_test!($field);
        #[cfg(feature = "bits")]
        test!(serde, $field, test_bits, 100_000);
    };
}

// Out of serde_tests macro, since it needs to be tested for several generic L.
// Tests from_uniform_bytes.
pub(crate) fn from_uniform_bytes_test<F: PrimeField, const L: usize>(
    mut rng: impl RngCore,
    n: usize,
) where
    F: FromUniformBytes<L>,
{
    use num_bigint::BigUint;

    let uniform_bytes = [0u8; L];
    assert_eq!(F::from_uniform_bytes(&uniform_bytes), F::ZERO);

    let mut uniform_bytes = [u8::MAX; L];

    for _ in 0..n {
        let e0 = BigUint::from_bytes_le(&uniform_bytes);
        let e0: F = crate::tests::big_to_fe(&e0);

        let e1 = F::from_uniform_bytes(&uniform_bytes);
        assert_eq!(e0, e1);

        rng.fill_bytes(&mut uniform_bytes[..]);
    }
}

#[macro_export]
macro_rules! test_uniform_bytes {
    ($field:ident, $size:expr, L $L: expr ) => {
        paste::paste! {
        #[test]
        fn [< from_uniform_bytes_test_ $L>]() {
            use rand::SeedableRng;
            use rand_xorshift::XorShiftRng;
            let mut rng = XorShiftRng::from_seed($crate::tests::SEED);
            $crate::tests::field::serde::from_uniform_bytes_test::<$field, $L>(&mut rng, $size);
        }

        }
    };

    ($field:ident,$size:expr, L $L:expr, $(L $rest:expr),+ ) => {
        test_uniform_bytes!( $field, $size, L $L );
        test_uniform_bytes! { $field, $size, $(L $rest),+ }
    };
}
