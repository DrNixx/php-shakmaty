<?php
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

// Default setup (starting position)
$setup = new \shakmaty\Setup();
assert($setup->turn === 1); // White
assert($setup->halfmoves === 0);
assert($setup->fullmoves === 1);
assert($setup->board !== null);
assert($setup->board->pieceAt(\shakmaty\Square::E1) === 'K');

// Castling rights in starting position
$castling = $setup->castlingRights;
assert($castling->has(\shakmaty\Square::A1)); // white queen-side rook
assert($castling->has(\shakmaty\Square::H1)); // white king-side rook
assert($castling->has(\shakmaty\Square::A8)); // black queen-side rook
assert($castling->has(\shakmaty\Square::H8)); // black king-side rook
assert($castling->count() === 4);

// No en passant in starting position
assert($setup->epSquare === null);

// Empty setup
$empty = \shakmaty\Setup::empty();
assert($empty->board->occupied()->isEmpty());
assert($empty->turn === 1);

// Mutation through writable scalar properties
$setup->turn = 0; // Black to move
assert($setup->turn === 0);

$setup->halfmoves = 10;
assert($setup->halfmoves === 10);

$setup->fullmoves = 5;
assert($setup->fullmoves === 5);

$setup->epSquare = \shakmaty\Square::E3;
assert($setup->epSquare === \shakmaty\Square::E3);

$setup->epSquare = null;
assert($setup->epSquare === null);

// Object-valued properties are read-only: direct assignment must throw
$threwReadonly = false;
try { $setup->board = new \shakmaty\Board(); } catch (\Throwable $e) { $threwReadonly = true; }
assert($threwReadonly, "writing the read-only 'board' property must throw");

$threwReadonly = false;
try { $setup->castlingRights = new \shakmaty\Bitboard(0); } catch (\Throwable $e) { $threwReadonly = true; }
assert($threwReadonly, "writing the read-only 'castlingRights' property must throw");

// Object-valued state is mutated through setters only
$setup->setBoard(new \shakmaty\Board());
$setup->setCastlingRights(new \shakmaty\Bitboard(0));
assert($setup->castlingRights->count() === 0);

// --- static factory: fromFen ---
$startFen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
$fromFen = \shakmaty\Setup::fromFen($startFen);
assert($fromFen->turn === 1);
assert($fromFen->fullmoves === 1);
assert($fromFen->castlingRights->count() === 4);
assert($fromFen->board->pieceAt(\shakmaty\Square::E1) === 'K');
assert((string) \shakmaty\Fen::fromSetup($fromFen) === $startFen); // lossless round-trip

// halfmove/fullmove counters and en passant square are parsed
$midFen = \shakmaty\Setup::fromFen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2");
assert($midFen->halfmoves === 0);
assert($midFen->fullmoves === 2);
assert($midFen->epSquare === \shakmaty\Square::E6);

// Crazyhouse pocket is parsed into the setup
$czSetup = \shakmaty\Setup::fromFen("rnbqkbnr/ppp1pppp/8/8/8/8/PPPPPPPP/RNBQKBNR[P] w KQkq - 0 1");
assert($czSetup->pockets !== null);
assert($czSetup->pockets->count(\shakmaty\Color::WHITE, \shakmaty\Role::PAWN) === 1);

// invalid FEN throws a ParseFenError subclass
$threw = false;
try { \shakmaty\Setup::fromFen('invalid'); } catch (\shakmaty\fen\ParseFenError $e) { $threw = true; }
assert($threw, "invalid FEN must throw ParseFenError");

echo "Step 6: All setup tests passed!\n";
