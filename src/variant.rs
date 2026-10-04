use ext_php_rs::prelude::*;
use shakmaty::variant::Variant;

/// Maps an integer discriminant to a shakmaty `Variant`.
///
/// 0 = Chess (default), 1 = Atomic, 2 = Antichess, 3 = KingOfTheHill,
/// 4 = ThreeCheck, 5 = Crazyhouse, 6 = RacingKings, 7 = Horde.
/// Out-of-range values fall back to Chess.
pub(crate) fn variant_from_int(value: i32) -> Variant {
    match value {
        1 => Variant::Atomic,
        2 => Variant::Antichess,
        3 => Variant::KingOfTheHill,
        4 => Variant::ThreeCheck,
        5 => Variant::Crazyhouse,
        6 => Variant::RacingKings,
        7 => Variant::Horde,
        _ => Variant::Chess,
    }
}

/// Maps a shakmaty `Variant` to its integer discriminant (see [`variant_from_int`]).
pub(crate) fn variant_to_int(value: Variant) -> i32 {
    match value {
        Variant::Chess => 0,
        Variant::Atomic => 1,
        Variant::Antichess => 2,
        Variant::KingOfTheHill => 3,
        Variant::ThreeCheck => 4,
        Variant::Crazyhouse => 5,
        Variant::RacingKings => 6,
        Variant::Horde => 7,
    }
}

/// PHP class: `shakmaty\Variant`
///
/// Discriminant of a dynamically dispatched chess variant position. Mirrors
/// the Rust [`shakmaty::variant::Variant`] enum.
///
/// # Constants
///
/// - `Variant::CHESS` = 0
/// - `Variant::ATOMIC` = 1
/// - `Variant::ANTICHESS` = 2
/// - `Variant::KING_OF_THE_HILL` = 3
/// - `Variant::THREE_CHECK` = 4
/// - `Variant::CRAZYHOUSE` = 5
/// - `Variant::RACING_KINGS` = 6
/// - `Variant::HORDE` = 7
///
/// # Usage
///
/// ```php
/// $v = new \shakmaty\Variant(\shakmaty\Variant::ATOMIC);
/// echo $v->uci();                       // "atomic"
/// $v2 = \shakmaty\Variant::fromUci("3check");
/// echo $v2->value();                    // 4
/// ```
///
/// # See Also
///
/// Original Rust type: [`shakmaty::variant::Variant`](https://docs.rs/shakmaty/0.30/shakmaty/variant/enum.Variant.html)
#[php_class]
#[php(name = "shakmaty\\Variant")]
pub struct PhpVariant {
    pub inner: Variant,
}

#[php_impl]
impl PhpVariant {
    /// Creates a `Variant`.
    ///
    /// Accepts values `0..=7`; out-of-range values fall back to `Chess` (0).
    #[php(constructor)]
    pub fn new(value: i32) -> Self {
        PhpVariant { inner: variant_from_int(value) }
    }

    /// Returns the numeric discriminant (0 = Chess … 7 = Horde).
    pub fn value(&self) -> i32 {
        variant_to_int(self.inner)
    }

    /// Returns the UCI name used by the engine option `UCI_Variant`
    /// (`"chess"`, `"atomic"`, `"antichess"`, `"kingofthehill"`,
    /// `"3check"`, `"crazyhouse"`, `"racingkings"`, `"horde"`).
    pub fn uci(&self) -> String {
        self.inner.uci().to_string()
    }

    /// Alias of [`uci()`](Self::uci) — returns the canonical variant name.
    pub fn __to_string(&self) -> String {
        self.inner.uci().to_string()
    }

    /// Selects a variant by its exact UCI name. Returns `null` for unknown names.
    pub fn from_uci(s: String) -> Option<Self> {
        Variant::from_uci(&s).ok().map(|v| PhpVariant { inner: v })
    }

    /// Selects a variant by its name or a known alias (e.g. `"Chess960"`,
    /// `"King of the Hill"`, `"3check"`). Returns `null` for unknown names.
    pub fn from_ascii(s: String) -> Option<Self> {
        Variant::from_ascii(s.as_bytes()).ok().map(|v| PhpVariant { inner: v })
    }

    /// Whether the variant tracks promoted pieces (only `Crazyhouse`).
    pub fn distinguishes_promoted(&self) -> bool {
        self.inner.distinguishes_promoted()
    }

    /// Returns all eight variants as an array.
    pub fn all() -> Vec<PhpVariant> {
        Variant::ALL.iter().map(|v| PhpVariant { inner: *v }).collect()
    }

    pub const CHESS: i32 = 0;
    pub const ATOMIC: i32 = 1;
    pub const ANTICHESS: i32 = 2;
    pub const KING_OF_THE_HILL: i32 = 3;
    pub const THREE_CHECK: i32 = 4;
    pub const CRAZYHOUSE: i32 = 5;
    pub const RACING_KINGS: i32 = 6;
    pub const HORDE: i32 = 7;
}
