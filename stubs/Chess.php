<?php
namespace shakmaty;

/**
 * Main chess position class. Extends the abstract {@see Position} base class and
 * adds FEN/SAN/UCI helpers.
 *
 * Inherits the full {@see Position} contract (board, turn, legalMoves, isCheck,
 * isCheckmate, outcome, play, us, them, captureMoves, ...).
 *
 * @method static self fromFen(string $fen) Creates a position from a FEN string. Throws on invalid.
 * @method string toFen() Returns the FEN string of the current position.
 * @method int legalMovesCount() Returns the number of legal moves.
 * @method bool playSan(string $san) Plays a move in SAN notation. Throws on invalid/illegal.
 * @method bool playUci(string $uci) Plays a move in UCI notation. Throws on invalid/illegal.
 */
class Chess extends Position {
    public function __construct() {}
}
