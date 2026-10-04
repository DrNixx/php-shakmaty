<?php
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

// === Abstract class semantics ===
$rc = new ReflectionClass('shakmaty\\Position');
assert($rc->isAbstract() === true);

$threw = false;
try {
    new \shakmaty\Position();
} catch (\Throwable $e) {
    $threw = true;
}
assert($threw === true);

$pos = new \shakmaty\Chess();
assert($pos instanceof \shakmaty\Position);

// === Required trait methods ===
assert($pos->turn() === 1);
assert($pos->board() instanceof \shakmaty\Board);
assert($pos->promoted() instanceof \shakmaty\Bitboard);
assert($pos->promoted()->isEmpty());
assert($pos->castlingRights()->count() === 4);
assert($pos->maybeEpSquare() === null);
assert($pos->halfmoves() === 0);
assert($pos->fullmoves() === 1);
assert($pos->legalMoves()->count() === 20);
assert($pos->isVariantEnd() === false);
assert($pos->hasInsufficientMaterial(\shakmaty\Color::WHITE) === false);
assert($pos->variantOutcome() === '*');

// === Provided/default methods ===
assert($pos->us()->count() === 16);
assert($pos->them()->count() === 16);
assert($pos->our(\shakmaty\Role::PAWN)->count() === 8);
assert($pos->their(\shakmaty\Role::PAWN)->count() === 8);
assert($pos->checkers()->isEmpty());
assert($pos->isCheck() === false);
assert($pos->isCheckmate() === false);
assert($pos->isStalemate() === false);
assert($pos->isInsufficientMaterial() === false);
assert($pos->isGameOver() === false);
assert($pos->outcome() === '*');
assert($pos->epSquare() === null);
assert($pos->epSquare(\shakmaty\EnPassantMode::LEGAL) === null);
assert($pos->pseudoLegalEpSquare() === null);
assert($pos->legalEpSquare() === null);
assert($pos->captureMoves()->count() === 0);
assert($pos->promotionMoves()->count() === 0);
assert($pos->enPassantMoves()->count() === 0);
assert($pos->castlingMoves(\shakmaty\CastlingSide::KING_SIDE)->count() === 0);
assert($pos->castlingMoves(\shakmaty\CastlingSide::QUEEN_SIDE)->count() === 0);

// === Move-level defaults ===
$moves = $pos->legalMoves();
$e4 = null;
for ($i = 0; $i < $moves->count(); $i++) {
    $m = $moves->get($i);
    if ($m->toLong() === 'e2e4') {
        $e4 = $m;
        break;
    }
}
assert($e4 !== null);
assert($pos->isLegal($e4) === true);
assert($pos->isIrreversible($e4) === true);
assert($pos->sanCandidates(\shakmaty\Role::PAWN, \shakmaty\Square::E4)->count() === 1);
assert($pos->kingAttackers(\shakmaty\Square::E1, \shakmaty\Color::BLACK, $pos->board()->occupied())->isEmpty());

// play() returns a NEW position; the original stays unchanged
$next = $pos->play($e4);
assert($next instanceof \shakmaty\Chess);
assert($next instanceof \shakmaty\Position);
assert($next->turn() === 0);
assert($pos->turn() === 1);

// toSetup()
$setup = $pos->toSetup(\shakmaty\EnPassantMode::LEGAL);
assert($setup instanceof \shakmaty\Setup);
assert($setup->turn === 1);

// playUnchecked() mutates
$p2 = new \shakmaty\Chess();
$p2->playUnchecked($e4);
assert($p2->turn() === 0);

// === Checkmate ===
$mate = \shakmaty\Chess::fromFen("r1bqkb1r/pppp1Qpp/2n2n2/4p3/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 4");
assert($mate->isCheckmate() === true);
assert($mate->isGameOver() === true);
assert($mate->outcome() === '1-0');
assert($mate->checkers()->count() === 1);

// === Stalemate ===
$stale = \shakmaty\Chess::fromFen("7k/5Q2/6K1/8/8/8/8/8 b - - 0 1");
assert($stale->isStalemate() === true);
assert($stale->isGameOver() === true);
assert($stale->outcome() === '1/2-1/2');
assert($stale->checkers()->isEmpty());

// === Illegal play() throws ===
$threwPlay = false;
try {
    $bad = new \shakmaty\Chess();
    $illegal = new \shakmaty\Move();
    $bad->play($illegal);
} catch (\Throwable $e) {
    $threwPlay = true;
}
assert($threwPlay === true);

echo "Step 16: All Position tests passed!\n";
