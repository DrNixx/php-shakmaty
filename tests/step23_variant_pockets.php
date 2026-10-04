<?php
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

use shakmaty\CastlingMode;
use shakmaty\Chess;
use shakmaty\Color;
use shakmaty\Pockets;
use shakmaty\RemainingChecks;
use shakmaty\Role;
use shakmaty\Variant;
use shakmaty\VariantPosition;

// --- Crazyhouse pockets via position ---
$crazyFen = "rnbqkbnr/ppp1pppp/8/8/8/8/PPPPPPPP/RNBQKBNR[P] w KQkq - 0 1";
$cz = VariantPosition::fromFen(Variant::CRAZYHOUSE, $crazyFen, CastlingMode::STANDARD);
assert($cz instanceof \shakmaty\Position);
$pockets = $cz->pockets();
assert($pockets instanceof Pockets);
assert($pockets->count(Color::WHITE, Role::PAWN) === 1);
assert($pockets->count(Color::BLACK, Role::PAWN) === 0);
assert($pockets->total() === 1);

// a legal drop consumes the pocket pawn
$cz->playUci('P@e4');
assert($cz->turn() === 0);
$after = $cz->pockets();
assert($after instanceof Pockets);
assert($after->count(Color::WHITE, Role::PAWN) === 0);
assert($after->isEmpty() === true);

// --- Positions without pockets return null ---
assert((new VariantPosition(Variant::CHESS))->pockets() === null);
assert((new VariantPosition(Variant::ATOMIC))->pockets() === null);
assert((new Chess())->pockets() === null);

// --- ThreeCheck remaining checks ---
$tc = VariantPosition::fromFen(
    Variant::THREE_CHECK,
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 2+3 0 1",
    CastlingMode::STANDARD
);
assert($tc->remainingChecks(Color::WHITE) instanceof RemainingChecks);
assert($tc->remainingChecks(Color::WHITE)->value() === 2);
assert($tc->remainingChecks(Color::BLACK)->value() === 3);

// giving check decreases the mover's remaining checks
$tc2 = VariantPosition::fromFen(
    Variant::THREE_CHECK,
    "4k3/8/8/8/8/8/8/R3K3 w Q - 3+3 0 1",
    CastlingMode::STANDARD
);
assert($tc2->remainingChecks(Color::WHITE)->value() === 3);
$tc2->playUci('a1a8'); // Rook to a8: check
assert($tc2->isCheck() === true);
assert($tc2->remainingChecks(Color::WHITE)->value() === 2);
assert($tc2->remainingChecks(Color::BLACK)->value() === 3);

// --- Positions without remaining checks return null ---
assert((new VariantPosition(Variant::ATOMIC))->remainingChecks(Color::WHITE) === null);
assert((new Chess())->remainingChecks(Color::WHITE) === null);

echo "Step 23: All VariantPosition pockets/checks tests passed!\n";
