<?php
namespace shakmaty;

/**
 * Represents a castling mode: Standard or Chess960.
 *
 * @method int value() Returns 0 for Standard, 1 for Chess960.
 * @method bool isStandard() Returns true if standard castling.
 * @method bool isChess960() Returns true if Chess960 castling.
 */
final class CastlingMode {
    const int STANDARD = 0;
    const int CHESS960 = 1;
    public function __construct(int $value) {}
}
