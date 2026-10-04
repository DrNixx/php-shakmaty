use ext_php_rs::prelude::*;

/// PHP class: `shakmaty\Color`
///
/// Represents a chess piece color — either White or Black.
///
/// # Constants
///
/// - `Color::WHITE` = 1
/// - `Color::BLACK` = 0
///
/// # Usage
///
/// ```php
/// $white = new \shakmaty\Color(\shakmaty\Color::WHITE);
/// echo $white->toChar(); // "w"
/// $black = $white->other();
/// echo $black->isBlack(); // true
/// ```
///
/// # See Also
///
/// Original Rust type: [`shakmaty::Color`](https://docs.rs/shakmaty/0.30/shakmaty/enum.Color.html)
#[php_class]
#[php(name = "shakmaty\\Color")]
pub struct PhpColor {
    pub inner: u8, // 0=Black, 1=White
}


#[php_impl]
impl PhpColor {
    /// Creates a new `Color`.
    ///
    /// Accepts 0 (Black) or any non-zero value (White).
    #[php(constructor)]
    pub fn new(value: i32) -> Self {
        let v = match value { 0 => 0, _ => 1 };
        PhpColor { inner: v }
    }

    /// Returns 1 for White, 0 for Black.
    pub fn value(&self) -> i32 { self.inner as i32 }

    /// Returns `true` if the color is White.
    pub fn is_white(&self) -> bool { self.inner == 1 }

    /// Returns `true` if the color is Black.
    pub fn is_black(&self) -> bool { self.inner == 0 }

    /// Returns the opposite color.
    ///
    /// ```php
    /// $white = new \shakmaty\Color(\shakmaty\Color::WHITE);
    /// $black = $white->other();
    /// var_dump($black->isBlack()); // true
    /// ```
    pub fn other(&self) -> Self {
        PhpColor { inner: self.inner ^ 1 }
    }

    /// Returns the FEN character for the color.
    ///
    /// Returns `"w"` for White, `"b"` for Black.
    pub fn to_char(&self) -> String {
        match self.inner { 1 => "w".to_string(), _ => "b".to_string() }
    }

    pub const WHITE: i32 = 1;
    pub const BLACK: i32 = 0;
}

/// PHP class: `shakmaty\Role`
///
/// Represents a chess piece role (type): Pawn, Knight, Bishop, Rook, Queen or King.
///
/// # Constants
///
/// - `Role::PAWN` = 1 through `Role::KING` = 6
///
/// # Usage
///
/// ```php
/// $knight = \shakmaty\Role::fromChar('N');
/// echo $knight->value(); // 2
/// echo $knight->toChar();    // "n"
/// echo $knight->upperChar(); // "N"
/// ```
///
/// # See Also
///
/// Original Rust type: [`shakmaty::Role`](https://docs.rs/shakmaty/0.30/shakmaty/enum.Role.html)
#[php_class]
#[php(name = "shakmaty\\Role")]
pub struct PhpRole {
    pub inner: u8, // 1=Pawn..6=King
}


#[php_impl]
impl PhpRole {
    /// Creates a new `Role`.
    ///
    /// Accepts values from 1 (Pawn) to 6 (King); out-of-range values fall back to Pawn.
    #[php(constructor)]
    pub fn new(value: i32) -> Self {
        let v = match value {
            1..=6 => value as u8,
            _ => 1,
        };
        PhpRole { inner: v }
    }

    /// Returns the numeric role (1=Pawn .. 6=King).
    pub fn value(&self) -> i32 { self.inner as i32 }

    /// Returns the lowercase FEN character for the role.
    ///
    /// ```php
    /// echo \shakmaty\Role::fromChar('N')->toChar(); // "n"
    /// ```
    pub fn to_char(&self) -> String {
        match self.inner {
            1 => "p", 2 => "n", 3 => "b",
            4 => "r", 5 => "q", 6 => "k",
            _ => "?",
        }.to_string()
    }

    /// Returns the uppercase FEN character for the role.
    pub fn upper_char(&self) -> String {
        self.to_char().to_uppercase()
    }

    /// Creates a `Role` from a character.
    ///
    /// Accepts both uppercase and lowercase: `P/p`, `N/n`, `B/b`, `R/r`, `Q/q`, `K/k`.
    ///
    /// Returns `None` for invalid characters.
    ///
    /// ```php
    /// $knight = \shakmaty\Role::fromChar('N');
    /// echo $knight->value(); // 2
    /// ```
    pub fn from_char(ch: String) -> Option<Self> {
        let c = ch.chars().next()?;
        let v = match c {
            'P' | 'p' => 1, 'N' | 'n' => 2, 'B' | 'b' => 3,
            'R' | 'r' => 4, 'Q' | 'q' => 5, 'K' | 'k' => 6,
            _ => return None,
        };
        Some(PhpRole { inner: v })
    }

    pub const PAWN: i32 = 1;
    pub const KNIGHT: i32 = 2;
    pub const BISHOP: i32 = 3;
    pub const ROOK: i32 = 4;
    pub const QUEEN: i32 = 5;
    pub const KING: i32 = 6;
}

