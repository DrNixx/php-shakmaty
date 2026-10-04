use ext_php_rs::prelude::*;
use shakmaty::Move;

/// PHP class: `shakmaty\Move`
///
/// Represents a chess move, typically obtained from [`Chess::legalMoves()`](Chess).
///
/// # Usage
///
/// ```php
/// $pos = new \shakmaty\Chess();
/// $moves = $pos->legalMoves();
/// $first = $moves->get(0);
/// echo $first->toLong(); // "a2a3"
/// var_dump($first->isCapture()); // false
/// ```
///
/// # See Also
///
/// Original Rust type: [`shakmaty::Move`](https://docs.rs/shakmaty/0.30/shakmaty/enum.Move.html)
#[php_class]
#[php(name = "shakmaty\\Move")]
pub struct PhpMove {
    pub inner: Move,
}

#[php_impl]
impl PhpMove {
    /// Internal constructor — moves are typically created from SAN/UCI or legalMoves().
    #[php(constructor)]
    pub fn new() -> Self {
        // Placeholder: a default invalid move (should not be called directly)
        PhpMove { inner: Move::Normal { role: shakmaty::Role::Pawn, from: shakmaty::Square::A1, capture: None, to: shakmaty::Square::A1, promotion: None } }
    }

    /// Returns the piece role making this move (0 = Pawn … 5 = King).
    pub fn role(&self) -> i32 {
        self.inner.role() as i32
    }

    /// Returns the origin square index of this move, or `null` if not applicable.
    pub fn from_sq(&self) -> Option<i32> {
        self.inner.from().map(|s| s as i32)
    }

    /// Returns the destination square index (0–63).
    pub fn to_sq(&self) -> i32 {
        self.inner.to() as i32
    }

    /// Returns the role of a captured piece, or `null` if no capture.
    pub fn capture(&self) -> Option<i32> {
        self.inner.capture().map(|r| r as i32)
    }

    /// Whether this move captures an opponent's piece (including en passant).
    pub fn is_capture(&self) -> bool {
        self.inner.is_capture()
    }

    /// Whether this move is a castling operation.
    pub fn is_castle(&self) -> bool {
        self.inner.is_castle()
    }

    /// Whether this move is an en passant capture.
    pub fn is_en_passant(&self) -> bool {
        self.inner.is_en_passant()
    }

    /// Whether this move involves a pawn promotion.
    pub fn is_promotion(&self) -> bool {
        self.inner.is_promotion()
    }

    /// Returns the promoted piece role, or `null` if not a promotion.
    pub fn promotion(&self) -> Option<i32> {
        self.inner.promotion().map(|r| r as i32)
    }

    /// Whether this move is a Crazyhouse piece drop (e.g. `N@f3`).
    pub fn is_put(&self) -> bool {
        self.inner.is_put()
    }

    /// Creates a Crazyhouse piece-drop move: drops `role` onto square `to`.
    ///
    /// # Errors
    ///
    /// Throws `"Invalid role"` if `role` is outside `1..=6` (Pawn..King), or
    /// `"Invalid square"` if `to` is outside `0..=63`.
    ///
    /// ```php
    /// $drop = \shakmaty\Move::fromPut(\shakmaty\Role::KNIGHT, \shakmaty\Square::F3);
    /// echo $drop->toLong(); // "N@f3"
    /// ```
    pub fn from_put(role: i32, to: i32) -> Result<PhpMove, &'static str> {
        let r = shakmaty::Role::try_from(role as u8).map_err(|_| "Invalid role")?;
        let sq = shakmaty::Square::try_from(to as u8).map_err(|_| "Invalid square")?;
        Ok(PhpMove { inner: Move::Put { role: r, to: sq } })
    }

    /// Returns this move in long algebraic notation without a dash separator,
    /// e.g. `"e2e4"`, `"Nb1c3"` (pawn moves have no leading letter), captures keep `'x'`
    /// (`"exd5"`), promotions append `=Q`. Castling and en passant are rendered as-is
    /// by the underlying type (e.g., `"O-O"`, `"exd6"`). Crazyhouse drops render as
    /// `"N@f3"` (or `"@e4"` for a pawn drop).
    pub fn to_long(&self) -> String {
        let m = self.inner; // shakmaty::Move is Copy
        match m {
            shakmaty::Move::Normal { role, from, capture, to, promotion } => {
                let mut s = if role == shakmaty::Role::Pawn {
                    String::new()
                } else {
                    format!("{}", role.upper_char())
                };
                match capture {
                    Some(_) => s.push_str(&format!("{}x{}", from, to)),
                    None    => s.push_str(&format!("{}{}",  from, to)),
                }
                if let Some(p) = promotion {
                    s.push('=');
                    s.push(p.upper_char());
                }
                s
            }
            other => format!("{}", other), // EnPassant / Castle / Put — unchanged (e.g. "exd6", "O-O")
        }
    }
}

/// PHP class: `shakmaty\MoveList`
///
/// A container for chess moves. Returned by [`Chess::legalMoves()`](Chess).
///
/// # Usage
///
/// ```php
/// $pos = new \shakmaty\Chess();
/// $moves = $pos->legalMoves();
/// for ($i = 0; $i < $moves->count(); $i++) {
///     $move = $moves->get($i);
/// }
/// ```
#[php_class]
#[php(name = "shakmaty\\MoveList")]
pub struct PhpMoveList {
    pub inner: Vec<PhpMove>,
}

#[php_impl]
impl PhpMoveList {
    /// Creates an empty move list.
    #[php(constructor)]
    pub fn new() -> Self {
        PhpMoveList { inner: Vec::new() }
    }

    /// Returns the number of moves in this list.
    pub fn count(&self) -> i32 {
        self.inner.len() as i32
    }

    /// Returns the move at the given index, or `null` if out of range.
    pub fn get(&self, index: i32) -> Option<PhpMove> {
        self.inner.get(index as usize).map(|m| PhpMove { inner: m.inner })
    }
}
