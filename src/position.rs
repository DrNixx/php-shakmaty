use ext_php_rs::prelude::*;
use ext_php_rs::flags::ClassFlags;
use crate::bitboard::PhpBitboard;
use crate::board::PhpBoard;
use crate::move_::{PhpMove, PhpMoveList};
use crate::pockets::PhpPockets;
use crate::remaining_checks::PhpRemainingChecks;
use crate::setup::PhpSetup;
use crate::zobrist64::PhpZobrist64;

/// PHP abstract class: `shakmaty\Position`
///
/// Abstract base class that mirrors the [`shakmaty::Position`] trait from the
/// underlying Rust crate. Concrete position classes such as
/// [`Chess`](crate::chess_pos::PhpChess) extend it and implement every method
/// declared here.
///
/// `Position` cannot be instantiated directly: the class is registered as
/// abstract (`ClassFlags::Abstract`) and its constructor is protected. Use
/// `$pos instanceof \shakmaty\Position` for type checks.
///
/// # Default implementations
///
/// The Rust trait provides default bodies for many methods (`checkers`,
/// `is_check`, `is_checkmate`, `is_stalemate`, `capture_moves`, ...).
/// ext-php-rs does not dispatch concrete methods of a Rust-registered parent
/// class on child instances, so those defaults are exposed on the concrete
/// subclasses (see `Chess`), delegating to `shakmaty::Position`. Therefore this
/// abstract class declares the full contract as abstract methods.
///
/// # See Also
///
/// Original Rust trait:
/// [`shakmaty::Position`](https://docs.rs/shakmaty/0.30/shakmaty/trait.Position.html)
#[php_class]
#[php(name = "shakmaty\\Position")]
#[php(flags = ClassFlags::Abstract)]
pub struct PhpPosition;

#[allow(unused_variables)]
#[php_impl]
impl PhpPosition {
    /// Protected constructor — this class is abstract and cannot be instantiated.
    #[php(vis = "protected")]
    pub fn __construct() -> Self {
        PhpPosition
    }

    // === Methods required by the trait (no default body) ===

    /// Piece positions on the board.
    #[php(abstract)]
    pub fn board(&self) -> PhpBoard { unimplemented!() }

    /// Positions of tracked promoted pieces (always empty for standard chess).
    #[php(abstract)]
    pub fn promoted(&self) -> PhpBitboard { unimplemented!() }

    /// Crazyhouse pockets, or `null` when the position has none.
    #[php(abstract)]
    pub fn pockets(&self) -> Option<PhpPockets> { unimplemented!() }

    /// Remaining checks for Three-Check for `color` (1 = White, 0 = Black), or `null`.
    #[php(abstract)]
    pub fn remaining_checks(&self, color: i32) -> Option<PhpRemainingChecks> { unimplemented!() }

    /// Side to move (1 = White, 0 = Black).
    #[php(abstract)]
    pub fn turn(&self) -> i32 { unimplemented!() }

    /// Castling rights as a bitboard.
    #[php(abstract)]
    pub fn castling_rights(&self) -> PhpBitboard { unimplemented!() }

    /// En passant target square after a double pawn push, unconditionally.
    #[php(abstract)]
    pub fn maybe_ep_square(&self) -> Option<i32> { unimplemented!() }

    /// Number of half-moves since the last capture or pawn move.
    #[php(abstract)]
    pub fn halfmoves(&self) -> i32 { unimplemented!() }

    /// Move number (starts at 1).
    #[php(abstract)]
    pub fn fullmoves(&self) -> i32 { unimplemented!() }

    /// Generates all legal moves.
    #[php(abstract)]
    pub fn legal_moves(&self) -> PhpMoveList { unimplemented!() }

    /// Whether the game is over due to a variant-specific end condition.
    #[php(abstract)]
    pub fn is_variant_end(&self) -> bool { unimplemented!() }

    /// Whether `color` has insufficient winning material.
    #[php(abstract)]
    pub fn has_insufficient_material(&self, color: i32) -> bool { unimplemented!() }

    /// Special variant outcome as a string ("1-0" / "0-1" / "1/2-1/2" / "*").
    #[php(abstract)]
    pub fn variant_outcome(&self) -> String { unimplemented!() }

    /// Plays a move without legality checks (mutates the position).
    #[php(abstract)]
    pub fn play_unchecked(&mut self, m: &PhpMove) { unimplemented!() }

    // === Provided trait methods (default behaviour) ===

    /// Squares occupied by the side to move.
    #[php(abstract)]
    pub fn us(&self) -> PhpBitboard { unimplemented!() }

