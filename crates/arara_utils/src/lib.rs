pub use ahash::AHasher;
use ahash::RandomState;
use std::{future::Future, pin::Pin, ops::Neg, cmp::Ordering};
pub use uuid::Uuid;

pub type BoxedFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// A hasher builder that will create a fixed hasher.
#[derive(Default)]
pub struct FixedState;

impl std::hash::BuildHasher for FixedState {
    type Hasher = AHasher;

    #[inline]
    fn build_hasher(&self) -> AHasher {
        AHasher::new_with_keys(
            0b1001010111101110000001001100010000000011001001101011001001111000,
            0b1100111101101011011110001011010100000100001111100011010011010101,
        )
    }
}

/// A std hash map implementing AHash, a high speed keyed hashing algorithm
/// intended for use in in-memory hashmaps.
///
/// AHash is designed for performance and is NOT cryptographically secure.
pub type HashMap<K, V> = std::collections::HashMap<K, V, RandomState>;

/// A stable std hash map implementing AHash, a high speed keyed hashing algorithm
/// intended for use in in-memory hashmaps.
///
/// Unlike [`HashMap`] this has an iteration order that only depends on the order
/// of insertions and deletions and not a random source.
///
/// AHash is designed for performance and is NOT cryptographically secure.
pub type StableHashMap<K, V> = std::collections::HashMap<K, V, FixedState>;

/// A std hash set implementing AHash, a high speed keyed hashing algorithm
/// intended for use in in-memory hashmaps.
///
/// AHash is designed for performance and is NOT cryptographically secure.
pub type HashSet<K> = std::collections::HashSet<K, RandomState>;

/// A stable std hash set implementing AHash, a high speed keyed hashing algorithm
/// intended for use in in-memory hashmaps.
///
/// Unlike [`HashSet`] this has an iteration order that only depends on the order
/// of insertions and deletions and not a random source.
///
/// AHash is designed for performance and is NOT cryptographically secure.
pub type StableHashSet<K> = std::collections::HashSet<K, FixedState>;

/// A wrapper type that enables ordering floats. This is a work around for the famous "rust float
/// ordering" problem. By using it, you acknowledge that sorting NaN is undefined according to spec.
/// This implementation treats NaN as the "smallest" float.
#[derive(Debug, Copy, Clone, PartialOrd)]
pub struct FloatOrd(pub f32);

#[allow(clippy::derive_ord_xor_partial_ord)]
impl Ord for FloatOrd {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or_else(|| {
            if self.0.is_nan() && !other.0.is_nan() {
                Ordering::Less
            } else if !self.0.is_nan() && other.0.is_nan() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        })
    }
}

impl Eq for FloatOrd {}

impl PartialEq for FloatOrd {
    fn eq(&self, other: &Self) -> bool {
        if self.0.is_nan() && other.0.is_nan() {
            true
        } else {
            self.0 == other.0
        }
    }
}


impl Neg for FloatOrd {
    type Output = FloatOrd;

    fn neg(self) -> Self::Output {
        FloatOrd(-self.0)
    }
}
