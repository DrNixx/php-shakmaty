<?php
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

// === Starting position ===
$pos = new \shakmaty\Chess();
assert($pos->turn() === 1); // White
assert($pos->fullmoves() === 1);
assert($pos->halfmoves() === 0);
assert(!$pos->isCheck());
assert(!$pos->isCheckmate());
assert(!$pos->isStalemate());
assert(!$pos->isInsufficientMaterial());
assert($pos->outcome() === '*');
assert($pos->legalMovesCount() === 20);
assert($pos->epSquare() === null);

// Castling rights
$cr = $pos->castlingRights();
assert($cr->count() === 4);

// Board
$board = $pos->board();
assert($board->pieceAt(\shakmaty\Square::E1) === 'K');

// === Play SAN ===
$pos->playSan("e4");
assert($pos->turn() === 0); // Black
assert($pos->legalMovesCount() > 0);
// Contract deviation (see report): after a pawn move the halfmove counter resets to 0 rather than incrementing to 1 —
// this is confirmed by shakmaty itself: its FEN after e4 contains the field "0" (... b KQkq - 0 1). Standard chess/FEN semantics.
assert($pos->halfmoves() === 0);

$pos->playSan("e5");
$pos->playSan("Qh5");
$pos->playSan("Nc6");
$pos->playSan("Bc4");
$pos->playSan("Nf6");
$pos->playSan("Qxf7"); // Scholar's mate
assert($pos->isCheckmate());
assert($pos->outcome() === '1-0');

// === Play UCI ===
$pos2 = new \shakmaty\Chess();
$pos2->playUci("e2e4");
assert($pos2->turn() === 0);

// === FEN roundtrip ===
$pos3 = new \shakmaty\Chess();
$fen = $pos3->toFen();
assert(strpos($fen, 'rnbqkbnr') !== false);
$pos3b = \shakmaty\Chess::fromFen($fen);
assert($pos3b->legalMovesCount() === 20);

// === FEN with checkmate ===
$pos4 = \shakmaty\Chess::fromFen("r1bqkb1r/pppp1Qpp/2n2n2/4p3/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 4");
assert($pos4->isCheckmate());
assert($pos4->outcome() === '1-0');

// === legalMoves returns MoveList ===
$moves = $pos3->legalMoves();
assert($moves->count() === 20);

echo "Step 8: All chess tests passed!\n";
