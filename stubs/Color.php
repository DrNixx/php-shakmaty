<?php
namespace shakmaty;

/**
 * Represents a chess piece color: White or Black.
 *
 * @method int value() Returns 1 for White, 0 for Black.
 * @method bool isWhite() Returns true if the color is White.
 * @method bool isBlack() Returns true if the color is Black.
 * @method self other() Returns the opposite color.
 * @method string toChar() Returns "w" for White, "b" for Black.
 */
final class Color {
    const int WHITE = 1;
    const int BLACK = 0;
    public function __construct(int $value) {}
}
