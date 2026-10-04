<?php
namespace shakmaty;

/**
 * Represents a chess piece role: Pawn, Knight, Bishop, Rook, Queen, or King.
 *
 * @method int value() Returns the role value (1..6).
 * @method string toChar() Returns lowercase character (p/n/b/r/q/k).
 * @method string upperChar() Returns uppercase character (P/N/B/R/Q/K).
 * @method self|null fromChar(string $ch) Creates role from character.
 */
final class Role {
    const int PAWN = 1;
    const int KNIGHT = 2;
    const int BISHOP = 3;
    const int ROOK = 4;
    const int QUEEN = 5;
    const int KING = 6;
    public function __construct(int $value) {}
}
