use ext_php_rs::prelude::*;
use shakmaty::{Board, Color, Role, Square};
use crate::bitboard::PhpBitboard;

/// PHP class: `shakmaty\Board`
///
/// Represents piece positions on a chessboard. Provides query methods
/// for accessing pieces by square, color, and role.
///
/// # Usage
///
/// ```php
/// $board = new \shakmaty\Board();
/// echo $board->pieceAt(\shakmaty\Square::E1); // "K"
/// echo $board->byColor(1)->count(); // 16 (white pieces)
/// ```
///
/// # See Also
///
/// Original Rust type: [`shakmaty::Board`](https://docs.rs/shakmaty/0.30/shakmaty/struct.Board.html)
#[php_class]
#[php(name = "shakmaty\\Board")]
pub struct PhpBoard {
    pub inner: Board,
}

#[php_impl]
impl PhpBoard {
    /// Creates the standard starting position.
    #[php(constructor)]
    pub fn new() -> Self {
        PhpBoard { inner: Board::new() }
    }

    /// Creates an empty board with no pieces.
    pub fn empty() -> Self {
        PhpBoard { inner: Board::empty() }
    }

    /// Returns the piece on a given square, or `null` if the square is empty.
    ///
    /// # Usage
    ///
    /// ```php
    /// $board = new \shakmaty\Board();
    /// echo $board->pieceAt(\shakmaty\Square::E1); // "K" (white king)
    /// var_dump($board->pieceAt(99)); // null (invalid square index)
    /// ```
    pub fn piece_at(&self, sq: i32) -> Option<String> {
        if let Ok(s) = Square::try_from(sq as u8) {
            return self.inner.piece_at(s).map(|p| p.char().to_string());
        }
        None
    }

    /// Returns a bitboard of all occupied squares.
    pub fn occupied(&self) -> PhpBitboard {
        PhpBitboard { inner: self.inner.occupied() }
    }

    /// Returns a bitboard of pieces belonging to the given color (1 = White, 0 = Black).
    pub fn by_color(&self, color_value: i32) -> PhpBitboard {
        let c = if color_value == 1 { Color::White } else { Color::Black };
        PhpBitboard { inner: self.inner.by_color(c) }
    }

    /// Returns a bitboard of pieces with the given role.
    pub fn by_role(&self, role_value: i32) -> PhpBitboard {
        if let Ok(r) = Role::try_from(role_value as u8) {
            PhpBitboard { inner: self.inner.by_role(r) }
        } else {
            PhpBitboard { inner: shakmaty::Bitboard(0) }
        }
    }

    /// Returns the role of the piece on a given square, or `null` if empty.
    pub fn role_at(&self, sq: i32) -> Option<i32> {
        if let Ok(s) = Square::try_from(sq as u8) {
            return self.inner.role_at(s).map(|r| r as i32);
        }
        None
    }

    /// Returns the color of the piece on a given square (1 = White, 0 = Black), or `null` if empty.
    pub fn color_at(&self, sq: i32) -> Option<i32> {
        if let Ok(s) = Square::try_from(sq as u8) {
            return self.inner.color_at(s).map(|c| match c { Color::White => 1, _ => 0 });
        }
        None
    }

    /// Returns the legacy 12-code of the piece on the given square.
    ///
    /// White codes 1..6 (King=1 … Pawn=6); black codes 9..14 (King=9 … Pawn=14).
    /// Returns `7` (NOPIECE) for an empty square or an out-of-range square index.
    pub fn legacy_piece_at(&self, sq: i32) -> i32 {
        if let Ok(s) = Square::try_from(sq as u8) {
            if let Some(p) = self.inner.piece_at(s) {
                let color_bit = if p.color == Color::Black { 1 } else { 0 };
                return (color_bit << 3) | (7 - p.role as i32);
            }
        }
        7
    }
}
