use ext_php_rs::class::RegisteredClass;
use ext_php_rs::exception::PhpException;
use ext_php_rs::prelude::*;
use ext_php_rs::zend::ce;
use shakmaty::fen::ParseFenError as RustParseFenError;

/// PHP exception: `shakmaty\fen\ParseFenError`
///
/// Base class for all FEN parsing errors. Extends the built-in `\Exception`.
/// Catch this type to handle any FEN parse failure, or catch one of the
/// concrete subclasses to distinguish the failing FEN part.
#[php_class]
#[php(name = "shakmaty\\fen\\ParseFenError")]
#[php(extends(ce = ce::exception, stub = "\\Exception"))]
#[derive(Default)]
pub struct PhpParseFenError;

/// PHP exception: `shakmaty\fen\InvalidFen`
#[php_class]
#[php(name = "shakmaty\\fen\\InvalidFen")]
#[php(extends(PhpParseFenError))]
#[derive(Default)]
pub struct PhpInvalidFen;

/// PHP exception: `shakmaty\fen\InvalidBoard`
#[php_class]
#[php(name = "shakmaty\\fen\\InvalidBoard")]
#[php(extends(PhpParseFenError))]
#[derive(Default)]
pub struct PhpInvalidBoard;

/// PHP exception: `shakmaty\fen\InvalidPocket`
#[php_class]
#[php(name = "shakmaty\\fen\\InvalidPocket")]
#[php(extends(PhpParseFenError))]
#[derive(Default)]
pub struct PhpInvalidPocket;

/// PHP exception: `shakmaty\fen\InvalidTurn`
#[php_class]
#[php(name = "shakmaty\\fen\\InvalidTurn")]
#[php(extends(PhpParseFenError))]
#[derive(Default)]
pub struct PhpInvalidTurn;

/// PHP exception: `shakmaty\fen\InvalidCastling`
#[php_class]
#[php(name = "shakmaty\\fen\\InvalidCastling")]
#[php(extends(PhpParseFenError))]
#[derive(Default)]
pub struct PhpInvalidCastling;

/// PHP exception: `shakmaty\fen\InvalidEpSquare`
#[php_class]
#[php(name = "shakmaty\\fen\\InvalidEpSquare")]
#[php(extends(PhpParseFenError))]
#[derive(Default)]
pub struct PhpInvalidEpSquare;

/// PHP exception: `shakmaty\fen\InvalidRemainingChecks`
#[php_class]
#[php(name = "shakmaty\\fen\\InvalidRemainingChecks")]
#[php(extends(PhpParseFenError))]
#[derive(Default)]
pub struct PhpInvalidRemainingChecks;

/// PHP exception: `shakmaty\fen\InvalidHalfmoveClock`
#[php_class]
#[php(name = "shakmaty\\fen\\InvalidHalfmoveClock")]
#[php(extends(PhpParseFenError))]
#[derive(Default)]
pub struct PhpInvalidHalfmoveClock;

/// PHP exception: `shakmaty\fen\InvalidFullmoves`
#[php_class]
#[php(name = "shakmaty\\fen\\InvalidFullmoves")]
#[php(extends(PhpParseFenError))]
#[derive(Default)]
pub struct PhpInvalidFullmoves;

/// PHP exception: `shakmaty\fen\LossyFenError`
///
/// Thrown when a `shakmaty\Setup` cannot be represented losslessly as a FEN.
/// The exception code is a bitmask of the reasons (see the `PROMOTED`,
/// `CASTLING_RIGHTS` and `POCKETS` constants), retrievable via `getCode()`.
#[php_class]
#[php(name = "shakmaty\\fen\\LossyFenError")]
#[php(extends(ce = ce::exception, stub = "\\Exception"))]
#[derive(Default)]
pub struct PhpLossyFenError;

#[php_impl]
impl PhpLossyFenError {
    /// Set of squares with promoted Crazyhouse pieces does not match the board.
    pub const PROMOTED: i32 = 1;
    /// More than two castling rights per side, or castling rights off the backrank.
    pub const CASTLING_RIGHTS: i32 = 2;
    /// More than 64 Crazyhouse pocket pieces.
    pub const POCKETS: i32 = 4;
}

/// Maps a Rust `ParseFenError` to the matching PHP exception subclass.
pub(crate) fn parse_error_to_exception(err: RustParseFenError) -> PhpException {
    let message = err.to_string();
    match err {
        RustParseFenError::InvalidFen => PhpException::from_class::<PhpInvalidFen>(message),
        RustParseFenError::InvalidBoard => PhpException::from_class::<PhpInvalidBoard>(message),
        RustParseFenError::InvalidPocket => PhpException::from_class::<PhpInvalidPocket>(message),
        RustParseFenError::InvalidTurn => PhpException::from_class::<PhpInvalidTurn>(message),
        RustParseFenError::InvalidCastling => {
            PhpException::from_class::<PhpInvalidCastling>(message)
        }
        RustParseFenError::InvalidEpSquare => {
            PhpException::from_class::<PhpInvalidEpSquare>(message)
        }
        RustParseFenError::InvalidRemainingChecks => {
            PhpException::from_class::<PhpInvalidRemainingChecks>(message)
        }
        RustParseFenError::InvalidHalfmoveClock => {
            PhpException::from_class::<PhpInvalidHalfmoveClock>(message)
        }
        RustParseFenError::InvalidFullmoves => PhpException::from_class::<PhpInvalidFullmoves>(message),
    }
}

/// Builds a `shakmaty\fen\LossyFenError` with the given kinds bitmask as code.
pub(crate) fn lossy_error_exception(kinds: u32) -> PhpException {
    PhpException::new(
        "setup cannot be losslessly represented as FEN".to_string(),
        kinds as i32,
        PhpLossyFenError::get_metadata().ce(),
    )
}
