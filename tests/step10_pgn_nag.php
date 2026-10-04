<?php
/** Step 10: shakmaty\pgn\Nag */
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

// Constants
assert(\shakmaty\pgn\Nag::GOOD_MOVE === 1);
assert(\shakmaty\pgn\Nag::MISTAKE === 2);
assert(\shakmaty\pgn\Nag::BRILLIANT_MOVE === 3);
assert(\shakmaty\pgn\Nag::BLUNDER === 4);
assert(\shakmaty\pgn\Nag::SPECULATIVE_MOVE === 5);
assert(\shakmaty\pgn\Nag::DUBIOUS_MOVE === 6);

// Constructor, value property, __toString(), glyph()
$blunder = new \shakmaty\pgn\Nag(\shakmaty\pgn\Nag::BLUNDER);
assert($blunder->value === 4);
assert($blunder->__toString() === '$4');
assert($blunder->glyph() === '??');

$plain = new \shakmaty\pgn\Nag(42);
assert($plain->value === 42);
assert($plain->__toString() === '$42');
assert($plain->glyph() === null);

// Clamping
assert((new \shakmaty\pgn\Nag(-5))->value === 0);
assert((new \shakmaty\pgn\Nag(1000))->value === 255);

// fromAscii()
assert(\shakmaty\pgn\Nag::fromAscii('??')->value === 4);
assert(\shakmaty\pgn\Nag::fromAscii('$24')->value === 24);
assert(\shakmaty\pgn\Nag::fromAscii('$1')->value === 1);
assert(\shakmaty\pgn\Nag::fromAscii('xyz') === null);

// read-only property: direct assignment must throw
$threw = false;
try { $blunder->value = 0; } catch (\Throwable $e) { $threw = true; }
assert($threw, "writing the read-only 'value' property must throw");

echo "Step 10 (pgn): Nag tests passed!\n";
