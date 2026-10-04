<?php
// Step 26: shakmaty\Fen — static factories, exceptions, conversions.
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

$start = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

function expect_exception(string $class, string $fen): void {
    try {
        \shakmaty\Fen::parse($fen);
        assert(false, "expected $class for '$fen'");
    } catch (\Throwable $e) {
        assert($e instanceof \shakmaty\fen\ParseFenError, "not a ParseFenError: " . get_class($e));
        assert(get_class($e) === $class, "expected $class, got " . get_class($e) . " for '$fen'");
    }
}

// --- parse / __toString ---
$fen = \shakmaty\Fen::parse($start);
assert($fen->__toString() === $start);
assert((string) $fen === $start);

// --- static empty ---
assert(\shakmaty\Fen::empty() instanceof \shakmaty\Fen);
assert((string) \shakmaty\Fen::empty() === "8/8/8/8/8/8/8/8 w - - 0 1");

// --- static isValid ---
assert(\shakmaty\Fen::isValid($start));
assert(!\shakmaty\Fen::isValid('not valid fen'));

// --- parse exception subclasses (reachable ones) ---
expect_exception(\shakmaty\fen\InvalidFen::class, "");
expect_exception(\shakmaty\fen\InvalidBoard::class, "8/8/8/8/8/8/8");
expect_exception(\shakmaty\fen\InvalidTurn::class, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR x - - 0 1");
expect_exception(\shakmaty\fen\InvalidCastling::class, "4k2r/8/8/8/8/8/8/RR2K2R w KBQk - 0 1");
expect_exception(\shakmaty\fen\InvalidEpSquare::class, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w  - 0 1");
expect_exception(\shakmaty\fen\InvalidPocket::class, "8/8/8/8/8/8/8/8[Z] w - - 0 1");
expect_exception(\shakmaty\fen\InvalidHalfmoveClock::class, "8/8/8/8/8/8/8/8 w - - abc 1");
expect_exception(\shakmaty\fen\InvalidFullmoves::class, "8/8/8/8/8/8/8/8 w - - 0 abc");

// InvalidRemainingChecks cannot be triggered from input (parser never emits it);
// verify only the class hierarchy.
assert(is_subclass_of(\shakmaty\fen\InvalidRemainingChecks::class, \shakmaty\fen\ParseFenError::class));

// --- private constructor ---
$threw = false;
try { new \shakmaty\Fen($start); } catch (\Throwable $e) { $threw = true; }
assert($threw, "public construction must fail");

// --- getSetup ---
assert($fen->getSetup() instanceof \shakmaty\Setup);

// --- getPosition ---
$pos = $fen->getPosition();
assert($pos instanceof \shakmaty\VariantPosition);
assert($pos->variant()->value() === \shakmaty\Variant::CHESS);
assert($pos->legalMovesCount() === 20);

$czFen = \shakmaty\Fen::parse("rnbqkbnr/ppp1pppp/8/8/8/8/PPPPPPPP/RNBQKBNR[P] w KQkq - 0 1");
$czPos = $czFen->getPosition(\shakmaty\Variant::CRAZYHOUSE);
assert($czPos->variant()->value() === \shakmaty\Variant::CRAZYHOUSE);

// --- fromPosition ---
assert((string) \shakmaty\Fen::fromPosition(new \shakmaty\Chess()) === $start);
assert((string) \shakmaty\Fen::fromPosition(new \shakmaty\VariantPosition(\shakmaty\Variant::CHESS)) === $start);

$threw = false;
try { \shakmaty\Fen::fromPosition(new \stdClass()); } catch (\Throwable $e) { $threw = true; }
assert($threw, "fromPosition must reject non-Position");

// --- fromSetup ---
assert((string) \shakmaty\Fen::fromSetup(new \shakmaty\Setup()) === $start);

// lossy setup: three castling rights on the first rank (A1|C1|H1 = bits 0,2,7 = 133)
$lossy = new \shakmaty\Setup();
$lossy->setCastlingRights(new \shakmaty\Bitboard(133));
$caught = null;
try {
    \shakmaty\Fen::fromSetup($lossy);
} catch (\shakmaty\fen\LossyFenError $e) {
    $caught = $e;
}
assert($caught !== null, "expected LossyFenError");
assert(($caught->getCode() & \shakmaty\fen\LossyFenError::CASTLING_RIGHTS) !== 0);

echo "Step 26: All Fen tests passed!\n";
