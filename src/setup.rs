use ext_php_rs::prelude::*;
use core::num::NonZeroU32;
use shakmaty::Setup;
use crate::bitboard::PhpBitboard;
use crate::board::PhpBoard;
use crate::pockets::PhpPockets;
use crate::remaining_checks::PhpRemainingChecks;

use ext_php_rs::exception::PhpException;
use shakmaty::fen::Fen;
use crate::fen_errors::parse_error_to_exception;

fn color_of(value: i32) -> shakmaty::Color {
    if value == 1 { shakmaty::Color::White } else { shakmaty::Color::Black }
}

/// PHP class: `shakmaty\Setup`
///
/// Represents a chess position setup that is not necessarily legal.
/// Allows setting board, turn, castling rights, en passant square,
/// and move clocks.
///
/// # Properties
///
/// Scalar state is exposed as writable PHP properties, object-valued state
/// as read-only properties:
///
/// - `$setup->turn` (`int`, writable) — side to move, `1` = White, `0` = Black.
/// - `$setup->halfmoves` (`int`, writable) — halfmove clock.
/// - `$setup->fullmoves` (`int`, writable) — full move number.
/// - `$setup->epSquare` (`?int`, writable) — en passant square index or `null`.
/// - `$setup->board` (`Board`, read-only) — write via `setBoard()`.
/// - `$setup->castlingRights` (`Bitboard`, read-only) — write via `setCastlingRights()`.
/// - `$setup->promoted` (`Bitboard`, read-only) — write via `setPromoted()`.
/// - `$setup->pockets` (`?Pockets`, read-only) — write via `setPockets()`, clear via `clearPockets()`.
///
/// Remaining checks stay method-only: `remainingChecks()`, `setRemainingChecks()`,
/// `clearRemainingChecks()`.
///
/// # Usage
///
/// ```php
/// $setup = new \shakmaty\Setup();
/// echo $setup->turn; // 1 (White)
/// $setup->turn = 0;
/// $setup->epSquare = \shakmaty\Square::E3;
///
/// // Or parse from a FEN string:
/// $setup = \shakmaty\Setup::fromFen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
/// ```
///
/// # See Also
///
/// Original Rust type: [`shakmaty::Setup`](https://docs.rs/shakmaty/0.30/shakmaty/struct.Setup.html)
#[php_class]
#[php(name = "shakmaty\\Setup")]
pub struct PhpSetup {
    pub inner: Setup,
}

#[php_impl]
impl PhpSetup {
    /// Creates the standard starting position setup.
    #[php(constructor)]
    pub fn new() -> Self {
        PhpSetup { inner: Setup::default() }
    }

    /// Creates an empty setup with no pieces and default values.
    pub fn empty() -> Self {
        PhpSetup { inner: Setup::empty() }
    }

    /// Parses a FEN/EPD string into a `shakmaty\Setup`.
    ///
    /// Unlike `Chess::fromFen()`, no legality check is performed: the piece
    /// placement, turn, castling rights, en passant square and move counters
    /// are taken as-is, so the resulting `Setup` may describe an illegal
    /// position. This makes it suitable for storing/round-tripping any FEN.
    ///
    /// # Errors
    ///
    /// Throws a subclass of `shakmaty\fen\ParseFenError` describing the invalid
    /// part (the board field is required; missing trailing fields get defaults).
    ///
    /// ```php
    /// $setup = \shakmaty\Setup::fromFen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// echo $setup->turn; // 1 (White)
    /// ```
    pub fn from_fen(fen: String) -> Result<Self, PhpException> {
        Fen::from_ascii(fen.as_bytes())
            .map(|parsed| PhpSetup { inner: parsed.into_setup() })
            .map_err(parse_error_to_exception)
    }

    /// Board of this setup as a `shakmaty\Board` object.
    ///
    /// Read-only PHP property `$board`; mutate via `setBoard()`.
    #[php(getter)]
    pub fn get_board(&self) -> PhpBoard {
        PhpBoard { inner: self.inner.board.clone() }
    }

    /// Sets the board for this setup from another Board instance.
    pub fn set_board(&mut self, board: &PhpBoard) {
        self.inner.board = board.inner.clone();
    }

    /// Side to move (PHP property `$turn`): `1` = White, `0` = Black. Writable.
    #[php(getter)]
    pub fn get_turn(&self) -> i32 {
        match self.inner.turn {
            shakmaty::Color::White => 1,
            shakmaty::Color::Black => 0,
        }
    }

