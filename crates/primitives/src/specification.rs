#![allow(non_camel_case_types)]

pub use SpecId::*;

/// Specification IDs and their activation block.
///
/// Information was obtained from the [Ethereum Execution Specifications](https://github.com/ethereum/execution-specs)
#[cfg(not(any(feature = "optimism", feature = "scroll")))]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, enumn::N)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpecId {
    FRONTIER = 0,         // Frontier               0
    FRONTIER_THAWING = 1, // Frontier Thawing       200000
    HOMESTEAD = 2,        // Homestead              1150000
    DAO_FORK = 3,         // DAO Fork               1920000
    TANGERINE = 4,        // Tangerine Whistle      2463000
    SPURIOUS_DRAGON = 5,  // Spurious Dragon        2675000
    BYZANTIUM = 6,        // Byzantium              4370000
    CONSTANTINOPLE = 7,   // Constantinople         7280000 is overwritten with PETERSBURG
    PETERSBURG = 8,       // Petersburg             7280000
    ISTANBUL = 9,         // Istanbul	            9069000
    MUIR_GLACIER = 10,    // Muir Glacier           9200000
    BERLIN = 11,          // Berlin	                12244000
    LONDON = 12,          // London	                12965000
    ARROW_GLACIER = 13,   // Arrow Glacier          13773000
    GRAY_GLACIER = 14,    // Gray Glacier           15050000
    MERGE = 15,           // Paris/Merge            15537394 (TTD: 58750000000000000000000)
    SHANGHAI = 16,        // Shanghai               17034870 (Timestamp: 1681338455)
    CANCUN = 17,          // Cancun                 19426587 (Timestamp: 1710338135)
    PRAGUE = 18,          // Praque                 TBD
    PRAGUE_EOF = 19,      // Praque+EOF             TBD
    #[default]
    LATEST = u8::MAX,
}

/// Specification IDs and their activation block.
///
/// Information was obtained from the [Ethereum Execution Specifications](https://github.com/ethereum/execution-specs)
#[cfg(feature = "optimism")]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, enumn::N)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpecId {
    FRONTIER = 0,
    FRONTIER_THAWING = 1,
    HOMESTEAD = 2,
    DAO_FORK = 3,
    TANGERINE = 4,
    SPURIOUS_DRAGON = 5,
    BYZANTIUM = 6,
    CONSTANTINOPLE = 7,
    PETERSBURG = 8,
    ISTANBUL = 9,
    MUIR_GLACIER = 10,
    BERLIN = 11,
    LONDON = 12,
    ARROW_GLACIER = 13,
    GRAY_GLACIER = 14,
    MERGE = 15,
    BEDROCK = 16,
    REGOLITH = 17,
    SHANGHAI = 18,
    CANYON = 19,
    CANCUN = 20,
    ECOTONE = 21,
    FJORD = 22,
    PRAGUE = 23,
    PRAGUE_EOF = 24,
    #[default]
    LATEST = u8::MAX,
}

/// Specification IDs and their activation block.
///
/// Information was obtained from the [Ethereum Execution Specifications](https://github.com/ethereum/execution-specs)
#[cfg(feature = "scroll")]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, enumn::N)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpecId {
    FRONTIER = 0,
    FRONTIER_THAWING = 1,
    HOMESTEAD = 2,
    DAO_FORK = 3,
    TANGERINE = 4,
    SPURIOUS_DRAGON = 5,
    BYZANTIUM = 6,
    CONSTANTINOPLE = 7,
    PETERSBURG = 8,
    ISTANBUL = 9,
    MUIR_GLACIER = 10,
    BERLIN = 11,
    LONDON = 12,
    ARROW_GLACIER = 13,
    GRAY_GLACIER = 14,
    MERGE = 15,
    SHANGHAI = 16,
    /// The scroll network initially started with Shanghai with some features disabled.
    PRE_BERNOULLI = 17,
    /// Bernoulli update introduces:
    ///   - Enable `SHA-256` precompile.
    ///   - Use `EIP-4844` blobs for Data Availability (not part of layer2).
    BERNOULLI = 18,
    /// Curie update introduces:
    ///   - Support `EIP-1559` transactions.
    ///   - Support the `BASEFEE`, `MCOPY`, `TLOAD`, `TSTORE` opcodes.
    /// Although the Curie update include new opcodes in Cancun, the most important change
    /// `EIP-4844` is not included. So we sort it before Cancun.
    CURIE = 19,
    CANCUN = 20,
    PRAGUE = 21,
    PRAGUE_EOF = 22,
    #[default]
    LATEST = u8::MAX,
}

