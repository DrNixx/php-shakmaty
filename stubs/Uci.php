<?php
namespace shakmaty;

/**
 * Wrapper for a Universal Chess Interface (UCI) move string.
 *
 * @method string __toString() Returns the UCI string (PHP magic __toString).
 * @method bool isValid() Validates the UCI move string.
 */
final class Uci {
    public function __construct(string $uci) {}
}
