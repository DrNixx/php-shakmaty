<?php
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

use shakmaty\Zobrist64;

// Role values follow shakmaty's 1-based TryFrom mapping (Pawn = 1 .. King = 6).
$roles = ['p' => 1, 'n' => 2, 'b' => 3, 'r' => 4, 'q' => 5, 'k' => 6];

// Helper: XOR the Polyglot component for every occupied square of a board.
function polyglot_pieces(\shakmaty\Board $board): Zobrist64 {
    static $roles = ['p' => 1, 'n' => 2, 'b' => 3, 'r' => 4, 'q' => 5, 'k' => 6]; // shakmaty Role::try_from is 1-based (Pawn=1 .. King=6)
    $h = new Zobrist64(0);
    for ($sq = 0; $sq < 64; $sq++) {
        $ch = $board->pieceAt($sq); // null when the square is empty -> contributes nothing to the hash
        if ($ch === null) { continue; }
        assert(isset($roles[strtolower($ch)]), "unexpected piece char: {$ch}");
        $white = ctype_upper($ch) ? 1 : 0;
        $h = $h->bitwiseXor(Zobrist64::forPiece($sq, $white, $roles[strtolower($ch)]));
    }
    return $h;
}

// --- 1. Polyglot start-position hash built from primitives, checked against the reference value ---
$pos = new \shakmaty\Chess(); // default position: White to move, full castling rights "KQkq"
assert($pos->turn() === 1);   // so forWhiteTurn is added (Polyglot hashTurn only contributes on White's turn)

$h = polyglot_pieces($pos->board());
// White to move -> add the Polyglot turn component.
$h = $h->bitwiseXor(Zobrist64::forWhiteTurn());
// Full castling rights "KQkq" are present in the default position -> all four components contribute.
foreach ([[1, 0], [1, 1], [0, 0], [0, 1]] as [$c, $side]) { // (color: 1=White/0=Black) x (side: 0=KingSide/1=QueenSide)
    $h = $h->bitwiseXor(Zobrist64::forCastlingRight($c, $side));
}

assert($h->toHex() === '463b96181691fc9c'); // canonical Polyglot starting-position hash (matches shakmaty's own reference)
// Cross-check against the position's built-in zobristHash().
assert($h->equals($pos->zobristHash()) === true);

$zero = '0000000000000000'; // zero hash as a 16-digit hex string (used by several checks below)

// --- 2. forPiece with an out-of-range square or role yields the zero hash (no exception is thrown). ---
foreach ([64, 99] as $badSq) { // squares are valid only in 0..=63; anything else -> Err -> zero hash
    assert(Zobrist64::forPiece($badSq, 1, 5)->toHex() === $zero);
}
foreach ([0, 7] as $badRole) { // roles are valid only in 1..=6 (Pawn=1 .. King=6); anything else -> Err -> zero hash
    assert(Zobrist64::forPiece(28, 1, $badRole)->toHex() === $zero);
}

// Sanity: a VALID piece component is non-zero.
assert(Zobrist64::forPiece(28, 1, 5)->isZero() === false); // white queen on square 28 (d3) -> real Polyglot value

// --- 3. forCastlingRight with an invalid side yields the zero hash (side is only valid as 0 or 1). ---
assert(Zobrist64::forCastlingRight(1, 5)->isZero()); // White king-side/queen-side are the only valid sides; 5 -> zero

// Valid castling components must be non-zero and distinct per color.
$castles = [Zobrist64::forCastlingRight(0, 0), Zobrist64::forCastlingRight(1, 0)]; // Black king-side vs White king-side differ by color
assert($castles[0]->isZero() === false);
assert($castles[1]->isZero() === false);

// --- 4. forEnPassantFile: all eight files are non-zero and pairwise distinct; out-of-range -> zero hash. ---
$files = []; // collect the hex values of each file component to verify they are pairwise distinct
for ($f = 0; $f < 8; $f++) { // File A(=a) .. H (=h), i.e. file index 0..7 (File::try_from is valid only in this range)
    assert(Zobrist64::forEnPassantFile($f)->isZero() === false);
    $files[] = Zobrist64::forEnPassantFile($f)->toHex(); // store the hex value for the pairwise-distinctness check below
}

assert(count(array_unique($files)) === 8, "the eight en-passant file components must be pairwise distinct");

// Out-of-range files (>= 8) yield the zero hash.
foreach ([99] as $badF) { // File::try_from is valid only in 0..=7; anything else -> Err -> zero hash
    assert(Zobrist64::forEnPassantFile($badF)->toHex() === $zero);
}

// --- 5. Position with an en-passant square: rebuild the hash from primitives and compare it against zobristHash(). ---

// After White plays d4 (double push) from the starting position, Black to move, full castling rights intact.
// Constructed in-memory via playUci so that shakmaty's internal EP-tracking state is preserved exactly as it would be
// after actually playing this move — note: reconstructing an equivalent board purely via fromFen() does NOT preserve the
// raw (unconditional) EP target square, since standard FEN only records an explicit ep-square when one was written into its 4th field.
$pA = new \shakmaty\Chess(); // starting position, White to move
assert($pA->playUci('d2d4') === true); // double push creates a raw EP target on the d-file (rank 3)
assert($pA->turn() === 0);             // now Black's turn

// maybeEpSquare() returns non-null here (the raw/unconditional EP target recorded after any double-pawn-push), but
// legalEpSquare() correctly reports null since no black pawn sits adjacent on c5 or f5 in this clean setup — so
// shakmaty's zobristHash(Legal) omits the EP component entirely, and our manual reconstruction must match that.

$hEp = polyglot_pieces($pA->board());
// Black to move -> do NOT add forWhiteTurn (Polyglot hashTurn only contributes when it is White's turn).

// Castling rights present in this FEN ("KQkq") are all four.
foreach ([[1, 0], [1, 1], [0, 0], [0, 1]] as [$c, $side]) { // (color: 1=White/0=Black) x (side: 0=KingSide/1=QueenSide)
    $hEp = $hEp->bitwiseXor(Zobrist64::forCastlingRight($c, $side));
}

// En-passant component. shakmaty's zobristHash() uses LEGAL en-passant mode by default; mirror it exactly via legalEpSquare().
$epMaybe = $pA->maybeEpSquare(); // unconditional target square from the FEN (Some here)
assert($epMaybe !== null, "expected a non-null ep square in this position");

// shakmaty's zobristHash() uses LEGAL en-passant mode by default; mirror it exactly via legalEpSquare().
$legalEp = $pA->legalEpSquare(); // Some(sq) only if a real capture exists (mode Legal, the hash default); null otherwise
if ($epMaybe !== null && $legalEp === null) {
    echo "note: FEN ep square present but not legal; EP component omitted to match zobristHash(Legal)\n";
} elseif ($legalEp !== null) {
    assert($epMaybe === $legalEp, 'in this clean position maybe-ep and legal-ep should agree'); // both d3 -> file c = 2 (C)? No: d-file index is 3.
    $hEp = $hEp->bitwiseXor(Zobrist64::forEnPassantFile($legalEp % 8));
}

assert($hEp->toHex() === $pA->zobristHash()->toHex()); // manual reconstruction must equal the built-in hash (Legal ep mode)

echo "Step 27: All Polyglot Zobrist tests passed!\n";