impl SpecId {
    /// Returns the `SpecId` for the given `u8`.
    #[inline]
    pub fn try_from_u8(spec_id: u8) -> Option<Self> {
        Self::n(spec_id)
    }

    /// Returns `true` if the given specification ID is enabled in this spec.
    #[inline]
    pub const fn is_enabled_in(self, other: Self) -> bool {
        Self::enabled(self, other)
    }

    /// Returns `true` if the given specification ID is enabled in this spec.
    #[inline]
    pub const fn enabled(our: SpecId, other: SpecId) -> bool {
        our as u8 >= other as u8
    }
}

impl From<&str> for SpecId {
    fn from(name: &str) -> Self {
        match name {
            "Frontier" => Self::FRONTIER,
            "Homestead" => Self::HOMESTEAD,
            "Tangerine" => Self::TANGERINE,
            "Spurious" => Self::SPURIOUS_DRAGON,
            "Byzantium" => Self::BYZANTIUM,
            "Constantinople" => Self::CONSTANTINOPLE,
            "Petersburg" => Self::PETERSBURG,
            "Istanbul" => Self::ISTANBUL,
            "MuirGlacier" => Self::MUIR_GLACIER,
            "Berlin" => Self::BERLIN,
            "London" => Self::LONDON,
            "Merge" => Self::MERGE,
            "Shanghai" => Self::SHANGHAI,
            "Cancun" => Self::CANCUN,
            "Prague" => Self::PRAGUE,
            "PragueEOF" => Self::PRAGUE_EOF,
            #[cfg(feature = "optimism")]
            "Bedrock" => SpecId::BEDROCK,
            #[cfg(feature = "optimism")]
            "Regolith" => SpecId::REGOLITH,
            #[cfg(feature = "optimism")]
            "Canyon" => SpecId::CANYON,
            #[cfg(feature = "optimism")]
            "Ecotone" => SpecId::ECOTONE,
            #[cfg(feature = "optimism")]
            "Fjord" => SpecId::FJORD,
            #[cfg(feature = "scroll")]
            "PreBernoulli" => SpecId::PRE_BERNOULLI,
            #[cfg(feature = "scroll")]
            "Bernoulli" => SpecId::BERNOULLI,
            #[cfg(feature = "scroll")]
            "Curie" => SpecId::CURIE,
            _ => Self::LATEST,
        }
    }
}

