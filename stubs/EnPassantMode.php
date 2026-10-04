<?php
namespace shakmaty;

/**
 * Represents an en passant mode: Legal, PseudoLegal, or Always.
 *
 * @method int value() Returns the mode value (0, 1, or 2).
 */
final class EnPassantMode {
    const int LEGAL = 0;
    const int PSEUDO_LEGAL = 1;
    const int ALWAYS = 2;
    public function __construct(int $value) {}
}
