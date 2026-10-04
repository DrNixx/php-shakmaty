<?php
namespace shakmaty;

/**
 * Represents a colored chess piece (color + role).
 *
 * @property-read Color $color Piece color (White or Black).
 * @property-read Role $role Piece role (Pawn through King).
 * @method string toChar() Returns character (K/Q/R/B/N/P/k/q/r/b/n/p).
 * @method string toUnicode() Returns Unicode chess symbol (♔♕♖♗♘♙♚♛♜♝♞♟).
 * @method self|null fromChar(string $ch) Creates piece from character.
 */
final class Piece {
    public function __construct(int $color, int $role) {}
}
