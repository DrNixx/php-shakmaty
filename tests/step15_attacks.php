<?php
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

// --- Pawn attacks ---
$wp = \shakmaty\Attacks::pawnAttacks(\shakmaty\Color::WHITE, \shakmaty\Square::E2);
assert($wp->count() === 2);
assert($wp->has(\shakmaty\Square::D3));
assert($wp->has(\shakmaty\Square::F3));

$bp = \shakmaty\Attacks::pawnAttacks(\shakmaty\Color::BLACK, \shakmaty\Square::E7);
assert($bp->count() === 2);
assert($bp->has(\shakmaty\Square::D6));
assert($bp->has(\shakmaty\Square::F6));

// --- Knight attacks ---
$kn = \shakmaty\Attacks::knightAttacks(\shakmaty\Square::G1);
assert($kn->count() === 3);
assert($kn->has(\shakmaty\Square::E2));
assert($kn->has(\shakmaty\Square::F3));
assert($kn->has(\shakmaty\Square::H3));

// --- King attacks ---
$kg = \shakmaty\Attacks::kingAttacks(\shakmaty\Square::E1);
assert($kg->count() === 5);
assert($kg->has(\shakmaty\Square::D1));
assert($kg->has(\shakmaty\Square::D2));
assert($kg->has(\shakmaty\Square::E2));
assert($kg->has(\shakmaty\Square::F1));
assert($kg->has(\shakmaty\Square::F2));

// --- Bishop / rook / queen vs. documented magic example ---
$occupied = new \shakmaty\Bitboard(0x3f7f28802826f5b9);

$bishop = \shakmaty\Attacks::bishopAttacks(\shakmaty\Square::D6, $occupied);
assert($bishop->toU64() === 0x0014001422010000);

$rook = \shakmaty\Attacks::rookAttacks(\shakmaty\Square::D6, $occupied);
assert($rook->toU64() === 0x0008370808000000);

$queen = \shakmaty\Attacks::queenAttacks(\shakmaty\Square::D6, $occupied);
assert($queen->toU64() === ($rook->toU64() | $bishop->toU64()));

// --- Bishop doc example: C2 with rank 6 blocking ---
$rank6 = \shakmaty\Bitboard::fromRank(\shakmaty\Rank::SIXTH);
$c2 = \shakmaty\Attacks::bishopAttacks(\shakmaty\Square::C2, $rank6);
assert($c2->has(\shakmaty\Square::G6));
assert(!$c2->has(\shakmaty\Square::H7));

// --- attacks() dispatch by piece ---
$empty = new \shakmaty\Bitboard(\shakmaty\Bitboard::EMPTY);

$knight_piece = new \shakmaty\Piece(\shakmaty\Color::WHITE, \shakmaty\Role::KNIGHT);
$from_piece = \shakmaty\Attacks::attacks(\shakmaty\Square::E4, $knight_piece, $empty);
$direct = \shakmaty\Attacks::knightAttacks(\shakmaty\Square::E4);
assert($from_piece->toU64() === $direct->toU64());

$king_piece = new \shakmaty\Piece(\shakmaty\Color::BLACK, \shakmaty\Role::KING);
$king_from_piece = \shakmaty\Attacks::attacks(\shakmaty\Square::E1, $king_piece, $empty);
assert($king_from_piece->toU64() === \shakmaty\Attacks::kingAttacks(\shakmaty\Square::E1)->toU64());

$rook_piece = new \shakmaty\Piece(\shakmaty\Color::WHITE, \shakmaty\Role::ROOK);
$rook_from_piece = \shakmaty\Attacks::attacks(\shakmaty\Square::D6, $rook_piece, $occupied);
assert($rook_from_piece->toU64() === $rook->toU64());

// --- ray ---
$ray = \shakmaty\Attacks::ray(\shakmaty\Square::E2, \shakmaty\Square::G4);
assert($ray->has(\shakmaty\Square::F3));
assert($ray->has(\shakmaty\Square::G4));
assert($ray->has(\shakmaty\Square::D1));

// --- between ---
$between = \shakmaty\Attacks::between(\shakmaty\Square::B1, \shakmaty\Square::B7);
assert($between->count() === 5);
assert($between->has(\shakmaty\Square::B2));
assert($between->has(\shakmaty\Square::B6));
assert(!$between->has(\shakmaty\Square::B1));
assert(!$between->has(\shakmaty\Square::B7));

// between on non-aligned squares is empty
assert(\shakmaty\Attacks::between(\shakmaty\Square::A1, \shakmaty\Square::B3)->isEmpty());

// --- aligned ---
assert(\shakmaty\Attacks::aligned(\shakmaty\Square::A1, \shakmaty\Square::B2, \shakmaty\Square::C3));
assert(!\shakmaty\Attacks::aligned(\shakmaty\Square::A1, \shakmaty\Square::B2, \shakmaty\Square::C4));

// --- out-of-range handling: empty / false, never throws ---
assert(\shakmaty\Attacks::knightAttacks(99)->isEmpty());
assert(\shakmaty\Attacks::pawnAttacks(\shakmaty\Color::WHITE, -1)->isEmpty());
assert(!\shakmaty\Attacks::aligned(0, 99, 63));

echo "Step 13: All attacks tests passed!\n";