/// PHP class: `shakmaty\CastlingSide`
///
/// Represents a castling side — either King-side (Kingside) or Queen-side.
///
/// # Constants
///
/// - `CastlingSide::KING_SIDE` = 0
/// - `CastlingSide::QUEEN_SIDE` = 1
///
/// # Usage
///
/// ```php
/// $ks = new \shakmaty\CastlingSide(\shakmaty\CastlingSide::KING_SIDE);
/// var_dump($ks->isKingSide()); // true
/// echo $ks->other()->value();  // 1 (Queen-side)
/// ```
///
/// # See Also
///
/// Original Rust type: [`shakmaty::CastlingSide`](https://docs.rs/shakmaty/0.30/shakmaty/enum.CastlingSide.html)
#[php_class]
#[php(name = "shakmaty\\CastlingSide")]
pub struct PhpCastlingSide {
    pub inner: u8, // 0=KingSide, 1=QueenSide
}


#[php_impl]
impl PhpCastlingSide {
    /// Creates a new `CastlingSide`.
    ///
    /// Accepts 0 (King-side) or any non-zero value (Queen-side).
    #[php(constructor)]
    pub fn new(value: i32) -> Self {
        PhpCastlingSide { inner: if value == 1 { 1 } else { 0 } }
    }

    /// Returns the numeric side (0=King-side, 1=Queen-side).
    pub fn value(&self) -> i32 { self.inner as i32 }

    /// Returns `true` if this is the King-side.
    pub fn is_king_side(&self) -> bool { self.inner == 0 }

    /// Returns `true` if this is the Queen-side.
    pub fn is_queen_side(&self) -> bool { self.inner == 1 }

    /// Returns the opposite castling side.
    pub fn other(&self) -> Self {
        PhpCastlingSide { inner: self.inner ^ 1 }
    }

    pub const KING_SIDE: i32 = 0;
    pub const QUEEN_SIDE: i32 = 1;
}

/// PHP class: `shakmaty\CastlingMode`
///
/// Represents the castling mode — either Standard (FIDE) or Chess960.
///
/// # Constants
///
/// - `CastlingMode::STANDARD` = 0
/// - `CastlingMode::CHESS960` = 1
///
/// # Usage
///
/// ```php
/// $mode = new \shakmaty\CastlingMode(\shakmaty\CastlingMode::CHESS960);
/// var_dump($mode->isChess960()); // true
/// echo $mode->value();           // 1
/// ```
///
/// # See Also
///
/// Original Rust type: [`shakmaty::CastlingMode`](https://docs.rs/shakmaty/0.30/shakmaty/enum.CastlingMode.html)
#[php_class]
#[php(name = "shakmaty\\CastlingMode")]
pub struct PhpCastlingMode {
    pub inner: u8, // 0=Standard, 1=Chess960
}


#[php_impl]
impl PhpCastlingMode {
    /// Creates a new `CastlingMode`.
    ///
    /// Accepts 0 (Standard) or any non-zero value (Chess960).
    #[php(constructor)]
    pub fn new(value: i32) -> Self {
        PhpCastlingMode { inner: if value == 1 { 1 } else { 0 } }
    }

    /// Returns the numeric mode (0=Standard, 1=Chess960).
    pub fn value(&self) -> i32 { self.inner as i32 }

    /// Returns `true` if this is Standard castling.
    pub fn is_standard(&self) -> bool { self.inner == 0 }

    /// Returns `true` if this is Chess960 (Fischer Random) castling.
    pub fn is_chess960(&self) -> bool { self.inner == 1 }

    pub const STANDARD: i32 = 0;

    #[php(name = "CHESS960")]
    pub const CHESS960: i32 = 1;
}

/// PHP class: `shakmaty\EnPassantMode`
///
/// Represents the en-passant capture mode used when generating moves.
///
/// # Constants
///
/// - `EnPassantMode::LEGAL` = 0 — only legal captures are generated
/// - `EnPassantMode::PSEUDO_LEGAL` = 1 — pseudo-legal (may include illegal) captures
/// - `EnPassantMode::ALWAYS` = 2 — always allow en-passant generation
///
/// # Usage
///
/// ```php
/// $mode = new \shakmaty\EnPassantMode(\shakmaty\EnPassantMode::PSEUDO_LEGAL);
/// echo $mode->value(); // 1
/// ```
///
/// # See Also
///
/// Original Rust type: [`shakmaty::EnPassantMode`](https://docs.rs/shakmaty/0.30/shakmaty/enum.EnPassantMode.html)
#[php_class]
#[php(name = "shakmaty\\EnPassantMode")]
pub struct PhpEnPassantMode {
    pub inner: u8, // 0=Legal, 1=PseudoLegal, 2=Always
}


#[php_impl]
impl PhpEnPassantMode {
    /// Creates a new `EnPassantMode`.
    ///
    /// Accepts values from 0 (Legal) to 2 (Always); out-of-range values fall back to Legal.
    #[php(constructor)]
    pub fn new(value: i32) -> Self {
        let v = match value { 0..=2 => value as u8, _ => 0 };
        PhpEnPassantMode { inner: v }
    }

    /// Returns the numeric mode (0=Legal, 1=Pseudo-Legal, 2=Always).
    pub fn value(&self) -> i32 { self.inner as i32 }

    pub const LEGAL: i32 = 0;
    pub const PSEUDO_LEGAL: i32 = 1;
    pub const ALWAYS: i32 = 2;
}
