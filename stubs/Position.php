<?php
namespace shakmaty;

/**
 * Abstract base class mirroring the Rust `shakmaty::Position` trait.
 *
 * Concrete position classes (e.g. Chess) extend it. It cannot be instantiated
 * directly — use `$pos instanceof Position` for type checks.
 *
 * @method Board board() Piece positions on the board.
 * @method Bitboard promoted() Tracked promoted pieces (empty for standard chess).
 * @method Pockets|null pockets() Crazyhouse pockets, or null.
 * @method RemainingChecks|null remainingChecks(int $color) Remaining checks for a color (1=White, 0=Black), or null.
 * @method Zobrist64 zobristHash(int|null $mode = null) Computes the 64-bit Zobrist hash (excludes move counters).
 * @method Zobrist64|null updateZobristHash(Zobrist64 $current, Move $m, int|null $mode = null) Incrementally updates a hash after a move, or null if unsupported.
 * @method int turn() Side to move: 1=White, 0=Black.
 * @method Bitboard castlingRights() Castling rights as a bitboard.
 * @method int|null maybeEpSquare() En passant target square after a double pawn push, or null.
 * @method int halfmoves() Half-move clock since last capture/pawn move.
 * @method int fullmoves() Full-move number (starts at 1).
 * @method MoveList legalMoves() All legal moves.
 * @method bool isVariantEnd() Variant-specific end condition (false for standard chess).
 * @method bool hasInsufficientMaterial(int $color) Whether a side has insufficient winning material.
 * @method string variantOutcome() Variant outcome ("*" for standard chess).
 * @method void playUnchecked(Move $m) Plays a move without legality checks.
 * @method Bitboard us() Squares occupied by the side to move.
 * @method Bitboard our(int $role) Squares occupied by a role of the side to move.
 * @method Bitboard them() Squares occupied by the opponent.
 * @method Bitboard their(int $role) Squares occupied by a role of the opponent.
 * @method Bitboard checkers() Pieces giving check.
 * @method bool isCheck() Whether the side to move is in check.
 * @method bool isCheckmate() Whether the position is checkmate.
 * @method bool isStalemate() Whether the position is stalemate.
 * @method bool isInsufficientMaterial() Whether both sides have insufficient material.
 * @method bool isGameOver() Whether the game is over.
 * @method string outcome() Game outcome ("1-0", "0-1", "1/2-1/2", "*").
 * @method int|null epSquare(int|null $mode = null) En passant square for the given mode (0=Legal, 1=PseudoLegal, 2=Always).
 * @method int|null pseudoLegalEpSquare() En passant square for a pseudo-legal capture.
 * @method int|null legalEpSquare() En passant square for a legal capture.
 * @method MoveList captureMoves() Capture moves.
 * @method MoveList promotionMoves() Promotion moves.
 * @method MoveList enPassantMoves() En passant moves.
 * @method MoveList castlingMoves(int $side) Castling moves (0=king-side, 1=queen-side).
 * @method MoveList sanCandidates(int $role, int $to) SAN candidate moves.
 * @method Bitboard kingAttackers(int $square, int $attacker, Bitboard $occupied) Attackers of a king square.
 * @method bool isIrreversible(Move $m) Whether a move is irreversible.
 * @method bool isLegal(Move $m) Whether a move is legal.
 * @method Chess play(Move $m) Plays a legal move, returning a new position. Throws if illegal.
 * @method Setup toSetup(int $mode) Converts the position to a Setup.
 */
abstract class Position {
    protected function __construct() {}
}
