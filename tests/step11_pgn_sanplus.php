<?php
/** Step 11: shakmaty\SanPlus (root namespace) */
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

// Plain move
$san = \shakmaty\SanPlus::fromAscii('Nf3');
assert($san->__toString() === 'Nf3');
assert($san->san() === 'Nf3');
assert($san->suffix() === null);
assert($san->isCheck() === false);
assert($san->isCheckmate() === false);
assert($san->isNull() === false);

// Check
$check = \shakmaty\SanPlus::fromAscii('Qh5+');
assert($check->__toString() === 'Qh5+');
assert($check->san() === 'Qh5');
assert($check->suffix() === '+');
assert($check->isCheck() === true);
assert($check->isCheckmate() === false);

// Checkmate
$mate = \shakmaty\SanPlus::fromAscii('Qxf7#');
assert($mate->__toString() === 'Qxf7#');
assert($mate->san() === 'Qxf7');
assert($mate->suffix() === '#');
assert($mate->isCheckmate() === true);

// Castling
$castle = \shakmaty\SanPlus::fromAscii('O-O');
assert($castle->__toString() === 'O-O');
assert($castle->san() === 'O-O');

// Null move
$null = \shakmaty\SanPlus::fromAscii('--');
assert($null->isNull() === true);
assert($null->__toString() === '--');

// toSan() returns a shakmaty\San instance
$wrapper = $san->toSan();
assert($wrapper instanceof \shakmaty\San);
assert($wrapper->__toString() === 'Nf3');

// Old pgn-namespace name must no longer exist
assert(class_exists('shakmaty\\pgn\\SanPlus') === false);

// Invalid input throws
$threw = false;
try {
    \shakmaty\SanPlus::fromAscii('xyz');
} catch (\Throwable $e) {
    $threw = true;
}
assert($threw === true);

echo "Step 11 (pgn): SanPlus tests passed!\n";
