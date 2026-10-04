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

echo "Step 3: All piece tests passed!\n";