    /// Squares occupied by `role` of the side to move.
    #[php(abstract)]
    pub fn our(&self, role: i32) -> PhpBitboard { unimplemented!() }

    /// Squares occupied by the opponent.
    #[php(abstract)]
    pub fn them(&self) -> PhpBitboard { unimplemented!() }

    /// Squares occupied by `role` of the opponent.
    #[php(abstract)]
    pub fn their(&self, role: i32) -> PhpBitboard { unimplemented!() }

    /// Bitboard of pieces giving check.
    #[php(abstract)]
    pub fn checkers(&self) -> PhpBitboard { unimplemented!() }

    /// Whether the side to move is in check.
    #[php(abstract)]
    pub fn is_check(&self) -> bool { unimplemented!() }

    /// Whether the side to move is checkmated.
    #[php(abstract)]
    pub fn is_checkmate(&self) -> bool { unimplemented!() }

    /// Whether the position is a stalemate.
    #[php(abstract)]
    pub fn is_stalemate(&self) -> bool { unimplemented!() }

    /// Whether both sides have insufficient winning material.
    #[php(abstract)]
    pub fn is_insufficient_material(&self) -> bool { unimplemented!() }

    /// Whether the game is over.
    #[php(abstract)]
    pub fn is_game_over(&self) -> bool { unimplemented!() }

    /// Game outcome as a string ("1-0" / "0-1" / "1/2-1/2" / "*").
    #[php(abstract)]
    pub fn outcome(&self) -> String { unimplemented!() }

    /// En passant square for the given mode (defaults to Legal when null).
    #[php(abstract)]
    pub fn ep_square(&self, mode: Option<i32>) -> Option<i32> { unimplemented!() }

    /// En passant square if it is the target of a pseudo-legal en passant move.
    #[php(abstract)]
    pub fn pseudo_legal_ep_square(&self) -> Option<i32> { unimplemented!() }

    /// En passant square if it is the target of a legal en passant move.
    #[php(abstract)]
    pub fn legal_ep_square(&self) -> Option<i32> { unimplemented!() }

    /// Generates capture moves.
    #[php(abstract)]
    pub fn capture_moves(&self) -> PhpMoveList { unimplemented!() }

    /// Generates promotion moves.
    #[php(abstract)]
    pub fn promotion_moves(&self) -> PhpMoveList { unimplemented!() }

    /// Generates en passant moves.
    #[php(abstract)]
    pub fn en_passant_moves(&self) -> PhpMoveList { unimplemented!() }

    /// Generates castling moves for the given side (0 = king-side, 1 = queen-side).
    #[php(abstract)]
    pub fn castling_moves(&self, side: i32) -> PhpMoveList { unimplemented!() }

    /// Generates SAN candidate moves of `role` to square `to`.
    #[php(abstract)]
    pub fn san_candidates(&self, role: i32, to: i32) -> PhpMoveList { unimplemented!() }

    /// Attacks that a king on `square` would have to deal with.
    #[php(abstract)]
    pub fn king_attackers(&self, square: i32, attacker: i32, occupied: &PhpBitboard) -> PhpBitboard { unimplemented!() }

    /// Whether a move is irreversible.
    #[php(abstract)]
    pub fn is_irreversible(&self, m: &PhpMove) -> bool { unimplemented!() }

    /// Tests a move for legality.
    #[php(abstract)]
    pub fn is_legal(&self, m: &PhpMove) -> bool { unimplemented!() }

    /// Plays a legal move, returning the resulting position, or throws on illegal move.
    #[php(abstract)]
    pub fn play(&self, m: &PhpMove) -> Result<PhpPosition, &'static str> { unimplemented!() }

    /// Converts the position to a `Setup` for the given en passant mode.
    #[php(abstract)]
    pub fn to_setup(&self, mode: i32) -> PhpSetup { unimplemented!() }

    // === Zobrist hash (64-bit) ===

    /// Computes the 64-bit Zobrist hash of the position (excludes halfmove/fullmove counters).
    #[php(abstract)]
    pub fn zobrist_hash(&self, mode: Option<i32>) -> PhpZobrist64 { unimplemented!() }

    /// Incrementally updates a 64-bit Zobrist hash after legal move `m`, or `null` if unsupported.
    #[php(abstract)]
    pub fn update_zobrist_hash(&self, current: &PhpZobrist64, m: &PhpMove, mode: Option<i32>) -> Option<PhpZobrist64> { unimplemented!() }
}
