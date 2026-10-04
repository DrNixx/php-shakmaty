<?php
namespace shakmaty;

/**
 * Represents a set of squares using a 64-bit integer bitboard.
 *
 * @method static self fromSquare(int $sq) Creates bitboard with a single square set.
 * @method static self fromRank(int $rank) Creates bitboard representing a rank.
 * @method static self fromFile(int $file) Creates bitboard representing a file.
 * @method int count() Returns the number of set bits (population count).
 * @method bool any() Returns true if any bit is set.
 * @method bool isEmpty() Returns true if no bits are set.
 * @method bool has(int $squareIndex) Returns true if the square is set.
 * @method int toU64() Returns the raw 64-bit value as a signed integer.
 * @method string __toString() Returns 64-character binary string (PHP magic __toString).
 * @method self bitwiseAnd(self $other) Bitwise AND.
 * @method self bitwiseOr(self $other) Bitwise OR.
 * @method self bitwiseXor(self $other) Bitwise XOR.
 * @method self bitwiseNot() Bitwise NOT (complement).
 * @method self shift(int $offset) Shift bits by offset (positive = toward black side).
 */
final class Bitboard {
    const int EMPTY = 0;
    const int ALL = -1;
    const int CORNERS = -9151314442816847871; // 0x8100000000000081
    const int BACKRANKS = -71777214294589697;  // 0xFF000000000000FF
    const int LIGHT_SQUARES = 6162805196077451090; // 0x55AA55AA55AA55AA
    const int DARK_SQUARES = -6433039863573779531; // 0xAA55AA55AA55AA55
    public function __construct(?int $rawValue = 0) {}
}
