<?php
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

use shakmaty\Variant;
use shakmaty\VariantPosition;

// === Constructor / variant selection ===
$def = new VariantPosition();
assert($def instanceof \shakmaty\Position);
assert($def->variant() instanceof Variant);
assert($def->variant()->value() === Variant::CHESS);
assert($def->legalMovesCount() === 20);
assert($def->turn() === 1);

$atomic = new VariantPosition(Variant::ATOMIC);
assert($atomic->variant()->value() === Variant::ATOMIC);
assert($atomic->legalMovesCount() === 20);

$antichess = new VariantPosition(Variant::ANTICHESS);
assert($antichess->variant()->value() === Variant::ANTICHESS);
assert($antichess->legalMovesCount() === 20);

$koth = new VariantPosition(Variant::KING_OF_THE_HILL);
assert($koth->variant()->value() === Variant::KING_OF_THE_HILL);
assert($koth->legalMovesCount() === 20);

$three = new VariantPosition(Variant::THREE_CHECK);
assert($three->variant()->value() === Variant::THREE_CHECK);
assert($three->legalMovesCount() === 20);
assert($three->isVariantEnd() === false);
assert($three->variantOutcome() === '*');

$crazy = new VariantPosition(Variant::CRAZYHOUSE);
assert($crazy->variant()->value() === Variant::CRAZYHOUSE);
assert($crazy->legalMovesCount() === 20);

$racing = new VariantPosition(Variant::RACING_KINGS);
assert($racing->variant()->value() === Variant::RACING_KINGS);
assert($racing->legalMovesCount() > 0);
assert($racing->board()->occupied()->count() === 16);

$horde = new VariantPosition(Variant::HORDE);
assert($horde->variant()->value() === Variant::HORDE);
assert($horde->legalMovesCount() > 0);
assert($horde->board()->occupied()->count() === 52);

// === Inherited Position contract ===
assert($def->us()->count() === 16);
assert($def->them()->count() === 16);
assert($def->checkers()->isEmpty());
assert($def->isCheck() === false);
assert($def->isCheckmate() === false);
assert($def->isGameOver() === false);
assert($def->outcome() === '*');
assert($def->maybeEpSquare() === null);
assert($def->toSetup(\shakmaty\EnPassantMode::LEGAL) instanceof \shakmaty\Setup);

// === play() returns a new VariantPosition, original unchanged ===
$moves = $def->legalMoves();
$first = $moves->get(0);
$next = $def->play($first);
assert($next instanceof VariantPosition);
assert($next->variant()->value() === Variant::CHESS);
assert($def->turn() === 1);
assert($next->turn() === 0);

// === playSan / playUnchecked mutate ===
$p = new VariantPosition(Variant::ATOMIC);
$p->playSan('e4');
assert($p->turn() === 0);

$p2 = new VariantPosition(Variant::CHESS);
$p2->playUnchecked($first);
assert($p2->turn() === 0);

// === swapTurn ===
$swapped = $def->swapTurn();
assert($swapped instanceof VariantPosition);
assert($swapped->turn() === 0);

// === toSetup / fromSetup roundtrip ===
$setup = $def->toSetup(\shakmaty\EnPassantMode::LEGAL);
$restored = VariantPosition::fromSetup(Variant::CHESS, $setup, \shakmaty\CastlingMode::STANDARD);
assert($restored instanceof VariantPosition);
assert($restored->variant()->value() === Variant::CHESS);
assert($restored->legalMovesCount() === 20);

// === fromFen / toFen ===
$fromFen = VariantPosition::fromFen(
    Variant::CHESS,
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    \shakmaty\CastlingMode::STANDARD
);
assert($fromFen->legalMovesCount() === 20);
assert(str_contains($fromFen->toFen(), 'rnbqkbnr'));

// === King of the Hill variant end (king on the center square d5) ===
$hill = VariantPosition::fromFen(
    Variant::KING_OF_THE_HILL,
    "8/8/8/3K4/8/8/8/7k w - - 0 1",
    \shakmaty\CastlingMode::STANDARD
);
assert($hill->isVariantEnd() === true);
assert($hill->variantOutcome() === '1-0');
assert($hill->isGameOver() === true);

// === Illegal play throws ===
$threw = false;
try {
    $bad = new VariantPosition(Variant::CHESS);
    $bad->play(new \shakmaty\Move());
} catch (\Throwable $e) {
    $threw = true;
}
assert($threw === true);

echo "Step 18: All VariantPosition tests passed!\n";
