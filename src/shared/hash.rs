use std::{
    collections::HashSet,
    hash::{BuildHasher, Hash, Hasher},
    ops::BitXor,
};

/// Type alias for [`HashSet`] using [`FxHasher`].
pub type FastSet<T> = HashSet<T, BuildFxHasher>;

/// Convenience methods to contruct a [`FastSet`].
pub trait FastSetBuilder<T> {
    fn with_capacity(capacity: usize) -> Self;
}

impl<T: Eq + Hash> FastSetBuilder<T> for FastSet<T> {
    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, BuildFxHasher)
    }
}

/// If you want an instance of [`FxHasher`] then this has you covered.
#[derive(Clone, Copy, Default)]
pub struct BuildFxHasher;

impl BuildHasher for BuildFxHasher {
    type Hasher = FxHasher;

    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        FxHasher { hash: 0 }
    }
}

/// Simplified implementation, in particular running on a system with 64 bit `usize` is assumed.
///
/// Checkout the [Firefox code](https://searchfox.org/mozilla-central/rev/633345116df55e2d37be9be6555aa739656c5a7d/mfbt/HashFunctions.h#109-153)
/// for a full description.
const K: u64 = 0x517c_c1b7_2722_0a95;

pub struct FxHasher {
    hash: u64,
}

impl FxHasher {
    #[inline]
    fn add(&mut self, i: u64) {
        self.hash = self.hash.rotate_left(5).bitxor(i).wrapping_mul(K);
    }
}

impl Hasher for FxHasher {
    #[inline]
    fn write(&mut self, mut bytes: &[u8]) {
        while bytes.len() >= 8 {
            self.add(u64::from_ne_bytes(bytes[..8].try_into().unwrap()));
            bytes = &bytes[8..];
        }
        if bytes.len() >= 4 {
            self.add(u64::from(u32::from_ne_bytes(
                bytes[..4].try_into().unwrap(),
            )));
            bytes = &bytes[4..];
        }
        if bytes.len() >= 2 {
            self.add(u64::from(u16::from_ne_bytes(
                bytes[..2].try_into().unwrap(),
            )));
            bytes = &bytes[2..];
        }
        if !bytes.is_empty() {
            self.add(u64::from(bytes[0]));
        }
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.add(u64::from(i));
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.add(u64::from(i));
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.add(u64::from(i));
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.add(i);
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.add(i as u64);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }
}
