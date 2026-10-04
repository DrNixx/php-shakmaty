<?php
namespace shakmaty;

/**
 * Represents a chess move. Typically obtained via Chess::legalMoves().
 *
 * @method int role() Returns the moving piece role (1..6).
 * @method int|null fromSq() Returns source square index.
 * @method int toSq() Returns target square index.
 * @method int|null capture() Returns captured piece role or null.
 * @method bool isCapture() Returns true if move captures.
 * @method bool isCastle() Returns true if move is castling.
 * @method bool isEnPassant() Returns true if move is en passant.
 * @method bool isPromotion() Returns true if move is a promotion.
 * @method int|null promotion() Returns promotion role or null.
 * @method string toLong() Returns long algebraic notation (e.g., "e2e4"); Crazyhouse drops render as "N@f3" / "@e4".
 * @method bool isPut() Returns true if this is a Crazyhouse drop (e.g. "N@f3").
 * @method static Move fromPut(int $role, int $to) Creates a Crazyhouse drop move (throws on invalid role/square).
 */
final class Move {
    public function __construct() {}
}
