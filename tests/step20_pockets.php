<?php
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

use shakmaty\ByRole;
use shakmaty\Color;
use shakmaty\Pockets;
use shakmaty\Role;

// --- ByRole (u8 counts) ---
$counts = new ByRole();
assert($counts->total() === 0);
assert($counts->isEmpty() === true);
assert($counts->get(Role::KNIGHT) === 0);

$counts->set(Role::KNIGHT, 2);
assert($counts->get(Role::KNIGHT) === 2);
assert($counts->knight === 2);
assert($counts->pawn === 0);
assert($counts->isEmpty() === false);

$counts->set(Role::PAWN, 3);
assert($counts->total() === 5);
assert($counts->toArray() === [3, 2, 0, 0, 0, 0]);
assert($counts->get(99) === 0);
$counts->set(99, 5);
assert($counts->total() === 5);
$counts->set(Role::QUEEN, 999);
assert($counts->queen === 255);
$counts->set(Role::QUEEN, -4);
assert($counts->queen === 0);

// copy() is independent
$clone = $counts->copy();
$clone->set(Role::ROOK, 7);
assert($clone->rook === 7);
assert($counts->rook === 0);

// --- Pockets (ByColor<ByRole<u8>>) ---
$pockets = new Pockets();
assert($pockets->isEmpty() === true);
assert($pockets->total() === 0);

$pockets->setCount(Color::WHITE, Role::QUEEN, 1);
$pockets->setCount(Color::BLACK, Role::PAWN, 2);
assert($pockets->count(Color::WHITE, Role::QUEEN) === 1);
assert($pockets->count(Color::BLACK, Role::PAWN) === 2);
assert($pockets->count(Color::WHITE, Role::PAWN) === 0);
assert($pockets->total() === 3);
assert($pockets->isEmpty() === false);

$white = $pockets->white;
assert($white instanceof ByRole);
assert($white->queen === 1);
assert($pockets->black->pawn === 2);
assert($pockets->get(Color::WHITE) instanceof ByRole);

// get() returns a copy; changes are only persisted via set()
$work = $pockets->get(Color::WHITE);
$work->set(Role::ROOK, 1);
assert($pockets->count(Color::WHITE, Role::ROOK) === 0);
$pockets->set(Color::WHITE, $work);
assert($pockets->count(Color::WHITE, Role::ROOK) === 1);
assert($pockets->count(Color::WHITE, Role::QUEEN) === 1);

assert($pockets->count(Color::WHITE, 99) === 0);
$pockets->setCount(Color::WHITE, 99, 3);
assert($pockets->total() === 4);

$pocketsClone = $pockets->copy();
$pocketsClone->setCount(Color::WHITE, Role::PAWN, 5);
assert($pocketsClone->count(Color::WHITE, Role::PAWN) === 5);
assert($pockets->count(Color::WHITE, Role::PAWN) === 0);

echo "Step 20: All Pockets tests passed!\n";
