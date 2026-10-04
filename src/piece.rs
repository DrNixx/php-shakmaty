use ext_php_rs::prelude::*;
use crate::enums::{PhpColor, PhpRole};

/// PHP class: `shakmaty\Piece`
///
/// Represents a colored chess piece — a combination of a Color and a Role.
///
/// # Usage
///
/// ```php
/// $wk = new \shakmaty\Piece(\shakmaty\Color::WHITE, \shakmaty\Role::KING);
/// echo $wk->toChar();    // "K"
/// echo $wk->toUnicode(); // "♔"
/// ```
///
/// # Properties
///
/// - `$piece->color` (`Color`, read-only) — the piece color.
/// - `$piece->role` (`Role`, read-only) — the piece role.
///
/// # See Also
///
/// Original Rust type: [`shakmaty::Piece`](https://docs.rs/shakmaty/0.30/shakmaty/struct.Piece.html)
#[php_class]
#[php(name = "shakmaty\\Piece")]
pub struct PhpPiece {
    pub color: u8,
    pub role: u8,
}

#[php_impl]
impl PhpPiece {
    /// Creates a new `Piece`.
    ///
    /// Accepts a color value (0=Black / non-zero=White) and a role value (1=Pawn .. 6=King);
    /// out-of-range values fall back to White Pawn.
    #[php(constructor)]
    pub fn new(color_value: i32, role_value: i32) -> Self {
        let c = match color_value { 0 => 0, _ => 1 };
        let r = match role_value { 1..=6 => role_value as u8, _ => 1 };
        PhpPiece { color: c, role: r }
    }

    /// Color of this piece (read-only PHP property `$color`).
    #[php(getter)]
    pub fn get_color(&self) -> PhpColor {
        PhpColor { inner: self.color }
    }

    /// Role (type) of this piece (read-only PHP property `$role`).
    #[php(getter)]
    pub fn get_role(&self) -> PhpRole {
        PhpRole { inner: self.role }
    }

    /// Returns the FEN character for this piece — uppercase for White, lowercase for Black.
    ///
    /// ```php
    /// $wk = new \shakmaty\Piece(\shakmaty\Color::WHITE, \shakmaty\Role::KING);
    /// echo $wk->toChar(); // "K"
    /// ```
    pub fn to_char(&self) -> String {
        let base = match self.role {
            1 => 'p', 2 => 'n', 3 => 'b',
            4 => 'r', 5 => 'q', 6 => 'k',
            _ => '?',
        };
        if self.color == 1 { base.to_uppercase().to_string() } else { base.to_string() }
    }

    /// Returns the Unicode glyph for this piece (e.g. `"♔"` white king, `"♚"` black king).
    pub fn to_unicode(&self) -> String {
        match (self.color, self.role) {
            (1, 1) => "♙", (0, 1) => "♟",
            (1, 2) => "♘", (0, 2) => "♞",
            (1, 3) => "♗", (0, 3) => "♝",
            (1, 4) => "♖", (0, 4) => "♜",
            (1, 5) => "♕", (0, 5) => "♛",
            (1, 6) => "♔", (0, 6) => "♚",
            _ => "?",
        }.to_string()
    }

    /// Creates a `Piece` from its FEN character.
    ///
    /// Accepts both uppercase and lowercase: `"P/p"`, `"N/n"`, `"B/b"`, `"R/r"`, `"Q/q"`, `"K/k"`;
    /// case determines the color (uppercase = White, lowercase = Black). Returns `None` for invalid characters.
    pub fn from_char(ch: String) -> Option<Self> {
        let c = ch.chars().next()?;
        let role = match c.to_ascii_lowercase() {
            'p' => 1, 'n' => 2, 'b' => 3,
            'r' => 4, 'q' => 5, 'k' => 6,
            _ => return None,
        };
        let color = if c.is_uppercase() { 1 } else { 0 };
        Some(PhpPiece { color, role })
    }
}
