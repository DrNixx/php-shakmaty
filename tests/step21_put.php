<?php
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

use shakmaty\CastlingMode;
use shakmaty\Move;
use shakmaty\Role;
use shakmaty\Square;
use shakmaty\Variant;
use shakmaty\VariantPosition;

// --- Construct and inspect a drop move ---
$drop = Move::fromPut(Role::KNIGHT, Square::F3);
assert($drop instanceof Move);
assert($drop->isPut() === true);
assert($drop->role() === Role::KNIGHT);
assert($drop->fromSq() === null);
assert($drop->toSq() === Square::F3);
assert($drop->isCapture() === false);
assert($drop->isPromotion() === false);
assert($drop->isCastle() === false);
assert($drop->isEnPassant() === false);
assert($drop->toLong() === 'N@f3');

$pawnDrop = Move::fromPut(Role::PAWN, Square::E4);
assert($pawnDrop->toLong() === '@e4');
assert($pawnDrop->isPut() === true);
assert($pawnDrop->role() === Role::PAWN);

// invalid inputs throw
$threw = false;
try { Move::fromPut(0, Square::E4); } catch (\Throwable $e) { $threw = true; }
assert($threw === true);
$threw = false;
try { Move::fromPut(Role::QUEEN, 99); } catch (\Throwable $e) { $threw = true; }
assert($threw === true);

// NOTE: shakmaty enforces a GLOBAL per-role cap for Crazyhouse — total pawns on the board plus both pockets must be <= 16. A full starting board already has all 16 pawns, so adding even one pocket pawn (the contract's literal FEN `...RNBQKBNR[P]`) is an objectively illegal position that shakmaty rejects with "Invalid position". To have a droppable white pocket-pawn we must first capture one of Black's board pawns into White's pocket, so Black has only 7 on the board (8 + 7 + 1 = 16). This keeps every assertion below intact.
$crazyFen = "rnbqkbnr/ppp1pppp/8/8/8/8/PPPPPPPP/RNBQKBNR[P] w KQkq - 0 1";

// --- Drop is playable in Crazyhouse (one pawn in White's pocket) ---
$pos = VariantPosition::fromFen(Variant::CRAZYHOUSE, $crazyFen, CastlingMode::STANDARD);
$pos->playUci('P@e4');
assert($pos->turn() === 0);
assert(str_contains($pos->toFen(), '4P3')); // pawn now on e4

// --- Legal drops are present in legalMoves() and report isPut() ---
$pos2 = VariantPosition::fromFen(Variant::CRAZYHOUSE, $crazyFen, CastlingMode::STANDARD);
$moves = $pos2->legalMoves();
$foundPut = false;
$putCount = 0;
for ($i = 0; $i < $moves->count(); $i++) {
    if ($moves->get($i)->isPut()) {
        $foundPut = true;
        $putCount++;
    }
}
assert($foundPut === true);
assert($putCount >= 1);

// --- SAN drop also works ---
$pos3 = VariantPosition::fromFen(Variant::CRAZYHOUSE, $crazyFen, CastlingMode::STANDARD);
$pos3->playSan('@e4');
assert($pos3->turn() === 0);

echo "Step 21: All Move::Put tests passed!\n";
