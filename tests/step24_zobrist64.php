<?php
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

use shakmaty\Zobrist64;

// --- construction and representation ---
$zero = new Zobrist64();
assert($zero->value === 0);
assert($zero->isZero() === true);
assert($zero->toHex() === '0000000000000000');
assert($zero->__toString() === '0000000000000000');

$h = Zobrist64::fromHex('463b96181691fc9c');
assert($h instanceof Zobrist64);
assert($h->isZero() === false);
assert($h->toHex() === '463b96181691fc9c');
assert($h->value === 0x463b96181691fc9c);

// optional 0x prefix and trim
assert($h->equals(Zobrist64::fromHex('0x463b96181691fc9c')) === true);
assert($h->equals(Zobrist64::fromHex('  463B96181691FC9C  ')) === true);

// constructor from raw bit pattern round-trips through hex
$raw = new Zobrist64($h->value);
assert($raw->toHex() === '463b96181691fc9c');

// top-bit-set hashes stay exact through hex
$big = Zobrist64::fromHex('ffffffffffffffff');
assert($big->toHex() === 'ffffffffffffffff');
assert($big->value === -1); // signed bit pattern

// --- xor / equality ---
assert($zero->bitwiseXor($h)->equals($h) === true);
assert($h->bitwiseXor($h)->isZero() === true);
assert($h->equals($zero) === false);

// --- invalid hex throws ---
$threw = false;
try { Zobrist64::fromHex('zzzz'); } catch (\Throwable $e) { $threw = true; }
assert($threw === true);

// read-only property: direct assignment must throw
$threw = false;
try { $h->value = 0; } catch (\Throwable $e) { $threw = true; }
assert($threw === true, "writing the read-only 'value' property must throw");

echo "Step 24: All Zobrist64 value-type tests passed!\n";
