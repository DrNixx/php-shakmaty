use ext_php_rs::exception::PhpException;
use ext_php_rs::prelude::*;
use ext_php_rs::types::{ZendClassObject, ZendObject};
use shakmaty::san::{San, SanError};

use crate::chess_pos::PhpChess;
use crate::move_::{PhpMove, PhpMoveList};
use crate::variant_position::PhpVariantPosition;

/// Parses `text` as a `shakmaty::san::San`, returning `None` on syntax errors.
fn parse_san(text: &str) -> Option<San> {
    text.parse::<San>().ok()
}

/// Maps a Rust `SanError` to the PHP exception thrown by `toMove()`.
fn san_error_exception(error: SanError) -> PhpException {
    PhpException::from_message(
        match error {
            SanError::IllegalSan => "Illegal SAN",
            SanError::AmbiguousSan => "Ambiguous SAN",
        }
        .to_string(),
    )
}

/// Exception for a `$pos` argument that is neither `Chess` nor `VariantPosition`.
fn position_not_found() -> PhpException {
    PhpException::from_message("Expected a shakmaty\\Position instance".to_string())
}

/// PHP class: `shakmaty\San`
///
/// A wrapper around a Standard Algebraic Notation (SAN) string.
/// SAN is the standard format for writing chess moves (e.g., "Nf3", "Qxf7#").
///
/// # Usage
///
/// ```php
/// $san = new \shakmaty\San("Nf3");
/// echo $san->__toString(); // "Nf3"
/// var_dump($san->isValid()); // true
///
/// // Convert a legal move to SAN and back
/// $move = $san->toMove(new \shakmaty\Chess());
/// echo $move->toLong(); // "Ng1f3"
/// ```
///
/// # See Also
///
/// - [`Chess::playSan()`](Chess)
/// - Original Rust type: [`shakmaty::san::San`](https://docs.rs/shakmaty/0.30/shakmaty/san/struct.San.html)
#[php_class]
#[php(name = "shakmaty\\San")]
pub struct PhpSan {
    inner: String,
}

#[php_impl]
impl PhpSan {
    /// Creates a new SAN wrapper. Does not validate immediately.
    #[php(constructor)]
    pub fn new(san: String) -> Self {
        PhpSan { inner: san }
    }

    /// Returns the underlying SAN string.
    pub fn __to_string(&self) -> String {
        self.inner.clone()
    }

    /// Validates the SAN string using the shakmaty parser.
    /// Returns `false` for strings that are not valid chess moves (e.g., "xyz").
    pub fn is_valid(&self) -> bool {
        parse_san(&self.inner).is_some()
    }

    /// Builds SAN from a legal `$move` in the context of `$pos`, disambiguating
    /// the origin file/rank only as needed.
    ///
    /// `$pos` must be a `shakmaty\Chess` or `shakmaty\VariantPosition` instance.
    ///
    /// # Errors
    ///
    /// Throws if `$pos` is not a `shakmaty\Position` instance.
    ///
    /// ```php
    /// $san = \shakmaty\San::fromMove(new \shakmaty\Chess(), $move);
    /// ```
    pub fn from_move(pos: &ZendObject, m: &PhpMove) -> Result<PhpSan, PhpException> {
        if let Ok(chess) = pos.extract::<&ZendClassObject<PhpChess>>() {
            return Ok(PhpSan {
                inner: San::from_move(&chess.inner, m.inner).to_string(),
            });
        }
        if let Ok(variant) = pos.extract::<&ZendClassObject<PhpVariantPosition>>() {
            return Ok(PhpSan {
                inner: San::from_move(&variant.inner, m.inner).to_string(),
            });
        }
        Err(position_not_found())
    }

    /// Resolves this SAN to the unique legal move in `$pos`.
    ///
    /// `$pos` must be a `shakmaty\Chess` or `shakmaty\VariantPosition` instance.
    ///
    /// # Errors
    ///
    /// Throws `"Invalid SAN"` if the stored string is not syntactically valid,
    /// `"Illegal SAN"` if it matches no legal move, or `"Ambiguous SAN"` if it
    /// matches more than one. Throws if `$pos` is not a `shakmaty\Position`.
    pub fn to_move(&self, pos: &ZendObject) -> Result<PhpMove, PhpException> {
        let san = parse_san(&self.inner)
            .ok_or_else(|| PhpException::from_message("Invalid SAN".to_string()))?;
        if let Ok(chess) = pos.extract::<&ZendClassObject<PhpChess>>() {
            return san
                .to_move(&chess.inner)
                .map(|inner| PhpMove { inner })
                .map_err(san_error_exception);
        }
        if let Ok(variant) = pos.extract::<&ZendClassObject<PhpVariantPosition>>() {
            return san
                .to_move(&variant.inner)
                .map(|inner| PhpMove { inner })
                .map_err(san_error_exception);
        }
        Err(position_not_found())
    }

    /// Searches `$moves` for the unique move matching this SAN (in any position).
    ///
    /// Returns the matching `shakmaty\Move`, or `null` when there is no unique
    /// match — i.e. no move matches, several matches are ambiguous, or the
    /// stored string is not syntactically valid SAN.
    pub fn find_move(&self, moves: &PhpMoveList) -> Option<PhpMove> {
        let san = parse_san(&self.inner)?;
        let mut list = shakmaty::MoveList::new();
        for m in &moves.inner {
            list.push(m.inner);
        }
        san.find_move(&list).ok().map(|m| PhpMove { inner: *m })
    }

    /// Whether this SAN can match `$m` in any position.
    ///
    /// Returns `false` for a syntactically invalid stored string.
    pub fn matches(&self, m: &PhpMove) -> bool {
        parse_san(&self.inner).is_some_and(|san| san.matches(m.inner))
    }
}
