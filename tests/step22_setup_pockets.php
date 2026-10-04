<?php
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

use shakmaty\Bitboard;
use shakmaty\CastlingMode;
use shakmaty\Color;
use shakmaty\EnPassantMode;
use shakmaty\Pockets;
use shakmaty\RemainingChecks;
use shakmaty\Role;
use shakmaty\Setup;
use shakmaty\Square;
use shakmaty\Variant;
use shakmaty\VariantPosition;

$setup = new Setup();

// --- promoted (read-only property; write via setPromoted) ---
assert($setup->promoted instanceof Bitboard);
assert($setup->promoted->isEmpty());
$setup->setPromoted(Bitboard::fromSquare(Square::D8));
assert($setup->promoted->has(Square::D8));

// --- pockets (read-only property; write via setPockets / clearPockets) ---
assert($setup->pockets === null);
$pockets = new Pockets();
$pockets->setCount(Color::WHITE, Role::PAWN, 1);
$setup->setPockets($pockets);
$roundtrip = $setup->pockets;
assert($roundtrip instanceof Pockets);
assert($roundtrip->count(Color::WHITE, Role::PAWN) === 1);
assert($roundtrip->count(Color::BLACK, Role::PAWN) === 0);
$setup->clearPockets();
assert($setup->pockets === null);

// --- remaining checks (still methods) ---
assert($setup->remainingChecks(Color::WHITE) === null);
$setup->setRemainingChecks(Color::WHITE, new RemainingChecks(2));
$setup->setRemainingChecks(Color::BLACK, new RemainingChecks(1));
assert($setup->remainingChecks(Color::WHITE)->value() === 2);
assert($setup->remainingChecks(Color::BLACK)->value() === 1);
$setup->clearRemainingChecks();
assert($setup->remainingChecks(Color::WHITE) === null);

// --- Crazyhouse: pockets survive toSetup() -> fromSetup() ---
// Black is missing the d7 pawn; that pawn is in White's pocket.
$czFen = "rnbqkbnr/ppp1pppp/8/8/8/8/PPPPPPPP/RNBQKBNR[P] w KQkq - 0 1";
$cz = VariantPosition::fromFen(Variant::CRAZYHOUSE, $czFen, CastlingMode::STANDARD);
$czSetup = $cz->toSetup(EnPassantMode::LEGAL);
assert($czSetup->pockets instanceof Pockets);
assert($czSetup->pockets->count(Color::WHITE, Role::PAWN) === 1);

$czRebuilt = VariantPosition::fromSetup(Variant::CRAZYHOUSE, $czSetup, CastlingMode::STANDARD);
assert($czRebuilt->legalMovesCount() > 0);
$czRebuilt->playUci('P@e4'); // legal drop consumes the pocket pawn
assert($czRebuilt->turn() === 0);

// --- ThreeCheck: remaining checks survive toSetup() -> fromSetup() ---
$tcFen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 3+3 0 1";
$tc = VariantPosition::fromFen(Variant::THREE_CHECK, $tcFen, CastlingMode::STANDARD);
$tcSetup = $tc->toSetup(EnPassantMode::LEGAL);
assert($tcSetup->remainingChecks(Color::WHITE)->value() === 3);
assert($tcSetup->remainingChecks(Color::BLACK)->value() === 3);

$tcSetup->setRemainingChecks(Color::WHITE, new RemainingChecks(1));
$tcRebuilt = VariantPosition::fromSetup(Variant::THREE_CHECK, $tcSetup, CastlingMode::STANDARD);
$tcRebuiltSetup = $tcRebuilt->toSetup(EnPassantMode::LEGAL);
assert($tcRebuiltSetup->remainingChecks(Color::WHITE)->value() === 1);
assert($tcRebuiltSetup->remainingChecks(Color::BLACK)->value() === 3);

echo "Step 22: All Setup pockets/checks tests passed!\n";
