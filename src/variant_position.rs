use ext_php_rs::prelude::*;
use shakmaty::{
    Bitboard, CastlingMode, CastlingSide, Color, EnPassantMode, MoveList, Position, Role, Setup,
    Square,
};
use shakmaty::fen::Fen;
use shakmaty::san::San;
use shakmaty::uci::UciMove;
use shakmaty::variant::{Variant, VariantPosition};
use shakmaty::zobrist::Zobrist64;
use crate::bitboard::PhpBitboard;
use crate::board::PhpBoard;
use crate::move_::{PhpMove, PhpMoveList};
use crate::pockets::PhpPockets;
use crate::position::PhpPosition;
use crate::remaining_checks::PhpRemainingChecks;
use crate::setup::PhpSetup;
use crate::zobrist64::PhpZobrist64;
use crate::variant::{variant_from_int, PhpVariant};

fn color_from_int(value: i32) -> Color {
    if value == 1 { Color::White } else { Color::Black }
}

fn role_from_int(value: i32) -> Option<Role> {
    Role::try_from(value as u8).ok()
}

fn square_from_int(value: i32) -> Option<Square> {
    Square::try_from(value as u8).ok()
}

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

fn side_from_int(value: i32) -> CastlingSide {
    if value == 1 { CastlingSide::QueenSide } else { CastlingSide::KingSide }
}

fn empty_bitboard() -> PhpBitboard {
    PhpBitboard { inner: Bitboard(0) }
}

fn moves_to_php(moves: MoveList) -> PhpMoveList {
    let mut vec = Vec::with_capacity(moves.len());
    for m in moves {
        vec.push(PhpMove { inner: m });
    }
    PhpMoveList { inner: vec }
}

/// PHP class: `shakmaty\VariantPosition`
///
/// Dynamically dispatched chess variant position. Extends the abstract
/// [`Position`](crate::position::PhpPosition) base class and mirrors the Rust
/// [`shakmaty::variant::VariantPosition`] enum. A single PHP class wraps all
/// eight shakmaty variants; the concrete variant is reported by `variant()`.
///
/// # Usage
///
/// ```php
/// $pos = new \shakmaty\VariantPosition(\shakmaty\Variant::ATOMIC);
/// echo $pos->variant()->uci();        // "atomic"
/// echo $pos->legalMovesCount();       // 20
/// $pos->playSan("e4");
/// echo $pos->turn();                  // 0 (Black)
/// ```
///
/// # See Also
///
/// Original Rust type: [`shakmaty::variant::VariantPosition`](https://docs.rs/shakmaty/0.30/shakmaty/variant/enum.VariantPosition.html)
#[php_class]
#[php(name = "shakmaty\\VariantPosition")]
#[php(extends(PhpPosition))]
pub struct PhpVariantPosition {
    pub inner: VariantPosition,
}

#[php_impl]
impl PhpVariantPosition {
    /// Creates a variant position for the given variant (default: Chess).
    #[php(constructor)]
    pub fn new(variant: Option<i32>) -> Self {
        let v = variant.map(variant_from_int).unwrap_or(Variant::Chess);
        PhpVariantPosition { inner: VariantPosition::new(v) }
    }

