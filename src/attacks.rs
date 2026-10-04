use ext_php_rs::prelude::*;
use shakmaty::{attacks, Bitboard, Color, Piece, Role, Square};

use crate::bitboard::PhpBitboard;
use crate::piece::PhpPiece;

/// PHP class: `shakmaty\Attacks`
///
/// Attack and ray lookup tables. A static-only utility class exposing the
/// free functions of [`shakmaty::attacks`](https://docs.rs/shakmaty/0.30/shakmaty/attacks/index.html).
///
/// Squares are passed as zero-based indices (`A1` = 0 .. `H8` = 63).
/// Out-of-range square arguments yield an empty `Bitboard` (or `false`) instead
/// of throwing.
///
/// # Usage
///
/// ```php
/// $att = \shakmaty\Attacks::bishop_attacks(
///     \shakmaty\Square::C2,
///     \shakmaty\Bitboard::from_rank(\shakmaty\Rank::SIXTH)
/// );
/// var_dump($att->has(\shakmaty\Square::G6)); // true
/// var_dump($att->has(\shakmaty\Square::H7)); // false
///
/// var_dump(\shakmaty\Attacks::aligned(
///     \shakmaty\Square::A1, \shakmaty\Square::B2, \shakmaty\Square::C3
/// )); // true
/// ```
#[php_class]
#[php(name = "shakmaty\\Attacks")]
pub struct PhpAttacks;

#[php_impl]
impl PhpAttacks {
    /// Constructs the (stateless) utility object.
    #[php(constructor)]
    pub fn new() -> Self {
        PhpAttacks
    }

    /// Attacks of a pawn of `$color` (0=Black, 1=White) standing on `$sq`.
    pub fn pawn_attacks(color_value: i32, sq: i32) -> PhpBitboard {
        let color = if color_value == 1 { Color::White } else { Color::Black };
        match Square::try_from(sq as u8) {
            Ok(s) => PhpBitboard { inner: attacks::pawn_attacks(color, s) },
            Err(_) => PhpBitboard { inner: Bitboard(0) },
        }
    }

    /// Attacks of a knight on `$sq`.
    pub fn knight_attacks(sq: i32) -> PhpBitboard {
        match Square::try_from(sq as u8) {
            Ok(s) => PhpBitboard { inner: attacks::knight_attacks(s) },
            Err(_) => PhpBitboard { inner: Bitboard(0) },
        }
    }

    /// Attacks of a king on `$sq`.
    pub fn king_attacks(sq: i32) -> PhpBitboard {
        match Square::try_from(sq as u8) {
            Ok(s) => PhpBitboard { inner: attacks::king_attacks(s) },
            Err(_) => PhpBitboard { inner: Bitboard(0) },
        }
    }

    /// Attacks of a bishop on `$sq`, blocked by `$occupied`.
    pub fn bishop_attacks(sq: i32, occupied: &PhpBitboard) -> PhpBitboard {
        match Square::try_from(sq as u8) {
            Ok(s) => PhpBitboard { inner: attacks::bishop_attacks(s, occupied.inner) },
            Err(_) => PhpBitboard { inner: Bitboard(0) },
        }
    }

    /// Attacks of a rook on `$sq`, blocked by `$occupied`.
    pub fn rook_attacks(sq: i32, occupied: &PhpBitboard) -> PhpBitboard {
        match Square::try_from(sq as u8) {
            Ok(s) => PhpBitboard { inner: attacks::rook_attacks(s, occupied.inner) },
            Err(_) => PhpBitboard { inner: Bitboard(0) },
        }
    }

    /// Attacks of a queen on `$sq`, blocked by `$occupied`.
    pub fn queen_attacks(sq: i32, occupied: &PhpBitboard) -> PhpBitboard {
        match Square::try_from(sq as u8) {
            Ok(s) => PhpBitboard { inner: attacks::queen_attacks(s, occupied.inner) },
            Err(_) => PhpBitboard { inner: Bitboard(0) },
        }
    }

    /// Attacks of `$piece` on `$sq`, blocked by `$occupied`.
    ///
    /// Dispatches by role: knight/king attacks ignore `$occupied`.
    pub fn attacks(sq: i32, piece: &PhpPiece, occupied: &PhpBitboard) -> PhpBitboard {
        let color = if piece.color == 1 { Color::White } else { Color::Black };
        let role = Role::try_from(piece.role).unwrap_or(Role::Pawn);
        let p = Piece { color, role };
        match Square::try_from(sq as u8) {
            Ok(s) => PhpBitboard { inner: attacks::attacks(s, p, occupied.inner) },
            Err(_) => PhpBitboard { inner: Bitboard(0) },
        }
    }

    /// The rank, file or diagonal containing both `$a` and `$b`
    /// (empty `Bitboard` if they are not aligned).
    pub fn ray(a: i32, b: i32) -> PhpBitboard {
        match (Square::try_from(a as u8), Square::try_from(b as u8)) {
            (Ok(sa), Ok(sb)) => PhpBitboard { inner: attacks::ray(sa, sb) },
            _ => PhpBitboard { inner: Bitboard(0) },
        }
    }

    /// The squares strictly between `$a` and `$b` (bounds excluded);
    /// empty `Bitboard` if they are not on the same rank, file or diagonal.
    pub fn between(a: i32, b: i32) -> PhpBitboard {
        match (Square::try_from(a as u8), Square::try_from(b as u8)) {
            (Ok(sa), Ok(sb)) => PhpBitboard { inner: attacks::between(sa, sb) },
            _ => PhpBitboard { inner: Bitboard(0) },
        }
    }

    /// Tests whether all three squares lie on one rank, file or diagonal.
    pub fn aligned(a: i32, b: i32, c: i32) -> bool {
        match (
            Square::try_from(a as u8),
            Square::try_from(b as u8),
            Square::try_from(c as u8),
        ) {
            (Ok(sa), Ok(sb), Ok(sc)) => attacks::aligned(sa, sb, sc),
            _ => false,
        }
    }
}
