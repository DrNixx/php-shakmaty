<?php
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

// Standard starting position
$board = new \shakmaty\Board();
assert($board->pieceAt(\shakmaty\Square::E1) === 'K');
assert($board->pieceAt(\shakmaty\Square::D1) === 'Q');
assert($board->pieceAt(\shakmaty\Square::A1) === 'R');
assert($board->pieceAt(\shakmaty\Square::B1) === 'N');
assert($board->pieceAt(\shakmaty\Square::C1) === 'B');
assert($board->pieceAt(\shakmaty\Square::E8) === 'k');
assert($board->pieceAt(\shakmaty\Square::D8) === 'q');
assert($board->pieceAt(\shakmaty\Square::E2) === 'P');
assert($board->pieceAt(\shakmaty\Square::E7) === 'p');

// Empty squares
assert($board->pieceAt(\shakmaty\Square::E4) === null);
assert($board->pieceAt(\shakmaty\Square::D4) === null);

// Occupied
$occ = $board->occupied();
assert($occ->count() === 32);

// By color
$white = $board->byColor(1); // Color::White
assert($white->count() === 16);
$black = $board->byColor(0); // Color::Black
assert($black->count() === 16);

// By role
$pawns = $board->byRole(\shakmaty\Role::PAWN);
assert($pawns->count() === 16); // 8 white + 8 black
$knights = $board->byRole(\shakmaty\Role::KNIGHT);
assert($knights->count() === 4);
$kings = $board->byRole(\shakmaty\Role::KING);
assert($kings->count() === 2);

// roleAt / colorAt
assert($board->roleAt(\shakmaty\Square::E1) === \shakmaty\Role::KING);
assert($board->colorAt(\shakmaty\Square::E1) === 1); // White
assert($board->roleAt(\shakmaty\Square::E8) === \shakmaty\Role::KING);
assert($board->colorAt(\shakmaty\Square::E8) === 0); // Black

// Empty board
$empty = \shakmaty\Board::empty();
assert($empty->occupied()->isEmpty());
assert($empty->pieceAt(\shakmaty\Square::E4) === null);

// legacyPieceAt
assert($board->legacyPieceAt(\shakmaty\Square::E1) === 1);   // White King
assert($board->legacyPieceAt(\shakmaty\Square::D1) === 2);   // White Queen
assert($board->legacyPieceAt(\shakmaty\Square::A1) === 3);   // White Rook
assert($board->legacyPieceAt(\shakmaty\Square::B1) === 5);   // White Knight
assert($board->legacyPieceAt(\shakmaty\Square::C1) === 4);   // White Bishop
assert($board->legacyPieceAt(\shakmaty\Square::E2) === 6);   // White Pawn
assert($board->legacyPieceAt(\shakmaty\Square::E8) === 9);   // Black King
assert($board->legacyPieceAt(\shakmaty\Square::E7) === 14);  // Black Pawn
assert($board->legacyPieceAt(\shakmaty\Square::E4) === 7);   // NOPIECE
assert($empty->legacyPieceAt(\shakmaty\Square::E4) === 7);   // empty board
assert($board->legacyPieceAt(99) === 7);                     // invalid square

echo "Step 5: All board tests passed!\n";
