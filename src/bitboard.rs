use ext_php_rs::prelude::*;
use shakmaty::Bitboard;

/// PHP class: `shakmaty\Bitboard`
///
/// A set of squares represented by a 64-bit integer. Supports common
/// set operations (AND, OR, XOR, NOT) and chess-specific queries.
///
/// # Constants
///
/// - `Bitboard::EMPTY` — empty set
/// - `Bitboard::ALL` — all 64 squares
/// - `Bitboard::CORNERS` — the four corner squares
/// - `Bitboard::BACKRANKS` — first and eighth ranks
/// - `Bitboard::LIGHT_SQUARES` — all light-colored squares
/// - `Bitboard::DARK_SQUARES` — all dark-colored squares
///
/// # Usage
///
/// ```php
/// $bb = \shakmaty\Bitboard::fromSquare(\shakmaty\Square::E4);
/// echo $bb->count(); // 1
/// var_dump($bb->has(28)); // true (e4)
/// ```
///
/// # See Also
///
/// Original Rust type: [`shakmaty::Bitboard`](https://docs.rs/shakmaty/0.30/shakmaty/struct.Bitboard.html)
#[php_class]
#[php(name = "shakmaty\\Bitboard")]
pub struct PhpBitboard {
    pub inner: Bitboard,
}

#[php_impl]
impl PhpBitboard {
    /// Creates a new `Bitboard` from a raw 64-bit value.
    ///
    /// Pass `null` or 0 for an empty bitboard.
    #[php(constructor)]
    pub fn new(raw_value: Option<i64>) -> Self {
        let u_val = raw_value.unwrap_or(0) as u64;
        PhpBitboard { inner: Bitboard(u_val) }
    }

    /// Creates a `Bitboard` containing only the given square.
    ///
    /// Accepts a zero-based square index (A1=0 .. H8=63); an out-of-range value yields an empty bitboard.
    pub fn from_square(sq: i32) -> Self {
        if let Ok(s) = shakmaty::Square::try_from(sq as u8) {
            PhpBitboard { inner: Bitboard::from_square(s) }
        } else {
            PhpBitboard { inner: Bitboard(0) }
        }
    }

    /// Creates a `Bitboard` containing all squares on the given rank.
    pub fn from_rank(rank: i32) -> Self {
        if let Ok(r) = shakmaty::Rank::try_from(rank as u8) {
            PhpBitboard { inner: Bitboard::from_rank(r) }
        } else {
            PhpBitboard { inner: Bitboard(0) }
        }
    }

    /// Creates a `Bitboard` containing all squares on the given file.
    pub fn from_file(file: i32) -> Self {
        if let Ok(f) = shakmaty::File::try_from(file as u8) {
            PhpBitboard { inner: Bitboard::from_file(f) }
        } else {
            PhpBitboard { inner: Bitboard(0) }
        }
    }

    /// Returns the number of squares set in this bitboard.
    pub fn count(&self) -> i32 {
        self.inner.count() as i32
    }

    /// Returns `true` if at least one square is set (the board is non-empty).
    pub fn any(&self) -> bool {
        self.inner.any()
    }

    /// Returns `true` if no squares are set.
    pub fn is_empty(&self) -> bool {
        !self.inner.any()
    }

    /// Returns whether the given square index (A1=0 .. H8=63) belongs to this bitboard.
    ///
    /// Out-of-range indices always return `false`.
    pub fn has(&self, square_index: i32) -> bool {
        if square_index < 0 || square_index > 63 {
            return false;
        }
        if let Ok(sq) = shakmaty::Square::try_from(square_index as u8) {
            return self.inner.contains(sq);
        }
        false
    }

    /// Returns the underlying raw 64-bit integer value of this bitboard.
    pub fn to_u64(&self) -> i64 {
        self.inner.0 as i64
    }

    /// Returns the board as a 64-character binary string, one character per square (LSB first).
    ///
    /// Implements PHP's magic `__toString()`, so instances can be cast directly: `(string) $bb`.
    pub fn __to_string(&self) -> String {
        format!("{:064b}", self.inner.0)
    }

    /// Returns the intersection of two bitboards (`AND`).
    pub fn bitwise_and(&self, other: &PhpBitboard) -> PhpBitboard {
        PhpBitboard { inner: self.inner & other.inner }
    }

    /// Returns the union of two bitboards (`OR`).
    pub fn bitwise_or(&self, other: &PhpBitboard) -> PhpBitboard {
        PhpBitboard { inner: self.inner | other.inner }
    }

    /// Returns the symmetric difference of two bitboards (`XOR`) — squares set in exactly one.
    pub fn bitwise_xor(&self, other: &PhpBitboard) -> PhpBitboard {
        PhpBitboard { inner: self.inner ^ other.inner }
    }

    /// Returns the complement of this bitboard within all 64 squares (`NOT`).
    pub fn bitwise_not(&self) -> PhpBitboard {
        PhpBitboard { inner: !self.inner }
    }

    /// Shifts every square in this board by `offset` positions (positive = toward rank 8, negative = toward rank 1).
    pub fn shift(&self, offset: i32) -> PhpBitboard {
        PhpBitboard { inner: self.inner.shift(offset) }
    }

    // Constants — as i64 because PHP uses signed 64-bit integers
    pub const EMPTY: i64 = 0;
    pub const ALL: i64 = -1; // u64::MAX as signed = -1
    pub const CORNERS: i64 = 0x8100_0000_0000_0081u64 as i64;
    pub const BACKRANKS: i64 = 0xFF00_0000_0000_00FFu64 as i64;
    pub const LIGHT_SQUARES: i64 = 0x55AA_55AA_55AA_55AAi64;
    pub const DARK_SQUARES: i64 = 0xAA55_AA55_AA55_AA55u64 as i64;
}