impl From<SpecId> for &'static str {
    fn from(spec_id: SpecId) -> Self {
        match spec_id {
            SpecId::FRONTIER => "Frontier",
            SpecId::FRONTIER_THAWING => "Frontier Thawing",
            SpecId::HOMESTEAD => "Homestead",
            SpecId::DAO_FORK => "DAO Fork",
            SpecId::TANGERINE => "Tangerine",
            SpecId::SPURIOUS_DRAGON => "Spurious",
            SpecId::BYZANTIUM => "Byzantium",
            SpecId::CONSTANTINOPLE => "Constantinople",
            SpecId::PETERSBURG => "Petersburg",
            SpecId::ISTANBUL => "Istanbul",
            SpecId::MUIR_GLACIER => "MuirGlacier",
            SpecId::BERLIN => "Berlin",
            SpecId::LONDON => "London",
            SpecId::ARROW_GLACIER => "Arrow Glacier",
            SpecId::GRAY_GLACIER => "Gray Glacier",
            SpecId::MERGE => "Merge",
            SpecId::SHANGHAI => "Shanghai",
            SpecId::CANCUN => "Cancun",
            SpecId::PRAGUE => "Prague",
            SpecId::PRAGUE_EOF => "PragueEOF",
            #[cfg(feature = "optimism")]
            SpecId::BEDROCK => "Bedrock",
            #[cfg(feature = "optimism")]
            SpecId::REGOLITH => "Regolith",
            #[cfg(feature = "optimism")]
            SpecId::CANYON => "Canyon",
            #[cfg(feature = "optimism")]
            SpecId::ECOTONE => "Ecotone",
            #[cfg(feature = "optimism")]
            SpecId::FJORD => "Fjord",
            #[cfg(feature = "scroll")]
            SpecId::PRE_BERNOULLI => "PreBernoulli",
            #[cfg(feature = "scroll")]
            SpecId::BERNOULLI => "Bernoulli",
            #[cfg(feature = "scroll")]
            SpecId::CURIE => "Curie",
            SpecId::LATEST => "Latest",
        }
    }
}

pub trait Spec: Sized + 'static {
    /// The specification ID.
    const SPEC_ID: SpecId;

    /// Returns `true` if the given specification ID is enabled in this spec.
    #[inline]
    fn enabled(spec_id: SpecId) -> bool {
        SpecId::enabled(Self::SPEC_ID, spec_id)
    }
}

macro_rules! spec {
    ($spec_id:ident, $spec_name:ident) => {
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $spec_name;

        impl Spec for $spec_name {
            const SPEC_ID: SpecId = $spec_id;
        }
    };
}

spec!(FRONTIER, FrontierSpec);
// FRONTIER_THAWING no EVM spec change
spec!(HOMESTEAD, HomesteadSpec);
// DAO_FORK no EVM spec change
spec!(TANGERINE, TangerineSpec);
spec!(SPURIOUS_DRAGON, SpuriousDragonSpec);
spec!(BYZANTIUM, ByzantiumSpec);
// CONSTANTINOPLE was overridden with PETERSBURG
spec!(PETERSBURG, PetersburgSpec);
spec!(ISTANBUL, IstanbulSpec);
// MUIR_GLACIER no EVM spec change
spec!(BERLIN, BerlinSpec);
spec!(LONDON, LondonSpec);
// ARROW_GLACIER no EVM spec change
// GRAY_GLACIER no EVM spec change
spec!(MERGE, MergeSpec);
spec!(SHANGHAI, ShanghaiSpec);
spec!(CANCUN, CancunSpec);
spec!(PRAGUE, PragueSpec);
spec!(PRAGUE_EOF, PragueEofSpec);

spec!(LATEST, LatestSpec);

// Optimism Hardforks
#[cfg(feature = "optimism")]
spec!(BEDROCK, BedrockSpec);
#[cfg(feature = "optimism")]
spec!(REGOLITH, RegolithSpec);
#[cfg(feature = "optimism")]
spec!(CANYON, CanyonSpec);
#[cfg(feature = "optimism")]
spec!(ECOTONE, EcotoneSpec);
#[cfg(feature = "optimism")]
spec!(FJORD, FjordSpec);

// Scroll Hardforks
#[cfg(feature = "scroll")]
spec!(PRE_BERNOULLI, PreBernoulliSpec);
#[cfg(feature = "scroll")]
spec!(BERNOULLI, BernoulliSpec);
#[cfg(feature = "scroll")]
spec!(CURIE, CurieSpec);

