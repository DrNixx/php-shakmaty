<?php
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

use shakmaty\Chess;
use shakmaty\Move;
use shakmaty\MoveList;
use shakmaty\San;
use shakmaty\Variant;
use shakmaty\VariantPosition;

// Helper: find a legal move by its long notation.
function find_by_long(MoveList $moves, string $long): ?Move {
    for ($i = 0; $i < $moves->count(); $i++) {
        if ($moves->get($i)->toLong() === $long) { return $moves->get($i); }
    }
    return null;
}

$start = new Chess();
$startMoves = $start->legalMoves();
$nf3 = find_by_long($startMoves, 'Ng1f3');
$e4 = find_by_long($startMoves, 'e2e4');
assert($nf3 !== null && $e4 !== null);

// --- fromMove ---
assert(San::fromMove($start, $nf3)->__toString() === 'Nf3');
assert(San::fromMove($start, $e4)->__toString() === 'e4');

// castling
$castle = new Chess();
foreach (['e2e4', 'e7e5', 'g1f3', 'b8c6', 'f1c4', 'f8c5'] as $uci) {
    assert($castle->playUci($uci) === true);
}
$oo = find_by_long($castle->legalMoves(), 'O-O');
assert($oo !== null);
assert(San::fromMove($castle, $oo)->__toString() === 'O-O');

// promotion
$promo = Chess::fromFen('8/P6k/8/8/8/8/6K1/8 w - - 0 1');
$promoMove = find_by_long($promo->legalMoves(), 'a7a8=Q');
assert($promoMove !== null);
assert(San::fromMove($promo, $promoMove)->__toString() === 'a8=Q');

// non-position argument throws
$threw = false;
try { San::fromMove(new \stdClass(), $nf3); } catch (\Throwable $e) { $threw = true; }
assert($threw === true);

// VariantPosition is accepted
$vp = new VariantPosition(Variant::CHESS);
assert(San::fromMove($vp, $vp->legalMoves()->get(0)) instanceof San);

// --- toMove ---
$resolved = (new San('Nf3'))->toMove($start);
assert($resolved instanceof Move);
assert($resolved->toLong() === 'Ng1f3');
assert(San::fromMove($start, $resolved)->__toString() === 'Nf3');

// VariantPosition accepted
assert((new San('Nf3'))->toMove($vp)->toLong() === 'Ng1f3');

// errors carry distinct messages
$msg = null;
try { (new San('zzz'))->toMove($start); } catch (\Throwable $e) { $msg = $e->getMessage(); }
assert($msg === 'Invalid SAN');

$msg = null;
try { (new San('Nf6'))->toMove($start); } catch (\Throwable $e) { $msg = $e->getMessage(); }
assert($msg === 'Illegal SAN');

$amb = Chess::fromFen('4k3/8/8/8/8/2N5/8/4K1N1 w - - 0 1');
$msg = null;
try { (new San('Ne2'))->toMove($amb); } catch (\Throwable $e) { $msg = $e->getMessage(); }
assert($msg === 'Ambiguous SAN');

// non-position throws
$threw = false;
try { (new San('Nf3'))->toMove(new \stdClass()); } catch (\Throwable $e) { $threw = true; }
assert($threw === true);

// --- findMove ---
assert((new San('Nf3'))->findMove($startMoves)->toLong() === 'Ng1f3');
assert((new San('Nf6'))->findMove($startMoves) === null);        // no match
assert((new San('zzz'))->findMove($startMoves) === null);        // invalid syntax
assert((new San('Ne2'))->findMove($amb->legalMoves()) === null); // ambiguous

// --- matches ---
assert((new San('Nf3'))->matches($nf3) === true);
assert((new San('Ng1f3'))->matches($nf3) === true);   // long form matches too
assert((new San('Nf3'))->matches($e4) === false);     // wrong move
assert((new San('Nxf3'))->matches($nf3) === false);   // capture mismatch
assert((new San('zzz'))->matches($nf3) === false);    // invalid syntax

echo "Step 29: All San move-conversion tests passed!\n";
