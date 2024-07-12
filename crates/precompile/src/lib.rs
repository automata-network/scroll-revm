//! # revm-precompile
//!
//! Implementations of EVM precompiled contracts.
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
#[cfg(not(feature = "std"))]
extern crate alloc as std;

pub mod blake2;
#[cfg(feature = "blst")]
pub mod bls12_381;
pub mod bn128;
pub mod fatal_precompile;
pub mod hash;
pub mod identity;
#[cfg(feature = "c-kzg")]
pub mod kzg_point_evaluation;
pub mod modexp;
pub mod secp256k1;
#[cfg(feature = "secp256r1")]
pub mod secp256r1;
pub mod utilities;

pub use fatal_precompile::fatal_precompile;

pub use primitives::{
    precompile::{PrecompileError as Error, *},
    Address, Bytes, HashMap, HashSet, Log, B256,
};
#[doc(hidden)]
pub use revm_primitives as primitives;

use cfg_if::cfg_if;
use core::hash::Hash;
use once_cell::race::OnceBox;
use std::{boxed::Box, vec::Vec};

pub fn calc_linear_cost_u32(len: usize, base: u64, word: u64) -> u64 {
    (len as u64 + 32 - 1) / 32 * word + base
}

#[derive(Clone, Default, Debug)]
pub struct Precompiles {
    /// Precompiles.
    inner: HashMap<Address, Precompile>,
    /// Addresses of precompile.
    addresses: HashSet<Address>,
}