    /// Sets the side to move (`1` = White, `0` = Black). Backs the writable
    /// PHP property `$turn`.
    #[php(setter)]
    pub fn set_turn(&mut self, color: i32) {
        self.inner.turn = if color == 1 { shakmaty::Color::White } else { shakmaty::Color::Black };
    }

    /// Castling rights as a bitboard (read-only PHP property `$castlingRights`).
    #[php(getter)]
    pub fn get_castling_rights(&self) -> PhpBitboard {
        PhpBitboard { inner: self.inner.castling_rights }
    }

    /// Sets the castling rights from a Bitboard instance.
    pub fn set_castling_rights(&mut self, bb: &PhpBitboard) {
        self.inner.castling_rights = bb.inner;
    }

    /// En passant square index (0–63), or `null` (writable PHP property `$epSquare`).
    #[php(getter)]
    pub fn get_ep_square(&self) -> Option<i32> {
        self.inner.ep_square.map(|s| s as i32)
    }

    /// Sets the en passant square by its 0-based index, or clears it with `null`.
    /// Backs the writable PHP property `$epSquare`.
    #[php(setter)]
    pub fn set_ep_square(&mut self, sq: Option<i32>) {
        self.inner.ep_square = sq.and_then(|v| {
            if (0..=63).contains(&v) {
                shakmaty::Square::try_from(v as u8).ok()
            } else {
                None
            }
        });
    }

    /// Halfmove clock — plies since last capture or pawn move
    /// (writable PHP property `$halfmoves`).
    #[php(getter)]
    pub fn get_halfmoves(&self) -> i32 {
        self.inner.halfmoves as i32
    }

    /// Sets the halfmove clock (clamped to 0). Backs the writable PHP property `$halfmoves`.
    #[php(setter)]
    pub fn set_halfmoves(&mut self, hm: i32) {
        self.inner.halfmoves = hm.max(0) as u32;
    }

    /// Full move number; starts at 1 (writable PHP property `$fullmoves`).
    #[php(getter)]
    pub fn get_fullmoves(&self) -> i32 {
        self.inner.fullmoves.get() as i32
    }

    /// Sets the full move number (clamped to minimum of 1).
    /// Backs the writable PHP property `$fullmoves`.
    #[php(setter)]
    pub fn set_fullmoves(&mut self, fm: i32) {
        self.inner.fullmoves = NonZeroU32::new(fm.max(1) as u32).unwrap_or(NonZeroU32::MIN);
    }

    /// Positions of tracked promoted pieces (Crazyhouse) as a bitboard
    /// (read-only PHP property `$promoted`).
    #[php(getter)]
    pub fn get_promoted(&self) -> PhpBitboard {
        PhpBitboard { inner: self.inner.promoted }
    }

    /// Sets the tracked promoted-pieces bitboard.
    pub fn set_promoted(&mut self, bb: &PhpBitboard) {
        self.inner.promoted = bb.inner;
    }

    /// Crazyhouse pockets, or `null` when not set
    /// (read-only PHP property `$pockets`).
    #[php(getter)]
    pub fn get_pockets(&self) -> Option<PhpPockets> {
        self.inner.pockets.as_ref().map(|p| PhpPockets { inner: *p })
    }

    /// Sets Crazyhouse pockets.
    pub fn set_pockets(&mut self, pockets: &PhpPockets) {
        self.inner.pockets = Some(pockets.inner);
    }

    /// Clears Crazyhouse pockets.
    pub fn clear_pockets(&mut self) {
        self.inner.pockets = None;
    }

    /// Remaining checks for Three-Check for `color` (`1` = White, `0` = Black), or `null` when unset.
    pub fn remaining_checks(&self, color: i32) -> Option<PhpRemainingChecks> {
        self.inner
            .remaining_checks
            .as_ref()
            .map(|rc| PhpRemainingChecks { inner: *rc.get(color_of(color)) })
    }

    /// Sets remaining checks for `color`, creating the default 3+3 pair when absent.
    pub fn set_remaining_checks(&mut self, color: i32, checks: &PhpRemainingChecks) {
        let pair = self.inner.remaining_checks.get_or_insert_with(Default::default);
        *pair.get_mut(color_of(color)) = checks.inner;
    }

    /// Clears remaining checks.
    pub fn clear_remaining_checks(&mut self) {
        self.inner.remaining_checks = None;
    }
}
