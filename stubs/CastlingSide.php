<?php
namespace shakmaty;

/**
 * Represents a castling side: King side or Queen side.
 *
 * @method int value() Returns 0 for KingSide, 1 for QueenSide.
 * @method bool isKingSide() Returns true if king side.
 * @method bool isQueenSide() Returns true if queen side.
 * @method self other() Returns the opposite castling side.
 */
final class CastlingSide {
    const int KING_SIDE = 0;
    const int QUEEN_SIDE = 1;
    public function __construct(int $value) {}
}
