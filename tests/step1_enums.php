<?php
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

// Color tests
$white = new \shakmaty\Color(\shakmaty\Color::WHITE);
assert($white->isWhite());
assert(!$white->isBlack());
assert($white->value() === 1);
assert($white->other()->isBlack());
assert($white->toChar() === 'w');

$black = new \shakmaty\Color(\shakmaty\Color::BLACK);
assert($black->isBlack());
assert($black->value() === 0);
assert($black->toChar() === 'b');

// Role tests
$king = new \shakmaty\Role(\shakmaty\Role::KING);
assert($king->value() === 6);
assert($king->toChar() === 'k');
assert($king->upperChar() === 'K');

$knight = \shakmaty\Role::fromChar('N');
assert($knight->value() === \shakmaty\Role::KNIGHT);
assert($knight->toChar() === 'n');

$pawn = new \shakmaty\Role(\shakmaty\Role::PAWN);
assert($pawn->value() === 1);
assert($pawn->toChar() === 'p');

// CastlingSide tests
$ks = new \shakmaty\CastlingSide(\shakmaty\CastlingSide::KING_SIDE);
assert($ks->isKingSide());
assert(!$ks->isQueenSide());
assert($ks->value() === 0);
assert($ks->other()->isQueenSide());

$qs = new \shakmaty\CastlingSide(\shakmaty\CastlingSide::QUEEN_SIDE);
assert($qs->isQueenSide());

// CastlingMode tests
$std = new \shakmaty\CastlingMode(\shakmaty\CastlingMode::STANDARD);
assert($std->isStandard());
assert(!$std->isChess960());

$c960 = new \shakmaty\CastlingMode(\shakmaty\CastlingMode::CHESS960);
assert($c960->isChess960());
assert(!$c960->isStandard());

// EnPassantMode tests
$legal = new \shakmaty\EnPassantMode(\shakmaty\EnPassantMode::LEGAL);
assert($legal->value() === 0);

$always = new \shakmaty\EnPassantMode(\shakmaty\EnPassantMode::ALWAYS);
assert($always->value() === 2);

echo "Step 1: All enum tests passed!\n";
