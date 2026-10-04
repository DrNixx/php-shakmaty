<?php
namespace shakmaty;

/**
 * Represents a chess position setup (not necessarily legal).
 *
 * State is exposed as PHP properties: scalar fields are writable, object-valued
 * fields are read-only (mutate them through methods).
 *
 * @property Board $board Read-only board. Mutate via setBoard().
 * @property int $turn Writable side to move: 1=White, 0=Black.
 * @property Bitboard $castlingRights Read-only castling rights. Mutate via setCastlingRights().
 * @property int|null $epSquare Writable en passant square index, or null.
 * @property int $halfmoves Writable half-move clock.
 * @property int $fullmoves Writable full-move number.
 * @property Bitboard $promoted Read-only tracked promoted pieces (Crazyhouse). Mutate via setPromoted().
 * @property Pockets|null $pockets Read-only Crazyhouse pockets, or null. Mutate via setPockets()/clearPockets().
 *
 * @method static self empty() Creates an empty setup.
 * @method static Setup fromFen(string $fen) Parses a FEN/EPD string into a Setup without any legality check (the result may be illegal). Throws a shakmaty\fen\ParseFenError subclass on invalid input.
 * @method void setBoard(Board $board) Sets the board.
 * @method void setCastlingRights(Bitboard $bb) Sets castling rights.
 * @method void setPromoted(Bitboard $bb) Sets the promoted-pieces bitboard.
 * @method void setPockets(Pockets $pockets) Sets Crazyhouse pockets.
 * @method void clearPockets() Clears Crazyhouse pockets.
 * @method RemainingChecks|null remainingChecks(int $color) Remaining checks for a color (1=White, 0=Black), or null.
 * @method void setRemainingChecks(int $color, RemainingChecks $checks) Sets remaining checks for a color.
 * @method void clearRemainingChecks() Clears remaining checks.
 */
final class Setup {
    public function __construct() {}
}
