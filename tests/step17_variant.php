<?php
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

// === Constants: all eight discriminants 0..7 ===
assert(\shakmaty\Variant::CHESS === 0);
assert(\shakmaty\Variant::ATOMIC === 1);
assert(\shakmaty\Variant::ANTICHESS === 2);
assert(\shakmaty\Variant::KING_OF_THE_HILL === 3);
assert(\shakmaty\Variant::THREE_CHECK === 4);
assert(\shakmaty\Variant::CRAZYHOUSE === 5);
assert(\shakmaty\Variant::RACING_KINGS === 6);
assert(\shakmaty\Variant::HORDE === 7);

// === Constructor + value/uci/distinguishesPromoted (Chess) ===
$chess = new \shakmaty\Variant(\shakmaty\Variant::CHESS);
assert($chess->value() === 0);
assert($chess->uci() === 'chess');
assert($chess->distinguishesPromoted() === false);

// === Crazyhouse: uci + distinguishesPromoted (true) ===
$crazy = new \shakmaty\Variant(\shakmaty\Variant::CRAZYHOUSE);
assert($crazy->uci() === 'crazyhouse');
assert($crazy->distinguishesPromoted() === true);

// === fromUci: exact UCI names, null for unknown ===
$atomic = \shakmaty\Variant::fromUci('atomic');
assert($atomic !== null && $atomic->value() === \shakmaty\Variant::ATOMIC);
$threeCheck = \shakmaty\Variant::fromUci('3check');
assert($threeCheck !== null && $threeCheck->value() === \shakmaty\Variant::THREE_CHECK);
assert(\shakmaty\Variant::fromUci('nope') === null);

// === fromAscii: names + aliases, null for unknown ===
$chess960 = \shakmaty\Variant::fromAscii('Chess960');
assert($chess960 !== null && $chess960->value() === \shakmaty\Variant::CHESS);
$koth = \shakmaty\Variant::fromAscii('King of the Hill');
assert($koth !== null && $koth->value() === \shakmaty\Variant::KING_OF_THE_HILL);
$horde = \shakmaty\Variant::fromAscii('Horde');
assert($horde !== null && $horde->value() === \shakmaty\Variant::HORDE);
assert(\shakmaty\Variant::fromAscii('nope') === null);

// === __toString alias of uci ===
$chess2 = new \shakmaty\Variant(\shakmaty\Variant::CHESS);
assert($chess2->__toString() === 'chess');

// === all(): returns the eight variants as an array ===
assert(count(\shakmaty\Variant::all()) === 8);

echo "Step 17: All Variant tests passed!\n";