#[cfg(not(any(feature = "optimism", feature = "scroll")))]
#[macro_export]
macro_rules! spec_to_generic {
    ($spec_id:expr, $e:expr) => {{
        match $spec_id {
            $crate::SpecId::FRONTIER | SpecId::FRONTIER_THAWING => {
                use $crate::FrontierSpec as SPEC;
                $e
            }
            $crate::SpecId::HOMESTEAD | SpecId::DAO_FORK => {
                use $crate::HomesteadSpec as SPEC;
                $e
            }
            $crate::SpecId::TANGERINE => {
                use $crate::TangerineSpec as SPEC;
                $e
            }
            $crate::SpecId::SPURIOUS_DRAGON => {
                use $crate::SpuriousDragonSpec as SPEC;
                $e
            }
            $crate::SpecId::BYZANTIUM => {
                use $crate::ByzantiumSpec as SPEC;
                $e
            }
            $crate::SpecId::PETERSBURG | $crate::SpecId::CONSTANTINOPLE => {
                use $crate::PetersburgSpec as SPEC;
                $e
            }
            $crate::SpecId::ISTANBUL | $crate::SpecId::MUIR_GLACIER => {
                use $crate::IstanbulSpec as SPEC;
                $e
            }
            $crate::SpecId::BERLIN => {
                use $crate::BerlinSpec as SPEC;
                $e
            }
            $crate::SpecId::LONDON
            | $crate::SpecId::ARROW_GLACIER
            | $crate::SpecId::GRAY_GLACIER => {
                use $crate::LondonSpec as SPEC;
                $e
            }
            $crate::SpecId::MERGE => {
                use $crate::MergeSpec as SPEC;
                $e
            }
            $crate::SpecId::SHANGHAI => {
                use $crate::ShanghaiSpec as SPEC;
                $e
            }
            $crate::SpecId::CANCUN => {
                use $crate::CancunSpec as SPEC;
                $e
            }
            $crate::SpecId::LATEST => {
                use $crate::LatestSpec as SPEC;
                $e
            }
            $crate::SpecId::PRAGUE => {
                use $crate::PragueSpec as SPEC;
                $e
            }
            $crate::SpecId::PRAGUE_EOF => {
                use $crate::PragueEofSpec as SPEC;
                $e
            }
        }
    }};
}

#[cfg(feature = "optimism")]
#[macro_export]
macro_rules! spec_to_generic {
    ($spec_id:expr, $e:expr) => {{
        match $spec_id {
            $crate::SpecId::FRONTIER | SpecId::FRONTIER_THAWING => {
                use $crate::FrontierSpec as SPEC;
                $e
            }
            $crate::SpecId::HOMESTEAD | SpecId::DAO_FORK => {
                use $crate::HomesteadSpec as SPEC;
                $e
            }
            $crate::SpecId::TANGERINE => {
                use $crate::TangerineSpec as SPEC;
                $e
            }
            $crate::SpecId::SPURIOUS_DRAGON => {
                use $crate::SpuriousDragonSpec as SPEC;
                $e
            }
            $crate::SpecId::BYZANTIUM => {
                use $crate::ByzantiumSpec as SPEC;
                $e
            }
            $crate::SpecId::PETERSBURG | $crate::SpecId::CONSTANTINOPLE => {
                use $crate::PetersburgSpec as SPEC;
                $e
            }
            $crate::SpecId::ISTANBUL | $crate::SpecId::MUIR_GLACIER => {
                use $crate::IstanbulSpec as SPEC;
                $e
            }
            $crate::SpecId::BERLIN => {
                use $crate::BerlinSpec as SPEC;
                $e
            }
            $crate::SpecId::LONDON
            | $crate::SpecId::ARROW_GLACIER
            | $crate::SpecId::GRAY_GLACIER => {
                use $crate::LondonSpec as SPEC;
                $e
            }
            $crate::SpecId::MERGE => {
                use $crate::MergeSpec as SPEC;
                $e
            }
            $crate::SpecId::SHANGHAI => {
                use $crate::ShanghaiSpec as SPEC;
                $e
            }
            $crate::SpecId::CANCUN => {
                use $crate::CancunSpec as SPEC;
                $e
            }
            $crate::SpecId::LATEST => {
                use $crate::LatestSpec as SPEC;
                $e
            }
            $crate::SpecId::PRAGUE => {
                use $crate::PragueSpec as SPEC;
                $e
            }
            $crate::SpecId::PRAGUE_EOF => {
                use $crate::PragueEofSpec as SPEC;
                $e
            }
            $crate::SpecId::BEDROCK => {
                use $crate::BedrockSpec as SPEC;
                $e
            }
            $crate::SpecId::REGOLITH => {
                use $crate::RegolithSpec as SPEC;
                $e
            }
            $crate::SpecId::CANYON => {
                use $crate::CanyonSpec as SPEC;
                $e
            }
            $crate::SpecId::ECOTONE => {
                use $crate::EcotoneSpec as SPEC;
                $e
            }
            $crate::SpecId::FJORD => {
                use $crate::FjordSpec as SPEC;
                $e
            }
        }
    }};
}

