<?php
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

use shakmaty\RemainingChecks;

assert(RemainingChecks::MAX === 3);

$def = new RemainingChecks();
assert($def->value() === 3);

assert((new RemainingChecks(0))->value() === 0);
assert((new RemainingChecks(2))->value() === 2);
assert((new RemainingChecks(99))->value() === 3);  // clamped to MAX
assert((new RemainingChecks(-1))->value() === 0);  // clamped to 0

assert((new RemainingChecks(0))->isZero() === true);
assert((new RemainingChecks(1))->isZero() === false);

assert((new RemainingChecks(3))->saturatingSub(2)->value() === 1);
assert((new RemainingChecks(1))->saturatingSub(5)->value() === 0);
assert((new RemainingChecks(1))->saturatingSub(-3)->value() === 1); // negative n treated as 0

echo "Step 19: All RemainingChecks tests passed!\n";
