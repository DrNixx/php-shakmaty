<?php
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

use shakmaty\CastlingMode;
use shakmaty\Chess;
use shakmaty\EnPassantMode;
use shakmaty\Variant;
use shakmaty\VariantPosition;
use shakmaty\Zobrist64;

// --- Chess: documented hash of the starting position ---
$pos = new Chess();
$start = $pos->zobristHash();
assert($start instanceof Zobrist64);
assert($start->toHex() === '463b96181691fc9c');
assert($pos->zobristHash(EnPassantMode::LEGAL)->equals($start) === true);
assert((new Chess())->zobristHash()->equals($start) === true);

// the hash changes after a move
$pos->playUci('g1f3');
assert($pos->zobristHash()->equals($start) === false);

// --- incremental update matches full recomputation ---
$p = new Chess();
$legal = $p->legalMoves();
$nf3 = null;
$e4 = null;
for ($i = 0; $i < $legal->count(); $i++) {
    $m = $legal->get($i);
    if ($m->toLong() === 'Ng1f3') { $nf3 = $m; }
    if ($m->toLong() === 'e2e4') { $e4 = $m; }
}
assert($nf3 !== null && $e4 !== null);

$updated = $p->updateZobristHash($p->zobristHash(), $nf3);
assert($updated instanceof Zobrist64);
assert($updated->equals($p->play($nf3)->zobristHash()) === true);

// double pawn push is not incrementally supported -> null
assert($p->updateZobristHash($p->zobristHash(), $e4) === null);

// alternative en-passant modes are accepted
assert($p->zobristHash(EnPassantMode::ALWAYS) instanceof Zobrist64);
assert($p->zobristHash(EnPassantMode::PSEUDO_LEGAL) instanceof Zobrist64);

// --- VariantPosition delegates to the wrapped variant ---
$vp = new VariantPosition(Variant::CHESS);
assert($vp->zobristHash() instanceof Zobrist64);
assert($vp->zobristHash()->equals($start) === true);

$vpLegal = $vp->legalMoves();
$vpNf3 = null;
for ($i = 0; $i < $vpLegal->count(); $i++) {
    if ($vpLegal->get($i)->toLong() === 'Ng1f3') { $vpNf3 = $vpLegal->get($i); break; }
}
assert($vpNf3 !== null);
$vpUpdated = $vp->updateZobristHash($vp->zobristHash(), $vpNf3);
assert($vpUpdated instanceof Zobrist64);
assert($vpUpdated->equals($vp->play($vpNf3)->zobristHash()) === true);

// --- variant state participates in the hash ---
// Empty Crazyhouse pockets hash to zero, so the default matches standard chess
$czEmpty = new VariantPosition(Variant::CRAZYHOUSE);
assert($czEmpty->zobristHash()->equals($start) === true);

// A non-empty pocket changes the hash (black is missing the d7 pawn, which is in White's pocket)
$czPocket = VariantPosition::fromFen(
    Variant::CRAZYHOUSE,
    "rnbqkbnr/ppp1pppp/8/8/8/8/PPPPPPPP/RNBQKBNR[P] w KQkq - 0 1",
    CastlingMode::STANDARD
);
assert($czPocket->zobristHash()->equals($start) === false);

// The default 3+3 remaining checks hash to zero...
$tcDefault = new VariantPosition(Variant::THREE_CHECK);
assert($tcDefault->zobristHash()->equals($start) === true);

// ...but any other remaining-checks value changes the hash
$tcChanged = VariantPosition::fromFen(
    Variant::THREE_CHECK,
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 2+3 0 1",
    CastlingMode::STANDARD
);
assert($tcChanged->zobristHash()->equals($start) === false);

echo "Step 25: All Position Zobrist tests passed!\n";