    /// Creates a variant position from a `Setup`. `mode`: 0 = Standard, 1 = Chess960.
    /// Throws `"Invalid position"` when the setup is not a valid position.
    pub fn from_setup(variant: i32, setup: &PhpSetup, mode: Option<i32>) -> Result<Self, &'static str> {
        let cm = castling_mode_from_int(mode.unwrap_or(0));
        VariantPosition::from_setup(variant_from_int(variant), setup.inner.clone(), cm)
            .map(|pos| PhpVariantPosition { inner: pos })
            .map_err(|_| "Invalid position")
    }

    /// Creates a variant position from a FEN string. `mode`: 0 = Standard, 1 = Chess960.
    /// Throws `"Invalid FEN"` or `"Invalid position"`.
    pub fn from_fen(variant: i32, fen: String, mode: Option<i32>) -> Result<Self, &'static str> {
        let parsed = Fen::from_ascii(fen.as_bytes()).map_err(|_| "Invalid FEN")?;
        let setup: Setup = parsed.into();
        let cm = castling_mode_from_int(mode.unwrap_or(0));
        VariantPosition::from_setup(variant_from_int(variant), setup, cm)
            .map(|pos| PhpVariantPosition { inner: pos })
            .map_err(|_| "Invalid position")
    }

    /// Returns the variant of this position.
    pub fn variant(&self) -> PhpVariant {
        PhpVariant { inner: self.inner.variant() }
    }

    /// Swaps the side to move. Throws `"Swap failed"` if the result is invalid.
    pub fn swap_turn(&self) -> Result<Self, &'static str> {
        self.inner.clone().swap_turn()
            .map(|pos| PhpVariantPosition { inner: pos })
            .map_err(|_| "Swap failed")
    }

    // === Position contract: required methods ===

    /// Piece positions on the board.
    pub fn board(&self) -> PhpBoard {
        PhpBoard { inner: self.inner.board().clone() }
    }

    /// Tracked promoted pieces (empty except Crazyhouse).
    pub fn promoted(&self) -> PhpBitboard {
        PhpBitboard { inner: self.inner.promoted() }
    }

    /// Crazyhouse pockets, or `null` when the variant has none.
    pub fn pockets(&self) -> Option<PhpPockets> {
        self.inner.pockets().map(|p| PhpPockets { inner: *p })
    }

    /// Remaining checks for Three-Check for `color` (1 = White, 0 = Black), or `null`.
    pub fn remaining_checks(&self, color: i32) -> Option<PhpRemainingChecks> {
        self.inner
            .remaining_checks()
            .map(|rc| PhpRemainingChecks { inner: *rc.get(color_from_int(color)) })
    }

    /// Side to move (1 = White, 0 = Black).
    pub fn turn(&self) -> i32 {
        match self.inner.turn() {
            Color::White => 1,
            Color::Black => 0,
        }
    }

    /// Castling rights as a bitboard.
    pub fn castling_rights(&self) -> PhpBitboard {
        PhpBitboard { inner: self.inner.castles().castling_rights() }
    }

    /// En passant target square after a double pawn push, unconditionally, or `null`.
    pub fn maybe_ep_square(&self) -> Option<i32> {
        self.inner.maybe_ep_square().map(|s| s as i32)
    }

    /// Half-move clock since the last capture or pawn move.
    pub fn halfmoves(&self) -> i32 {
        self.inner.halfmoves() as i32
    }

    /// Move number (starts at 1).
    pub fn fullmoves(&self) -> i32 {
        self.inner.fullmoves().get() as i32
    }

    /// Generates all legal moves.
    pub fn legal_moves(&self) -> PhpMoveList {
        moves_to_php(self.inner.legal_moves())
    }

    /// Whether the game is over due to a variant-specific end condition.
    pub fn is_variant_end(&self) -> bool {
        self.inner.is_variant_end()
    }

    /// Whether `color` (1 = White, 0 = Black) has insufficient winning material.
    pub fn has_insufficient_material(&self, color: i32) -> bool {
        self.inner.has_insufficient_material(color_from_int(color))
    }

    /// Special variant outcome as a string.
    pub fn variant_outcome(&self) -> String {
        self.inner.variant_outcome().as_str().to_string()
    }

    /// Plays a move without legality checks (mutates the position).
    pub fn play_unchecked(&mut self, m: &PhpMove) {
        self.inner.play_unchecked(m.inner);
    }

    // === Position contract: provided (default) methods ===

    /// Squares occupied by the side to move.
    pub fn us(&self) -> PhpBitboard {
        PhpBitboard { inner: self.inner.us() }
    }

    /// Squares occupied by `role` of the side to move.
    pub fn our(&self, role: i32) -> PhpBitboard {
        match role_from_int(role) {
            Some(r) => PhpBitboard { inner: self.inner.our(r) },
            None => empty_bitboard(),
        }
    }

    /// Squares occupied by the opponent.
    pub fn them(&self) -> PhpBitboard {
        PhpBitboard { inner: self.inner.them() }
    }

    /// Squares occupied by `role` of the opponent.
    pub fn their(&self, role: i32) -> PhpBitboard {
        match role_from_int(role) {
            Some(r) => PhpBitboard { inner: self.inner.their(r) },
            None => empty_bitboard(),
        }
    }

    /// Bitboard of pieces giving check.
    pub fn checkers(&self) -> PhpBitboard {
        PhpBitboard { inner: self.inner.checkers() }
    }

    /// Whether the side to move is in check.
    pub fn is_check(&self) -> bool {
        self.inner.is_check()
    }

    /// Whether the side to move is checkmated.
    pub fn is_checkmate(&self) -> bool {
        self.inner.is_checkmate()
    }

    /// Whether the position is a stalemate.
    pub fn is_stalemate(&self) -> bool {
        self.inner.is_stalemate()
    }

    /// Whether both sides have insufficient winning material.
    pub fn is_insufficient_material(&self) -> bool {
        self.inner.is_insufficient_material()
    }

    /// Whether the game is over.
    pub fn is_game_over(&self) -> bool {
        self.inner.is_game_over()
    }

    /// Game outcome as a string: `"1-0"`, `"0-1"`, `"1/2-1/2"`, or `"*"`.
    pub fn outcome(&self) -> String {
        self.inner.outcome().as_str().to_string()
    }

    /// En passant square for the given mode (0 = Legal [default], 1 = PseudoLegal, 2 = Always), or `null`.
    pub fn ep_square(&self, mode: Option<i32>) -> Option<i32> {
        let mode = mode.map(ep_mode_from_int).unwrap_or(EnPassantMode::Legal);
        self.inner.ep_square(mode).map(|s| s as i32)
    }

    /// En passant square if it is the target of a pseudo-legal en passant move.
    pub fn pseudo_legal_ep_square(&self) -> Option<i32> {
        self.inner.pseudo_legal_ep_square().map(|s| s as i32)
    }

    /// En passant square if it is the target of a legal en passant move.
    pub fn legal_ep_square(&self) -> Option<i32> {
        self.inner.legal_ep_square().map(|s| s as i32)
    }

    /// Generates capture moves.
    pub fn capture_moves(&self) -> PhpMoveList {
        moves_to_php(self.inner.capture_moves())
    }

    /// Generates promotion moves.
    pub fn promotion_moves(&self) -> PhpMoveList {
        moves_to_php(self.inner.promotion_moves())
    }

    /// Generates en passant moves.
    pub fn en_passant_moves(&self) -> PhpMoveList {
        moves_to_php(self.inner.en_passant_moves())
    }

    /// Generates castling moves for the given side (0 = king-side, 1 = queen-side).
    pub fn castling_moves(&self, side: i32) -> PhpMoveList {
        moves_to_php(self.inner.castling_moves(side_from_int(side)))
    }

    /// Generates SAN candidate moves of `role` to square `to`.
    pub fn san_candidates(&self, role: i32, to: i32) -> PhpMoveList {
        match (role_from_int(role), square_from_int(to)) {
            (Some(r), Some(sq)) => moves_to_php(self.inner.san_candidates(r, sq)),
            _ => moves_to_php(MoveList::new()),
        }
    }

    /// Attacks that a king on `square` would have to deal with.
    pub fn king_attackers(&self, square: i32, attacker: i32, occupied: &PhpBitboard) -> PhpBitboard {
        match square_from_int(square) {
            Some(sq) => PhpBitboard {
                inner: self.inner.king_attackers(sq, color_from_int(attacker), occupied.inner),
            },
            None => empty_bitboard(),
        }
    }

    /// Whether a move is irreversible.
    pub fn is_irreversible(&self, m: &PhpMove) -> bool {
        self.inner.is_irreversible(m.inner)
    }

    /// Tests a move for legality.
    pub fn is_legal(&self, m: &PhpMove) -> bool {
        self.inner.is_legal(m.inner)
    }

    /// Plays a legal move, returning the resulting `VariantPosition`, or throws on an illegal move.
    pub fn play(&self, m: &PhpMove) -> Result<PhpVariantPosition, &'static str> {
        self.inner.clone().play(m.inner)
            .map(|pos| PhpVariantPosition { inner: pos })
            .map_err(|_| "Illegal move")
    }

    /// Converts the position to a `Setup` for the given en passant mode (0 = Legal, 1 = PseudoLegal, 2 = Always).
    pub fn to_setup(&self, mode: i32) -> PhpSetup {
        PhpSetup { inner: self.inner.to_setup(ep_mode_from_int(mode)) }
    }

    // === VariantPosition-specific helpers ===

    /// Converts the current position to a FEN string.
    pub fn to_fen(&self) -> String {
        Fen::from_position(&self.inner, EnPassantMode::Legal).to_string()
    }

    /// Returns the number of legal moves available to the side to move.
    pub fn legal_moves_count(&self) -> i32 {
        self.inner.legal_moves().len() as i32
    }

    /// Plays a move given as SAN notation (mutates the position). Throws on invalid/illegal.
    pub fn play_san(&mut self, san: String) -> Result<bool, &'static str> {
        let parsed: San = san.parse().map_err(|_| "Invalid SAN")?;
        let m = parsed.to_move(&self.inner).map_err(|_| "Illegal move")?;
        self.inner = self.inner.clone().play(m).map_err(|_| "Play failed")?;
        Ok(true)
    }

    /// Plays a move given as UCI notation (mutates the position). Throws on invalid/illegal.
    pub fn play_uci(&mut self, uci: String) -> Result<bool, &'static str> {
        let parsed = UciMove::from_ascii(uci.as_bytes()).map_err(|_| "Invalid UCI")?;
        let m = parsed.to_move(&self.inner).map_err(|_| "Illegal move")?;
        self.inner = self.inner.clone().play(m).map_err(|_| "Play failed")?;
        Ok(true)
    }

    // === Zobrist hash (64-bit) ===

    /// Computes the 64-bit Zobrist hash of the position (excludes halfmove/fullmove counters).
    pub fn zobrist_hash(&self, mode: Option<i32>) -> PhpZobrist64 {
        let mode = mode.map(ep_mode_from_int).unwrap_or(EnPassantMode::Legal);
        PhpZobrist64 { inner: self.inner.zobrist_hash::<Zobrist64>(mode) }
    }

    /// Incrementally updates a 64-bit Zobrist hash after legal move `m`, or `null` when unsupported.
    pub fn update_zobrist_hash(
        &self,
        current: &PhpZobrist64,
        m: &PhpMove,
        mode: Option<i32>,
    ) -> Option<PhpZobrist64> {
        let mode = mode.map(ep_mode_from_int).unwrap_or(EnPassantMode::Legal);
        self.inner
            .update_zobrist_hash::<Zobrist64>(current.inner, m.inner, mode)
            .map(|z| PhpZobrist64 { inner: z })
    }
}
