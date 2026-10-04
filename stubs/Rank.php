<?php
namespace shakmaty;

/**
 * Represents a chessboard rank (row) from First (0) to Eighth (7).
 *
 * @method int value() Returns the rank index (0..7).
 * @method string toChar() Returns digit character (1..8).
 * @method int distance(self $other) Returns absolute rank distance.
 * @method self flipVertical() Returns the vertically mirrored rank.
 * @method self|null fromChar(string $ch) Creates from character (1-8).
 */
final class Rank {
    const int FIRST = 0; const int SECOND = 1; const int THIRD = 2; const int FOURTH = 3;
    const int FIFTH = 4; const int SIXTH = 5; const int SEVENTH = 6; const int EIGHTH = 7;
    public function __construct(int $value) {}
}
