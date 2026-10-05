<?php
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

// Create piece from color and role values
$wk = new \shakmaty\Piece(\shakmaty\Color::WHITE, \shakmaty\Role::KING);
assert($wk->toChar() === 'K');
assert($wk->toUnicode() === "♔");
assert($wk->color->isWhite());
assert($wk->role->value() === \shakmaty\Role::KING);

$bp = new \shakmaty\Piece(\shakmaty\Color::BLACK, \shakmaty\Role::PAWN);
assert($bp->toChar() === 'p');
assert($bp->toUnicode() === "♟");
assert($bp->color->isBlack());
assert($bp->role->value() === \shakmaty\Role::PAWN);

// fromChar
$wn = \shakmaty\Piece::fromChar('N');
assert($wn->color->isWhite());
assert($wn->role->value() === \shakmaty\Role::KNIGHT);
assert($wn->toChar() === 'N');

$bb = \shakmaty\Piece::fromChar('b');
assert($bb->color->isBlack());
assert($bb->role->value() === \shakmaty\Role::BISHOP);
assert($bb->toChar() === 'b');

$wq = \shakmaty\Piece::fromChar('Q');
assert($wq->color->isWhite());
assert($wq->role->value() === \shakmaty\Role::QUEEN);
assert($wq->toUnicode() === "♕");

// Read-only properties: direct assignment must throw
$threw = false;
try { $wk->color = new \shakmaty\Color(\shakmaty\Color::BLACK); } catch (\Throwable $e) { $threw = true; }
assert($threw, "writing the read-only 'color' property must throw");

$threw = false;
try { $wk->role = new \shakmaty\Role(\shakmaty\Role::PAWN); } catch (\Throwable $e) { $threw = true; }
assert($threw, "writing the read-only 'role' property must throw");

// legacyCode / fromLegacyCode
assert(\shakmaty\Piece::legacyCode($wk->color, $wk->role) === 1);    // White King
assert(\shakmaty\Piece::legacyCode($bp->color, $bp->role) === 14);   // Black Pawn
$wpawn = new \shakmaty\Piece(\shakmaty\Color::WHITE, \shakmaty\Role::PAWN);
assert(\shakmaty\Piece::legacyCode($wpawn->color, $wpawn->role) === 6); // White Pawn
$bking = new \shakmaty\Piece(\shakmaty\Color::BLACK, \shakmaty\Role::KING);
assert(\shakmaty\Piece::legacyCode($bking->color, $bking->role) === 9); // Black King

$roundWk = \shakmaty\Piece::fromLegacyCode(1);
assert($roundWk->color->isWhite() && $roundWk->role->value() === \shakmaty\Role::KING);
$roundBp = \shakmaty\Piece::fromLegacyCode(14);
assert($roundBp->color->isBlack() && $roundBp->role->value() === \shakmaty\Role::PAWN);
assert(\shakmaty\Piece::fromLegacyCode(7) === null);   // NOPIECE
assert(\shakmaty\Piece::fromLegacyCode(0) === null);   // invalid
assert(\shakmaty\Piece::fromLegacyCode(8) === null);   // invalid
assert(\shakmaty\Piece::fromLegacyCode(15) === null);  // invalid

echo "Step 3: All piece tests passed!\n";
