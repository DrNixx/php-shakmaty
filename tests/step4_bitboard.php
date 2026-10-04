<?php
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

// Empty bitboard
$empty = new \shakmaty\Bitboard();
assert($empty->isEmpty());
assert(!$empty->any());
assert($empty->count() === 0);
assert($empty->__toString() === str_repeat('0', 64));

// From square
$bb = \shakmaty\Bitboard::fromSquare(\shakmaty\Square::E4);
assert($bb->has(\shakmaty\Square::E4));
assert(!$bb->has(\shakmaty\Square::A1));
assert($bb->count() === 1);

// From rank
$rank1 = \shakmaty\Bitboard::fromRank(\shakmaty\Rank::FIRST);
assert($rank1->count() === 8);
assert($rank1->has(\shakmaty\Square::A1));
assert($rank1->has(\shakmaty\Square::H1));
assert(!$rank1->has(\shakmaty\Square::A2));

// From file
$file_a = \shakmaty\Bitboard::fromFile(\shakmaty\File::A);
assert($file_a->count() === 8);
assert($file_a->has(\shakmaty\Square::A1));
assert($file_a->has(\shakmaty\Square::A8));

// Bitwise operations
$bb1 = \shakmaty\Bitboard::fromSquare(\shakmaty\Square::E4);
$bb2 = \shakmaty\Bitboard::fromSquare(\shakmaty\Square::E5);

$or_bb = $bb1->bitwiseOr($bb2);
assert($or_bb->count() === 2);
assert($or_bb->has(\shakmaty\Square::E4));
assert($or_bb->has(\shakmaty\Square::E5));

$and_bb = $bb1->bitwiseAnd($bb2);
assert($and_bb->isEmpty());

$xor_bb = $bb1->bitwiseXor($bb2);
assert($xor_bb->count() === 2);

$not_bb = $bb1->bitwiseNot();
assert($not_bb->has(\shakmaty\Square::A1));
assert(!$not_bb->has(\shakmaty\Square::E4));

// Shift
$bb_shift = $bb1->shift(8);
assert($bb_shift->has(\shakmaty\Square::E5));
assert(!$bb_shift->has(\shakmaty\Square::E4));

// toU64
$val = $bb->toU64();
assert($val > 0);

// Constants
assert((new \shakmaty\Bitboard(\shakmaty\Bitboard::EMPTY))->isEmpty());
$all = new \shakmaty\Bitboard(\shakmaty\Bitboard::ALL);
assert($all->count() === 64);

$corners = new \shakmaty\Bitboard(\shakmaty\Bitboard::CORNERS);
assert($corners->has(\shakmaty\Square::A1));
assert($corners->has(\shakmaty\Square::H1));
assert($corners->has(\shakmaty\Square::A8));
assert($corners->has(\shakmaty\Square::H8));
assert($corners->count() === 4);

$backranks = new \shakmaty\Bitboard(\shakmaty\Bitboard::BACKRANKS);
assert($backranks->count() === 16);

echo "Step 4: All bitboard tests passed!\n";
