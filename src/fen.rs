use ext_php_rs::exception::PhpException;
use ext_php_rs::prelude::*;
use ext_php_rs::types::{ZendClassObject, ZendObject};
use shakmaty::fen::Fen;
use shakmaty::variant::{Variant, VariantPosition};
use shakmaty::{CastlingMode, EnPassantMode};

use crate::chess_pos::PhpChess;
use crate::fen_errors::{lossy_error_exception, parse_error_to_exception};
use crate::setup::PhpSetup;
use crate::variant::variant_from_int;
use crate::variant_position::PhpVariantPosition;

fn ep_mode_from_int(value: i32) -> EnPassantMode {
    match value {
        1 => EnPassantMode::PseudoLegal,
        2 => EnPassantMode::Always,
        _ => EnPassantMode::Legal,
    }
}

fn castling_mode_from_int(value: i32) -> CastlingMode {
    if value == 1 { CastlingMode::Chess960 } else { CastlingMode::Standard }
}

/// PHP class: `shakmaty\Fen`
///
/// Value object wrapping a parsed Forsyth-Edwards Notation (FEN/EPD) string.
/// The Rust `shakmaty::fen::Fen` is stored internally, so an instance is always
/// syntactically valid once constructed.
///
/// # Construction
///
/// The public constructor was removed; create instances with the static
/// factories:
///
/// - `Fen::empty(): Fen` — returns an empty board `8/8/8/8/8/8/8/8 w - - 0 1`.
/// - `Fen::parse(string $fen): Fen` — parse a FEN/EPD string (throws a
///   `shakmaty\fen\ParseFenError` subclass on invalid input).
/// - `Fen::fromPosition(\shakmaty\Position $pos, ?int $mode = null): Fen`
/// - `Fen::fromSetup(\shakmaty\Setup $setup): Fen` (throws
///   `shakmaty\fen\LossyFenError` if the setup cannot be represented losslessly).
///
/// # Usage
///
/// ```php
/// $fen = \shakmaty\Fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
/// echo (string) $fen;          // __toString()
/// $setup = $fen->getSetup();
/// $pos = $fen->getPosition();  // \shakmaty\VariantPosition (Chess by default)
/// ```
///
/// # See Also
///
/// - [`Chess::fromFen()`](Chess)
/// - [`Chess::toFen()`](Chess)
/// - Original Rust type: [`shakmaty::fen::Fen`](https://docs.rs/shakmaty/0.30/shakmaty/fen/struct.Fen.html)
#[php_class]
#[php(name = "shakmaty\\Fen")]
pub struct PhpFen {
    pub inner: Fen,
}

#[php_impl]
impl PhpFen {
    /// Private constructor — use `Fen::empty()`, `Fen::parse()`, `Fen::fromPosition()`
    /// or `Fen::fromSetup()` instead.
    #[php(vis = "private")]
    pub fn __construct() -> Self {
        Self::empty()
    }

    /// Returns a FEN representing an empty board.
    ///
    /// This is the FEN `8/8/8/8/8/8/8/8 w - - 0 1`. Useful as a neutral
    /// starting value (e.g. in tests) without parsing a string.
    pub fn empty() -> Self {
        PhpFen { inner: Fen::empty() }
    }

    /// Parses a FEN or EPD string.
    ///
    /// Missing FEN fields are filled with defaults (the board field is required).
    ///
    /// # Errors
    ///
    /// Throws a subclass of `shakmaty\fen\ParseFenError` describing the invalid part.
    ///
    /// ```php
    /// $fen = \shakmaty\Fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// ```
    pub fn parse(fen: String) -> Result<Self, PhpException> {
        Fen::from_ascii(fen.as_bytes())
            .map(|inner| PhpFen { inner })
            .map_err(parse_error_to_exception)
    }

    /// Builds a FEN from a position (`shakmaty\Chess` or `shakmaty\VariantPosition`).
    ///
    /// `$mode` is the en passant mode used to serialize the position:
    /// 0 = Legal (default), 1 = PseudoLegal, 2 = Always.
    ///
    /// # Errors
    ///
    /// Throws when `$pos` is not a `shakmaty\Position` instance.
    pub fn from_position(pos: &ZendObject, mode: Option<i32>) -> Result<Self, PhpException> {
        let em = ep_mode_from_int(mode.unwrap_or(0));
        if let Ok(chess) = pos.extract::<&ZendClassObject<PhpChess>>() {
            return Ok(PhpFen { inner: Fen::from_position(&chess.inner, em) });
        }
        if let Ok(variant) = pos.extract::<&ZendClassObject<PhpVariantPosition>>() {
            return Ok(PhpFen { inner: Fen::from_position(&variant.inner, em) });
        }
        Err(PhpException::from_message(
            "Expected a shakmaty\\Position instance".to_string(),
        ))
    }

    /// Builds a FEN from a `shakmaty\Setup`.
    ///
    /// # Errors
    ///
    /// Throws `shakmaty\fen\LossyFenError` if the setup cannot be losslessly
    /// represented (its `getCode()` is a bitmask of `LossyFenError` constants).
    pub fn from_setup(setup: &PhpSetup) -> Result<Self, PhpException> {
        Fen::try_from_setup(setup.inner.clone())
            .map(|inner| PhpFen { inner })
            .map_err(|e| lossy_error_exception(e.kinds().bits()))
    }

    /// Returns the underlying FEN string.
    ///
    /// Implements PHP's magic `__toString()`, so instances can be cast directly:
    /// `(string) $fen`.
    pub fn __to_string(&self) -> String {
        self.inner.to_string()
    }

    /// Validates whether `$fen` is a syntactically valid FEN/EPD string.
    ///
    /// This static helper never throws; use `Fen::parse()` if you want the specific error.
    pub fn is_valid(fen: String) -> bool {
        Fen::from_ascii(fen.as_bytes()).is_ok()
    }

    /// Returns the FEN contents as a `shakmaty\Setup`.
    pub fn get_setup(&self) -> PhpSetup {
        PhpSetup { inner: self.inner.as_setup().clone() }
    }

    /// Converts the FEN into a `shakmaty\VariantPosition`.
    ///
    /// `$variant`: 0 = Chess (default) … 7 = Horde. `$mode`: 0 = Standard (default), 1 = Chess960.
    ///
    /// # Errors
    ///
    /// Throws `"Invalid position"` if the resulting position is not legal.
    pub fn get_position(
        &self,
        variant: Option<i32>,
        mode: Option<i32>,
    ) -> Result<PhpVariantPosition, PhpException> {
        let variant = variant.map(variant_from_int).unwrap_or(Variant::Chess);
        let mode = castling_mode_from_int(mode.unwrap_or(0));
        VariantPosition::from_setup(variant, self.inner.as_setup().clone(), mode)
            .map(|inner| PhpVariantPosition { inner })
            .map_err(|_| PhpException::from_message("Invalid position".to_string()))
    }
}
