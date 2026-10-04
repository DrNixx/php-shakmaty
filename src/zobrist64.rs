use ext_php_rs::prelude::*;
use shakmaty::zobrist::{Zobrist64, ZobristValue};
use shakmaty::{CastlingSide, Color, File, Piece, Role, Square};

/// PHP class: `shakmaty\Zobrist64`
///
/// A 64-bit Zobrist hash value. Mirrors the Rust
/// [`shakmaty::zobrist::Zobrist64`] value type.
///
/// PHP integers are signed 64-bit, so the read-only `$value` property returns the raw bit pattern
/// as a signed integer (hashes with the top bit set become negative). Use
/// `toHex()` / `__toString()` for the exact unsigned value.
///
/// # Usage
///
/// ```php
/// $h = new \shakmaty\Chess()->zobristHash();
/// echo $h->toHex();                    // e.g. "463b96181691fc9c"
/// $same = \shakmaty\Zobrist64::fromHex($h->toHex());
/// var_dump($h->equals($same));         // true
/// $zero = $h->bitwiseXor($h);
/// var_dump($zero->isZero());           // true
/// ```
///
/// # Properties
///
/// - `$hash->value` (`int`, read-only) — raw 64-bit bit pattern as a signed
///   PHP integer (may be negative; use `toHex()` for the unsigned value).
///
/// # See Also
///
/// Original Rust type: [`shakmaty::zobrist::Zobrist64`](https://docs.rs/shakmaty/0.30/shakmaty/zobrist/struct.Zobrist64.html)
#[php_class]
#[php(name = "shakmaty\\Zobrist64")]
pub struct PhpZobrist64 {
    pub inner: Zobrist64,
}

#[php_impl]
impl PhpZobrist64 {
    /// Creates a `Zobrist64` from a raw 64-bit value (two's-complement bit pattern).
    ///
    /// Defaults to `0`.
    #[php(constructor)]
    pub fn new(value: Option<i64>) -> Self {
        PhpZobrist64 {
            inner: Zobrist64(value.unwrap_or(0) as u64),
        }
    }

    /// Raw 64-bit value as a signed PHP integer (may be negative).
    ///
    /// Backs the read-only PHP property `$value`.
    #[php(getter)]
    pub fn get_value(&self) -> i64 {
        self.inner.0 as i64
    }

    /// Legacy 24-bit hash: the top 24 bits of the value, shifted into the low
    /// 24 bits (`(value >>> 40) & 0xFFFFFF`).
    ///
    /// Reproduces the hashes used by the old `ZobristHash.php` engine so newly
    /// computed positions stay compatible with its pre-indexed collections.
    /// The shift is logical (zero-fill): the result is always in
    /// `0 ..= 0xFFFFFF`, even when the top bit of the value is set.
    pub fn legacy24(&self) -> i64 {
        ((self.inner.0 >> 40) & 0xFF_FFFF) as i64
    }

    /// Returns the unsigned value as a 16-digit lowercase hexadecimal string.
    pub fn to_hex(&self) -> String {
        format!("{:016x}", self.inner.0)
    }

    /// Returns the lowercase hexadecimal string representation (alias of `toHex()`).
    pub fn __to_string(&self) -> String {
        format!("{:016x}", self.inner.0)
    }

    /// Creates a `Zobrist64` from a hexadecimal string (an optional leading `0x` is accepted).
    ///
    /// # Errors
    ///
    /// Throws `"Invalid hex"` if the string is not valid hexadecimal.
    pub fn from_hex(hex: String) -> Result<Self, &'static str> {
        let s = hex.trim();
        let s = s
            .strip_prefix("0x")
            .or_else(|| s.strip_prefix("0X"))
            .unwrap_or(s);
        u64::from_str_radix(s, 16)
            .map(|v| PhpZobrist64 { inner: Zobrist64(v) })
            .map_err(|_| "Invalid hex")
    }

    /// Whether the hash value is zero.
    pub fn is_zero(&self) -> bool {
        self.inner.0 == 0
    }

    /// Returns a new `Zobrist64` equal to the bitwise XOR of this hash and `other`.
    pub fn bitwise_xor(&self, other: &PhpZobrist64) -> Self {
        PhpZobrist64 {
            inner: Zobrist64(self.inner.0 ^ other.inner.0),
        }
    }

    /// Whether two hashes are equal.
    pub fn equals(&self, other: &PhpZobrist64) -> bool {
        self.inner == other.inner
    }

    /// Polyglot Zobrist component for a piece: `$sq` (0..63), `$color`
    /// (0=Black, 1=White), `$role` (1=Pawn .. 6=King).
    ///
    /// Out-of-range square or role yields the zero hash (no exception).
    pub fn for_piece(sq: i32, color_value: i32, role_value: i32) -> Self {
        let Ok(square) = Square::try_from(sq as u8) else {
            return PhpZobrist64 { inner: Zobrist64(0) };
        };
        let Ok(role) = Role::try_from(role_value as u8) else {
            return PhpZobrist64 { inner: Zobrist64(0) };
        };
        let color = if color_value == 1 { Color::White } else { Color::Black };
        PhpZobrist64 {
            inner: Zobrist64::zobrist_for_piece(square, Piece { color, role }),
        }
    }

    /// Polyglot Zobrist component for the side to move. Only White's turn
    /// contributes; Black adds nothing (Polyglot `hashTurn`).
    pub fn for_white_turn() -> Self {
        PhpZobrist64 { inner: Zobrist64::zobrist_for_white_turn() }
    }

    /// Polyglot Zobrist component for one castling right.
    ///
    /// `$color` (0=Black, 1=White); `$side` follows shakmaty numbering
    /// (`CastlingSide::KING_SIDE` = 0, `QUEEN_SIDE` = 1). Other values yield zero.
    pub fn for_castling_right(color_value: i32, side_value: i32) -> Self {
        let color = if color_value == 1 { Color::White } else { Color::Black };
        let side = match side_value {
            0 => CastlingSide::KingSide,
            1 => CastlingSide::QueenSide,
            _ => return PhpZobrist64 { inner: Zobrist64(0) },
        };
        PhpZobrist64 {
            inner: Zobrist64::zobrist_for_castling_right(color, side),
        }
    }

    /// Polyglot Zobrist component for an en-passant file (`$file`: 0=File A .. 7=File H).
    ///
    /// Out-of-range file yields the zero hash.
    pub fn for_en_passant_file(file_value: i32) -> Self {
        let Ok(file) = File::try_from(file_value as u8) else {
            return PhpZobrist64 { inner: Zobrist64(0) };
        };
        PhpZobrist64 { inner: Zobrist64::zobrist_for_en_passant_file(file) }
    }
}