impl Precompiles {
    /// Returns the precompiles for the given spec.
    pub fn new(spec: PrecompileSpecId) -> &'static Self {
        match spec {
            PrecompileSpecId::HOMESTEAD => Self::homestead(),
            PrecompileSpecId::BYZANTIUM => Self::byzantium(),
            PrecompileSpecId::ISTANBUL => Self::istanbul(),
            PrecompileSpecId::BERLIN => Self::berlin(),
            #[cfg(feature = "scroll")]
            PrecompileSpecId::PRE_BERNOULLI => Self::pre_bernoulli(),
            #[cfg(feature = "scroll")]
            PrecompileSpecId::BERNOULLI => Self::bernoulli(),
            PrecompileSpecId::CANCUN => Self::cancun(),
            PrecompileSpecId::PRAGUE => Self::prague(),
            PrecompileSpecId::LATEST => Self::latest(),
        }
    }

    /// Returns precompiles for Homestead spec.
    pub fn homestead() -> &'static Self {
        static INSTANCE: OnceBox<Precompiles> = OnceBox::new();
        INSTANCE.get_or_init(|| {
            let mut precompiles = Precompiles::default();
            precompiles.extend([
                secp256k1::ECRECOVER,
                hash::SHA256,
                hash::RIPEMD160,
                identity::FUN,
            ]);
            Box::new(precompiles)
        })
    }

    /// Returns inner HashMap of precompiles.
    pub fn inner(&self) -> &HashMap<Address, Precompile> {
        &self.inner
    }

    /// Returns precompiles for Byzantium spec.
    pub fn byzantium() -> &'static Self {
        static INSTANCE: OnceBox<Precompiles> = OnceBox::new();
        INSTANCE.get_or_init(|| {
            let mut precompiles = Self::homestead().clone();
            precompiles.extend([
                // EIP-196: Precompiled contracts for addition and scalar multiplication on the elliptic curve alt_bn128.
                // EIP-197: Precompiled contracts for optimal ate pairing check on the elliptic curve alt_bn128.
                bn128::add::BYZANTIUM,
                bn128::mul::BYZANTIUM,
                bn128::pair::BYZANTIUM,
                // EIP-198: Big integer modular exponentiation.
                modexp::BYZANTIUM,
            ]);
            Box::new(precompiles)
        })
    }

    /// Returns precompiles for Istanbul spec.
    pub fn istanbul() -> &'static Self {
        static INSTANCE: OnceBox<Precompiles> = OnceBox::new();
        INSTANCE.get_or_init(|| {
            let mut precompiles = Self::byzantium().clone();
            precompiles.extend([
                // EIP-1108: Reduce alt_bn128 precompile gas costs.
                bn128::add::ISTANBUL,
                bn128::mul::ISTANBUL,
                bn128::pair::ISTANBUL,
                // EIP-152: Add BLAKE2 compression function `F` precompile.
                blake2::FUN,
            ]);
            Box::new(precompiles)
        })
    }

    /// Returns precompiles for Berlin spec.
    pub fn berlin() -> &'static Self {
        static INSTANCE: OnceBox<Precompiles> = OnceBox::new();
        INSTANCE.get_or_init(|| {
            let mut precompiles = Self::istanbul().clone();
            precompiles.extend([
                // EIP-2565: ModExp Gas Cost.
                modexp::BERLIN,
            ]);
            Box::new(precompiles)
        })
    }

    /// Returns precompiles for Cancun spec.
    ///
    /// If the `c-kzg` feature is not enabled KZG Point Evaluation precompile will not be included,
    /// effectively making this the same as Berlin.
    pub fn cancun() -> &'static Self {
        static INSTANCE: OnceBox<Precompiles> = OnceBox::new();
        INSTANCE.get_or_init(|| {
            let mut precompiles = Self::berlin().clone();

            // EIP-4844: Shard Blob Transactions
            cfg_if! {
                if #[cfg(feature = "c-kzg")] {
                    let precompile = kzg_point_evaluation::POINT_EVALUATION.clone();
                } else {
                    // TODO move constants to separate file.
                    let precompile = fatal_precompile(u64_to_address(0x0A), "c-kzg feature is not enabled".into());
                }
            }

            precompiles.extend([
                precompile,
            ]);

            Box::new(precompiles)
        })
    }

    /// Returns precompiles for Prague spec.
    pub fn prague() -> &'static Self {
        static INSTANCE: OnceBox<Precompiles> = OnceBox::new();
        INSTANCE.get_or_init(|| {
            let precompiles = Self::cancun().clone();

            // Don't include BLS12-381 precompiles in no_std builds.
            #[cfg(feature = "blst")]
            let precompiles = {
                let mut precompiles = precompiles;
                precompiles.extend(bls12_381::precompiles());
                precompiles
            };

            Box::new(precompiles)
        })
    }

    /// Returns precompiles for Scroll
    #[cfg(feature = "scroll")]
    pub fn pre_bernoulli() -> &'static Self {
        static INSTANCE: OnceBox<Precompiles> = OnceBox::new();
        INSTANCE.get_or_init(|| {
            let mut precompiles = Precompiles::default();
            precompiles.extend([
                secp256k1::ECRECOVER,          // 0x01
                hash::SHA256_PRE_BERNOULLI,    // 0x02
                hash::RIPEMD160_PRE_BERNOULLI, // 0x03
                identity::FUN,                 // 0x04
                modexp::BERNOULLI,             // 0x05
                bn128::add::ISTANBUL,          // 0x06
                bn128::mul::ISTANBUL,          // 0x07
                bn128::pair::BERNOULLI,        // 0x08
                blake2::BERNOULLI,             // 0x09
            ]);
            Box::new(precompiles)
        })
    }

    /// Returns precompiles for Scroll
    #[cfg(feature = "scroll")]
    pub fn bernoulli() -> &'static Self {
        static INSTANCE: OnceBox<Precompiles> = OnceBox::new();
        INSTANCE.get_or_init(|| {
            let mut precompiles = Self::pre_bernoulli().clone();
            precompiles.extend([
                hash::SHA256, // 0x02
            ]);
            Box::new(precompiles)
        })
    }

    /// Returns the precompiles for the latest spec.
    pub fn latest() -> &'static Self {
        Self::prague()
    }

    /// Returns an iterator over the precompiles addresses.
    #[inline]
    pub fn addresses(&self) -> impl ExactSizeIterator<Item = &Address> {
        self.inner.keys()
    }

    /// Consumes the type and returns all precompile addresses.
    #[inline]
    pub fn into_addresses(self) -> impl ExactSizeIterator<Item = Address> {
        self.inner.into_keys()
    }

    /// Is the given address a precompile.
    #[inline]
    pub fn contains(&self, address: &Address) -> bool {
        self.inner.contains_key(address)
    }

    /// Returns the precompile for the given address.
    #[inline]
    pub fn get(&self, address: &Address) -> Option<&Precompile> {
        self.inner.get(address)
    }

    /// Returns the precompile for the given address.
    #[inline]
    pub fn get_mut(&mut self, address: &Address) -> Option<&mut Precompile> {
        self.inner.get_mut(address)
    }

    /// Is the precompiles list empty.
    pub fn is_empty(&self) -> bool {
        self.inner.len() == 0
    }

    /// Returns the number of precompiles.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns the precompiles addresses as a set.
    pub fn addresses_set(&self) -> &HashSet<Address> {
        &self.addresses
    }

    /// Extends the precompiles with the given precompiles.
    ///
    /// Other precompiles with overwrite existing precompiles.
    #[inline]
    pub fn extend(&mut self, other: impl IntoIterator<Item = PrecompileWithAddress>) {
        let items = other.into_iter().collect::<Vec<_>>();
        self.addresses.extend(items.iter().map(|p| *p.address()));
        self.inner.extend(items.into_iter().map(Into::into));
    }
}

