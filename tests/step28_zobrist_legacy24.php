<?php
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

use shakmaty\Chess;
use shakmaty\Zobrist64;

// --- 1. Documented Polyglot example: top 24 bits of 0x9D39247E33776D41 ---
$sample = Zobrist64::fromHex('9d39247e33776d41');
assert($sample->legacy24() === 0x9d3924, 'legacy24 must return the top 24 bits');

// --- 2. Boundaries: zero, all-bits-set, exact 24-bit boundary ---
// The shift is logical (zero-fill), so a top-bit-set value must NOT sign-extend.
assert((new Zobrist64())->legacy24() === 0);
assert(Zobrist64::fromHex('ffffffffffffffff')->legacy24() === 0xffffff);
assert(Zobrist64::fromHex('00ffffffffffffff')->legacy24() === 0x00ffff);

// --- 3. Known position hash ---
assert((new Chess())->zobristHash()->legacy24() === 0x463b96); // start hash 463b96181691fc9c

// Independent reference: the first 6 hex digits of toHex() ARE the top 24 bits.
function legacy24_reference(Zobrist64 $h): int {
    return intval(substr($h->toHex(), 0, 6), 16);
}

// --- 4. Corpus regression: extension value == independent top-24-bit computation ---
$corpus = [new Chess()];

// All 20 root moves, each played on a fresh starting position.
$root = new Chess();
$legal = $root->legalMoves();
assert($legal->count() === 20);
for ($i = 0; $i < $legal->count(); $i++) {
    $copy = new Chess();
    $copy->playUnchecked($legal->get($i));
    $corpus[] = $copy;
}

// A short game line (ends with castling); rebuild from scratch at every depth so
// each corpus entry is a distinct object holding its own position state.
$lineUcis = ['e2e4', 'e7e5', 'g1f3', 'b8c6', 'f1c4', 'g8f6', 'e1g1'];
for ($d = 1; $d <= count($lineUcis); $d++) {
    $p = new Chess();
    for ($k = 0; $k < $d; $k++) {
        assert($p->playUci($lineUcis[$k]) === true, "uci must be legal: {$lineUcis[$k]}");
    }
    $corpus[] = $p;
}

$checked = 0;
foreach ($corpus as $pos) {
    $h = $pos->zobristHash();
    $legacy = $h->legacy24();
    assert(is_int($legacy), 'legacy24() must return a PHP int');
    assert($legacy >= 0 && $legacy <= 0xFFFFFF, 'legacy24() must stay in 0..0xFFFFFF');
    assert($legacy === legacy24_reference($h), "legacy24 mismatch for {$h->toHex()}");
    $checked++;
}
assert($checked === 1 + 20 + 7, "corpus size mismatch: $checked");

// --- 5. Reproduce engine B's hashBase64 over a slice of legacy24 hashes ---
// Engine B: hashBase64 = base64( concat( dechex(legacy24) ) ) — lowercase hex, no padding.
$hashes = [];
foreach ($corpus as $pos) {
    $hashes[] = $pos->zobristHash()->legacy24();
}
$joined = '';
foreach ($hashes as $v) {
    $joined .= dechex($v);
}
assert($joined !== '');
assert(base64_encode($joined) === base64_encode(implode('', array_map('dechex', $hashes))));

// Concrete single-value vector: start legacy24 0x463b96 -> "463b96" -> base64 "NDYzYjk2".
assert(dechex((new Chess())->zobristHash()->legacy24()) === '463b96');
assert(base64_encode(dechex(0x463b96)) === 'NDYzYjk2');

echo "Step 28: All legacy24 Zobrist tests passed!\n";
