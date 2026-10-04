<?php
namespace shakmaty;

/**
 * Number of checks a side still needs to give to win a game of Three-Check
 * (an integer in 0..=3, default 3). Mirrors `shakmaty::RemainingChecks`.
 *
 * @method int value() Returns the number of remaining checks (0..=3).
 * @method bool isZero() Whether the side has no checks left.
 * @method RemainingChecks saturatingSub(int $n) Returns a copy with n checks subtracted (saturating at zero).
 */
class RemainingChecks {
    public const MAX = 3;

    public function __construct(?int $value = null) {}
}