#[derive(Clone, Debug)]
pub struct PrecompileWithAddress(pub Address, pub Precompile);

impl From<(Address, Precompile)> for PrecompileWithAddress {
    fn from(value: (Address, Precompile)) -> Self {
        PrecompileWithAddress(value.0, value.1)
    }
}

impl From<PrecompileWithAddress> for (Address, Precompile) {
    fn from(value: PrecompileWithAddress) -> Self {
        (value.0, value.1)
    }
}

impl PrecompileWithAddress {
    /// Returns reference of address.
    #[inline]
    pub fn address(&self) -> &Address {
        &self.0
    }

    /// Returns reference of precompile.
    #[inline]
    pub fn precompile(&self) -> &Precompile {
        &self.1
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum PrecompileSpecId {
    HOMESTEAD,
    BYZANTIUM,
    ISTANBUL,
    BERLIN,
    #[cfg(feature = "scroll")]
    PRE_BERNOULLI,
    #[cfg(feature = "scroll")]
    BERNOULLI,
    CANCUN,
    PRAGUE,
    LATEST,
}

impl PrecompileSpecId {
    /// Returns the appropriate precompile Spec for the primitive [SpecId](revm_primitives::SpecId)
    pub const fn from_spec_id(spec_id: revm_primitives::SpecId) -> Self {
        use revm_primitives::SpecId::*;
        match spec_id {
            FRONTIER | FRONTIER_THAWING | HOMESTEAD | DAO_FORK | TANGERINE | SPURIOUS_DRAGON => {
                Self::HOMESTEAD
            }
            BYZANTIUM | CONSTANTINOPLE | PETERSBURG => Self::BYZANTIUM,
            ISTANBUL | MUIR_GLACIER => Self::ISTANBUL,
            BERLIN | LONDON | ARROW_GLACIER | GRAY_GLACIER | MERGE | SHANGHAI => Self::BERLIN,
            CANCUN => Self::CANCUN,
            PRAGUE | PRAGUE_EOF => Self::PRAGUE,
            LATEST => Self::LATEST,
            #[cfg(feature = "optimism")]
            BEDROCK | REGOLITH | CANYON => Self::BERLIN,
            #[cfg(feature = "optimism")]
            ECOTONE | FJORD => Self::CANCUN,
            #[cfg(feature = "scroll")]
            PRE_BERNOULLI => Self::PRE_BERNOULLI,
            #[cfg(feature = "scroll")]
            BERNOULLI | CURIE => Self::BERNOULLI,
        }
    }
}

/// Const function for making an address by concatenating the bytes from two given numbers.
///
/// Note that 32 + 128 = 160 = 20 bytes (the length of an address). This function is used
/// as a convenience for specifying the addresses of the various precompiles.
#[inline]
pub const fn u64_to_address(x: u64) -> Address {
    let x = x.to_be_bytes();
    Address::new([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7],
    ])
}