#[cfg(feature = "scroll")]
#[macro_export]
macro_rules! spec_to_generic {
    ($spec_id:expr, $e:expr) => {{
        // We are transitioning from var to generic spec.
        match $spec_id {
            $crate::SpecId::FRONTIER | SpecId::FRONTIER_THAWING => {
                use $crate::FrontierSpec as SPEC;
                $e
            }
            $crate::SpecId::HOMESTEAD | SpecId::DAO_FORK => {
                use $crate::HomesteadSpec as SPEC;
                $e
            }
            $crate::SpecId::TANGERINE => {
                use $crate::TangerineSpec as SPEC;
                $e
            }
            $crate::SpecId::SPURIOUS_DRAGON => {
                use $crate::SpuriousDragonSpec as SPEC;
                $e
            }
            $crate::SpecId::BYZANTIUM => {
                use $crate::ByzantiumSpec as SPEC;
                $e
            }
            $crate::SpecId::PETERSBURG | $crate::SpecId::CONSTANTINOPLE => {
                use $crate::PetersburgSpec as SPEC;
                $e
            }
            $crate::SpecId::ISTANBUL | $crate::SpecId::MUIR_GLACIER => {
                use $crate::IstanbulSpec as SPEC;
                $e
            }
            $crate::SpecId::BERLIN => {
                use $crate::BerlinSpec as SPEC;
                $e
            }
            $crate::SpecId::LONDON
            | $crate::SpecId::ARROW_GLACIER
            | $crate::SpecId::GRAY_GLACIER => {
                use $crate::LondonSpec as SPEC;
                $e
            }
            $crate::SpecId::MERGE => {
                use $crate::MergeSpec as SPEC;
                $e
            }
            $crate::SpecId::SHANGHAI => {
                use $crate::ShanghaiSpec as SPEC;
                $e
            }
            $crate::SpecId::CANCUN => {
                use $crate::CancunSpec as SPEC;
                $e
            }
            $crate::SpecId::LATEST => {
                use $crate::LatestSpec as SPEC;
                $e
            }
            $crate::SpecId::PRAGUE => {
                use $crate::PragueSpec as SPEC;
                $e
            }
            $crate::SpecId::PRAGUE_EOF => {
                use $crate::PragueEofSpec as SPEC;
                $e
            }
            $crate::SpecId::PRE_BERNOULLI => {
                use $crate::PreBernoulliSpec as SPEC;
                $e
            }
            $crate::SpecId::BERNOULLI => {
                use $crate::BernoulliSpec as SPEC;
                $e
            }
            $crate::SpecId::CURIE => {
                use $crate::CurieSpec as SPEC;
                $e
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spec_to_generic() {
        use SpecId::*;

        spec_to_generic!(FRONTIER, assert_eq!(SPEC::SPEC_ID, FRONTIER));
        spec_to_generic!(FRONTIER_THAWING, assert_eq!(SPEC::SPEC_ID, FRONTIER));
        spec_to_generic!(HOMESTEAD, assert_eq!(SPEC::SPEC_ID, HOMESTEAD));
        spec_to_generic!(DAO_FORK, assert_eq!(SPEC::SPEC_ID, HOMESTEAD));
        spec_to_generic!(TANGERINE, assert_eq!(SPEC::SPEC_ID, TANGERINE));
        spec_to_generic!(SPURIOUS_DRAGON, assert_eq!(SPEC::SPEC_ID, SPURIOUS_DRAGON));
        spec_to_generic!(BYZANTIUM, assert_eq!(SPEC::SPEC_ID, BYZANTIUM));
        spec_to_generic!(CONSTANTINOPLE, assert_eq!(SPEC::SPEC_ID, PETERSBURG));
        spec_to_generic!(PETERSBURG, assert_eq!(SPEC::SPEC_ID, PETERSBURG));
        spec_to_generic!(ISTANBUL, assert_eq!(SPEC::SPEC_ID, ISTANBUL));
        spec_to_generic!(MUIR_GLACIER, assert_eq!(SPEC::SPEC_ID, ISTANBUL));
        spec_to_generic!(BERLIN, assert_eq!(SPEC::SPEC_ID, BERLIN));
        spec_to_generic!(LONDON, assert_eq!(SPEC::SPEC_ID, LONDON));
        spec_to_generic!(ARROW_GLACIER, assert_eq!(SPEC::SPEC_ID, LONDON));
        spec_to_generic!(GRAY_GLACIER, assert_eq!(SPEC::SPEC_ID, LONDON));
        spec_to_generic!(MERGE, assert_eq!(SPEC::SPEC_ID, MERGE));
        #[cfg(feature = "optimism")]
        spec_to_generic!(BEDROCK, assert_eq!(SPEC::SPEC_ID, BEDROCK));
        #[cfg(feature = "optimism")]
        spec_to_generic!(REGOLITH, assert_eq!(SPEC::SPEC_ID, REGOLITH));
        spec_to_generic!(SHANGHAI, assert_eq!(SPEC::SPEC_ID, SHANGHAI));
        #[cfg(feature = "optimism")]
        spec_to_generic!(CANYON, assert_eq!(SPEC::SPEC_ID, CANYON));
        #[cfg(feature = "scroll")]
        spec_to_generic!(PRE_BERNOULLI, assert_eq!(SPEC::SPEC_ID, PRE_BERNOULLI));
        #[cfg(feature = "scroll")]
        spec_to_generic!(BERNOULLI, assert_eq!(SPEC::SPEC_ID, BERNOULLI));
        #[cfg(feature = "scroll")]
        spec_to_generic!(CURIE, assert_eq!(SPEC::SPEC_ID, CURIE));
        spec_to_generic!(CANCUN, assert_eq!(SPEC::SPEC_ID, CANCUN));
        #[cfg(feature = "optimism")]
        spec_to_generic!(ECOTONE, assert_eq!(SPEC::SPEC_ID, ECOTONE));
        #[cfg(feature = "optimism")]
        spec_to_generic!(FJORD, assert_eq!(SPEC::SPEC_ID, FJORD));
        spec_to_generic!(PRAGUE, assert_eq!(SPEC::SPEC_ID, PRAGUE));
        spec_to_generic!(PRAGUE_EOF, assert_eq!(SPEC::SPEC_ID, PRAGUE_EOF));
        spec_to_generic!(LATEST, assert_eq!(SPEC::SPEC_ID, LATEST));
    }
}

#[cfg(feature = "optimism")]
#[cfg(test)]
mod optimism_tests {
    use super::*;

    #[test]
    fn test_bedrock_post_merge_hardforks() {
        assert!(BedrockSpec::enabled(SpecId::MERGE));
        assert!(!BedrockSpec::enabled(SpecId::SHANGHAI));
        assert!(!BedrockSpec::enabled(SpecId::CANCUN));
        assert!(!BedrockSpec::enabled(SpecId::LATEST));
        assert!(BedrockSpec::enabled(SpecId::BEDROCK));
        assert!(!BedrockSpec::enabled(SpecId::REGOLITH));
    }

    #[test]
    fn test_regolith_post_merge_hardforks() {
        assert!(RegolithSpec::enabled(SpecId::MERGE));
        assert!(!RegolithSpec::enabled(SpecId::SHANGHAI));
        assert!(!RegolithSpec::enabled(SpecId::CANCUN));
        assert!(!RegolithSpec::enabled(SpecId::LATEST));
        assert!(RegolithSpec::enabled(SpecId::BEDROCK));
        assert!(RegolithSpec::enabled(SpecId::REGOLITH));
    }

    #[test]
    fn test_bedrock_post_merge_hardforks_spec_id() {
        assert!(SpecId::enabled(SpecId::BEDROCK, SpecId::MERGE));
        assert!(!SpecId::enabled(SpecId::BEDROCK, SpecId::SHANGHAI));
        assert!(!SpecId::enabled(SpecId::BEDROCK, SpecId::CANCUN));
        assert!(!SpecId::enabled(SpecId::BEDROCK, SpecId::LATEST));
        assert!(SpecId::enabled(SpecId::BEDROCK, SpecId::BEDROCK));
        assert!(!SpecId::enabled(SpecId::BEDROCK, SpecId::REGOLITH));
    }

    #[test]
    fn test_regolith_post_merge_hardforks_spec_id() {
        assert!(SpecId::enabled(SpecId::REGOLITH, SpecId::MERGE));
        assert!(!SpecId::enabled(SpecId::REGOLITH, SpecId::SHANGHAI));
        assert!(!SpecId::enabled(SpecId::REGOLITH, SpecId::CANCUN));
        assert!(!SpecId::enabled(SpecId::REGOLITH, SpecId::LATEST));
        assert!(SpecId::enabled(SpecId::REGOLITH, SpecId::BEDROCK));
        assert!(SpecId::enabled(SpecId::REGOLITH, SpecId::REGOLITH));
    }

    #[test]
    fn test_canyon_post_merge_hardforks() {
        assert!(CanyonSpec::enabled(SpecId::MERGE));
        assert!(CanyonSpec::enabled(SpecId::SHANGHAI));
        assert!(!CanyonSpec::enabled(SpecId::CANCUN));
        assert!(!CanyonSpec::enabled(SpecId::LATEST));
        assert!(CanyonSpec::enabled(SpecId::BEDROCK));
        assert!(CanyonSpec::enabled(SpecId::REGOLITH));
        assert!(CanyonSpec::enabled(SpecId::CANYON));
    }

    #[test]
    fn test_canyon_post_merge_hardforks_spec_id() {
        assert!(SpecId::enabled(SpecId::CANYON, SpecId::MERGE));
        assert!(SpecId::enabled(SpecId::CANYON, SpecId::SHANGHAI));
        assert!(!SpecId::enabled(SpecId::CANYON, SpecId::CANCUN));
        assert!(!SpecId::enabled(SpecId::CANYON, SpecId::LATEST));
        assert!(SpecId::enabled(SpecId::CANYON, SpecId::BEDROCK));
        assert!(SpecId::enabled(SpecId::CANYON, SpecId::REGOLITH));
        assert!(SpecId::enabled(SpecId::CANYON, SpecId::CANYON));
    }

    #[test]
    fn test_ecotone_post_merge_hardforks() {
        assert!(EcotoneSpec::enabled(SpecId::MERGE));
        assert!(EcotoneSpec::enabled(SpecId::SHANGHAI));
        assert!(EcotoneSpec::enabled(SpecId::CANCUN));
        assert!(!EcotoneSpec::enabled(SpecId::LATEST));
        assert!(EcotoneSpec::enabled(SpecId::BEDROCK));
        assert!(EcotoneSpec::enabled(SpecId::REGOLITH));
        assert!(EcotoneSpec::enabled(SpecId::CANYON));
        assert!(EcotoneSpec::enabled(SpecId::ECOTONE));
    }

    #[test]
    fn test_ecotone_post_merge_hardforks_spec_id() {
        assert!(SpecId::enabled(SpecId::ECOTONE, SpecId::MERGE));
        assert!(SpecId::enabled(SpecId::ECOTONE, SpecId::SHANGHAI));
        assert!(SpecId::enabled(SpecId::ECOTONE, SpecId::CANCUN));
        assert!(!SpecId::enabled(SpecId::ECOTONE, SpecId::LATEST));
        assert!(SpecId::enabled(SpecId::ECOTONE, SpecId::BEDROCK));
        assert!(SpecId::enabled(SpecId::ECOTONE, SpecId::REGOLITH));
        assert!(SpecId::enabled(SpecId::ECOTONE, SpecId::CANYON));
        assert!(SpecId::enabled(SpecId::ECOTONE, SpecId::ECOTONE));
    }

    #[test]
    fn test_fjord_post_merge_hardforks() {
        assert!(FjordSpec::enabled(SpecId::MERGE));
        assert!(FjordSpec::enabled(SpecId::SHANGHAI));
        assert!(FjordSpec::enabled(SpecId::CANCUN));
        assert!(!FjordSpec::enabled(SpecId::LATEST));
        assert!(FjordSpec::enabled(SpecId::BEDROCK));
        assert!(FjordSpec::enabled(SpecId::REGOLITH));
        assert!(FjordSpec::enabled(SpecId::CANYON));
        assert!(FjordSpec::enabled(SpecId::ECOTONE));
        assert!(FjordSpec::enabled(SpecId::FJORD));
    }

    #[test]
    fn test_fjord_post_merge_hardforks_spec_id() {
        assert!(SpecId::enabled(SpecId::FJORD, SpecId::MERGE));
        assert!(SpecId::enabled(SpecId::FJORD, SpecId::SHANGHAI));
        assert!(SpecId::enabled(SpecId::FJORD, SpecId::CANCUN));
        assert!(!SpecId::enabled(SpecId::FJORD, SpecId::LATEST));
        assert!(SpecId::enabled(SpecId::FJORD, SpecId::BEDROCK));
        assert!(SpecId::enabled(SpecId::FJORD, SpecId::REGOLITH));
        assert!(SpecId::enabled(SpecId::FJORD, SpecId::CANYON));
        assert!(SpecId::enabled(SpecId::FJORD, SpecId::ECOTONE));
        assert!(SpecId::enabled(SpecId::FJORD, SpecId::FJORD));
    }
}

#[cfg(feature = "scroll")]
#[cfg(test)]
mod scroll_tests {
    use super::*;

    #[test]
    fn test_pre_bernoulli_post_merge_hardforks() {
        assert!(PreBernoulliSpec::enabled(SpecId::MERGE));
        assert!(PreBernoulliSpec::enabled(SpecId::SHANGHAI));
        assert!(!PreBernoulliSpec::enabled(SpecId::BERNOULLI));
        assert!(!PreBernoulliSpec::enabled(SpecId::CURIE));
        assert!(!PreBernoulliSpec::enabled(SpecId::CANCUN));
        assert!(!PreBernoulliSpec::enabled(SpecId::LATEST));
    }

    #[test]
    fn test_bernoulli_post_merge_hardforks() {
        assert!(BernoulliSpec::enabled(SpecId::MERGE));
        assert!(BernoulliSpec::enabled(SpecId::SHANGHAI));
        assert!(BernoulliSpec::enabled(SpecId::PRE_BERNOULLI));
        assert!(!BernoulliSpec::enabled(SpecId::CURIE));
        assert!(!BernoulliSpec::enabled(SpecId::CANCUN));
        assert!(!BernoulliSpec::enabled(SpecId::LATEST));
    }

    #[test]
    fn test_curie_post_merge_hardforks() {
        assert!(CurieSpec::enabled(SpecId::MERGE));
        assert!(CurieSpec::enabled(SpecId::SHANGHAI));
        assert!(CurieSpec::enabled(SpecId::PRE_BERNOULLI));
        assert!(CurieSpec::enabled(SpecId::BERNOULLI));
        assert!(!CurieSpec::enabled(SpecId::CANCUN));
        assert!(!CurieSpec::enabled(SpecId::LATEST));
    }
}
