<?php
/**
 * shakmaty integration test — exercises all classes and the main scenarios.
 */
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

$pass = 0;
$fail = 0;

function test(string $name, bool $condition): void {
    global $pass, $fail;
    if ($condition) {
        $pass++;
    } else {
        $fail++;
        echo "  FAIL: $name\n";
    }
}

// ---- Step 1: Enums ----
$white = new \shakmaty\Color(\shakmaty\Color::WHITE);
test("Color::WHITE isWhite", $white->isWhite());
test("Color::WHITE value", $white->value() === 1);
test("Color::other()", $white->other()->isBlack());

$king = new \shakmaty\Role(\shakmaty\Role::KING);
test("Role::KING value", $king->value() === 6);
$n = \shakmaty\Role::fromChar('N');
test("Role::fromChar('N')", $n->value() === \shakmaty\Role::KNIGHT);

$ks = new \shakmaty\CastlingSide(\shakmaty\CastlingSide::KING_SIDE);
test("CastlingSide::KING_SIDE", $ks->isKingSide());

$std = new \shakmaty\CastlingMode(\shakmaty\CastlingMode::STANDARD);
test("CastlingMode::STANDARD", $std->isStandard());

$legal = new \shakmaty\EnPassantMode(\shakmaty\EnPassantMode::LEGAL);
test("EnPassantMode::LEGAL value", $legal->value() === 0);

// ---- Step 2: Square, File, Rank ----
$sq = \shakmaty\Square::fromAscii("e4");
test("Square::fromAscii('e4')", $sq->__toString() === 'e4');
test("Square file", $sq->file()->value() === \shakmaty\File::E);
test("Square rank", $sq->rank()->value() === \shakmaty\Rank::FOURTH);
test("Square isLight", $sq->isLight());

$sq_a1 = new \shakmaty\Square(\shakmaty\Square::A1);
test("Square A1 isDark", $sq_a1->isDark());
test("Square A1 flipHorizontal", $sq_a1->flipHorizontal()->value() === \shakmaty\Square::H1);

$file_h = \shakmaty\File::fromChar('h');
test("File::fromChar('h')", $file_h->value() === \shakmaty\File::H);

$rank_8 = \shakmaty\Rank::fromChar('8');
test("Rank::fromChar('8')", $rank_8->value() === \shakmaty\Rank::EIGHTH);

$fromCoords = \shakmaty\Square::fromCoords(
    new \shakmaty\File(\shakmaty\File::D),
    new \shakmaty\Rank(\shakmaty\Rank::FIFTH)
);
test("Square::fromCoords(D, 5)", $fromCoords->__toString() === 'd5');
test("Square::NS null square", \shakmaty\Square::NS === 64);

// ---- Step 3: Piece ----
$wk = new \shakmaty\Piece(\shakmaty\Color::WHITE, \shakmaty\Role::KING);
test("Piece White King toChar", $wk->toChar() === 'K');
test("Piece White King toUnicode", $wk->toUnicode() === "♔");
test("Piece color property", $wk->color->isWhite());
test("Piece role property", $wk->role->value() === \shakmaty\Role::KING);

$bp = \shakmaty\Piece::fromChar('p');
test("Piece::fromChar('p')", $bp->color->isBlack() && $bp->role->value() === \shakmaty\Role::PAWN);

// legacyCode / fromLegacyCode (Step 3)
test("Piece::legacyCode White King", \shakmaty\Piece::legacyCode($wk->color, $wk->role) === 1);
test("Piece::legacyCode Black Pawn", \shakmaty\Piece::legacyCode($bp->color, $bp->role) === 14);
test("Piece::fromLegacyCode 1 White King", (function () {
    $p = \shakmaty\Piece::fromLegacyCode(1);
    return $p !== null && $p->color->isWhite() && $p->role->value() === \shakmaty\Role::KING;
})());
test("Piece::fromLegacyCode NOPIECE null", \shakmaty\Piece::fromLegacyCode(7) === null);

// ---- Step 4: Bitboard ----
$bb = \shakmaty\Bitboard::fromSquare(\shakmaty\Square::E4);
test("Bitboard fromSquare has E4", $bb->has(\shakmaty\Square::E4));
test("Bitboard fromSquare count", $bb->count() === 1);
test("Bitboard !has A1", !$bb->has(\shakmaty\Square::A1));
test("Bitboard any", $bb->any());
test("Bitboard !isEmpty", !$bb->isEmpty());

$empty_bb = new \shakmaty\Bitboard(\shakmaty\Bitboard::EMPTY);
test("Bitboard EMPTY isEmpty", $empty_bb->isEmpty());

$rank1_bb = \shakmaty\Bitboard::fromRank(\shakmaty\Rank::FIRST);
test("Bitboard fromRank count", $rank1_bb->count() === 8);

$file_a_bb = \shakmaty\Bitboard::fromFile(\shakmaty\File::A);
test("Bitboard fromFile count", $file_a_bb->count() === 8);

$bb_or = $bb->bitwiseOr($rank1_bb);
test("Bitboard bitwiseOr", $bb_or->count() === 9);

$bb_and = $bb->bitwiseAnd($rank1_bb);
test("Bitboard bitwiseAnd empty", $bb_and->isEmpty());

$bb_not = $bb->bitwiseNot();
test("Bitboard bitwiseNot", !$bb_not->has(\shakmaty\Square::E4) && $bb_not->has(\shakmaty\Square::A1));

$bb_shift = $bb->shift(8);
test("Bitboard shift", $bb_shift->has(\shakmaty\Square::E5) && !$bb_shift->has(\shakmaty\Square::E4));

$all_bb = new \shakmaty\Bitboard(\shakmaty\Bitboard::ALL);
test("Bitboard ALL count", $all_bb->count() === 64);

$corners = new \shakmaty\Bitboard(\shakmaty\Bitboard::CORNERS);
test("Bitboard CORNERS count", $corners->count() === 4);

$backranks = new \shakmaty\Bitboard(\shakmaty\Bitboard::BACKRANKS);
test("Bitboard BACKRANKS count", $backranks->count() === 16);

// ---- Step 5: Board ----
$board = new \shakmaty\Board();
test("Board start K at E1", $board->pieceAt(\shakmaty\Square::E1) === 'K');
test("Board start Q at D1", $board->pieceAt(\shakmaty\Square::D1) === 'Q');
test("Board start null at E4", $board->pieceAt(\shakmaty\Square::E4) === null);
test("Board occupied count", $board->occupied()->count() === 32);
test("Board white count", $board->byColor(1)->count() === 16);
test("Board black count", $board->byColor(0)->count() === 16);
test("Board pawns count", $board->byRole(\shakmaty\Role::PAWN)->count() === 16);
test("Board knights count", $board->byRole(\shakmaty\Role::KNIGHT)->count() === 4);
test("Board roleAt E1", $board->roleAt(\shakmaty\Square::E1) === \shakmaty\Role::KING);
test("Board colorAt E1", $board->colorAt(\shakmaty\Square::E1) === 1);

// legacyPieceAt (Step 5)
test("Board legacyPieceAt E1 (White King)", $board->legacyPieceAt(\shakmaty\Square::E1) === 1);
test("Board legacyPieceAt E7 (Black Pawn)", $board->legacyPieceAt(\shakmaty\Square::E7) === 14);
test("Board legacyPieceAt E4 (NOPIECE)", $board->legacyPieceAt(\shakmaty\Square::E4) === 7);

$empty_board = \shakmaty\Board::empty();
test("Board empty", $empty_board->occupied()->isEmpty());

// ---- Step 6: Setup ----
$setup = new \shakmaty\Setup();
test("Setup turn", $setup->turn === 1);
test("Setup halfmoves", $setup->halfmoves === 0);
test("Setup fullmoves", $setup->fullmoves === 1);
test("Setup board has K at E1", $setup->board->pieceAt(\shakmaty\Square::E1) === 'K');
test("Setup castling rights count", $setup->castlingRights->count() === 4);
test("Setup epSquare null", $setup->epSquare === null);

$setup->turn = 0;
test("Setup turn writable", $setup->turn === 0);
$setup->halfmoves = 5;
test("Setup halfmoves writable", $setup->halfmoves === 5);
$setup->epSquare = \shakmaty\Square::E3;
test("Setup ep_square writable", $setup->epSquare === \shakmaty\Square::E3);
$setup->epSquare = null;
test("Setup ep_square clear", $setup->epSquare === null);

$setupFromFen = \shakmaty\Setup::fromFen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
test("Setup fromFen turn", $setupFromFen->turn === 1);
test("Setup fromFen round-trip", (string) \shakmaty\Fen::fromSetup($setupFromFen) === "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
test("Setup fromFen crazyhouse pocket", \shakmaty\Setup::fromFen("rnbqkbnr/ppp1pppp/8/8/8/8/PPPPPPPP/RNBQKBNR[P] w KQkq - 0 1")->pockets !== null);
test("Setup fromFen invalid throws ParseFenError", (function () {
    try { \shakmaty\Setup::fromFen('invalid'); } catch (\shakmaty\fen\ParseFenError $e) { return true; }
    return false;
})());

// ---- Step 8: Chess ----
$pos = new \shakmaty\Chess();
test("Chess new turn", $pos->turn() === 1);
test("Chess legalMovesCount 20", $pos->legalMovesCount() === 20);
test("Chess !isCheck", !$pos->isCheck());
test("Chess !isCheckmate", !$pos->isCheckmate());
test("Chess outcome *", $pos->outcome() === '*');
test("Chess epSquare null", $pos->epSquare() === null);

$pos->playSan("e4");
test("Chess playSan e4 turn black", $pos->turn() === 0);
test("Chess playSan e4 halfmoves", $pos->halfmoves() === 0);

$pos->playSan("e5");
$pos->playSan("Qh5");
$pos->playSan("Nc6");
$pos->playSan("Bc4");
$pos->playSan("Nf6");
$pos->playSan("Qxf7");
test("Chess Scholar's mate isCheckmate", $pos->isCheckmate());
test("Chess Scholar's mate outcome 1-0", $pos->outcome() === '1-0');

// FEN from Chess
$fen_str = $pos->toFen();
$pos2 = \shakmaty\Chess::fromFen($fen_str);
test("Chess FEN roundtrip same checkmate", $pos2->isCheckmate());

// FEN with checkmate (Scholar's mate position)
$pos3 = \shakmaty\Chess::fromFen("r1bqkb1r/pppp1Qpp/2n2n2/4p3/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 4");
test("Chess fromFen checkmate", $pos3->isCheckmate());
test("Chess fromFen outcome 1-0", $pos3->outcome() === '1-0');

// UCI play
$pos4 = new \shakmaty\Chess();
$pos4->playUci("e2e4");
test("Chess playUci e2e4 turn", $pos4->turn() === 0);

// legalMoves as MoveList
$moves = $pos4->legalMoves();
test("Chess legalMoves returns MoveList", $moves->count() > 0);

// ---- Steps 9-11: FEN, SAN, UCI ----
$fen = \shakmaty\Fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
test("Fen parse + __toString roundtrip", (string) $fen === "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
test("Fen empty", (string) \shakmaty\Fen::empty() === "8/8/8/8/8/8/8/8 w - - 0 1");
test("Fen isValid static", \shakmaty\Fen::isValid("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"));
test("Fen !isValid static", !\shakmaty\Fen::isValid("invalid"));
test("Fen parse throws ParseFenError", (function () {
    try { \shakmaty\Fen::parse('invalid'); } catch (\shakmaty\fen\ParseFenError $e) { return true; }
    return false;
})());
test("Fen getSetup returns Setup", $fen->getSetup() instanceof \shakmaty\Setup);
test("Fen getPosition returns VariantPosition(Chess)", (function () use ($fen) {
    $p = $fen->getPosition();
    return $p instanceof \shakmaty\VariantPosition && $p->variant()->value() === \shakmaty\Variant::CHESS;
})());
test("Fen fromPosition", str_starts_with((string) \shakmaty\Fen::fromPosition(new \shakmaty\Chess()), "rnbqkbnr/pppppppp"));
test("Fen fromSetup", str_starts_with((string) \shakmaty\Fen::fromSetup(new \shakmaty\Setup()), "rnbqkbnr/pppppppp"));

$san = new \shakmaty\San("Nf3");
test("San __toString", $san->__toString() === "Nf3");

// San <-> Move conversion
$sanStart = new \shakmaty\Chess();
$sanMoves = $sanStart->legalMoves();
$sanNf3 = null;
for ($i = 0; $i < $sanMoves->count(); $i++) {
    if ($sanMoves->get($i)->toLong() === 'Ng1f3') { $sanNf3 = $sanMoves->get($i); }
}
test("San::fromMove", \shakmaty\San::fromMove($sanStart, $sanNf3)->__toString() === 'Nf3');
test("San::toMove", (new \shakmaty\San('Nf3'))->toMove($sanStart)->toLong() === 'Ng1f3');
test("San::findMove", (new \shakmaty\San('Nf3'))->findMove($sanMoves)->toLong() === 'Ng1f3');
test("San::matches", (new \shakmaty\San('Nf3'))->matches($sanNf3) === true);

$uci = new \shakmaty\Uci("e2e4");
test("Uci isValid e2e4", $uci->isValid());
test("Uci e7e8q valid", (new \shakmaty\Uci("e7e8q"))->isValid());
$bad_uci = new \shakmaty\Uci("xxx");
test("Uci !isValid xxx", !$bad_uci->isValid());

// ---- Step 14: PGN reader ----
$pgnText = "[White \"Alice\"]\n\n1. e4 e5 2. Nf3 Nc6 1-0";
$pgnReader = new \shakmaty\pgn\Reader($pgnText);
$pgnGame = $pgnReader->readGame();
test("PGN readGame returns Game", $pgnGame instanceof \shakmaty\pgn\Game);
test("PGN tag White", $pgnGame->tag('White') === 'Alice');
test("PGN outcome", $pgnGame->outcome() === '1-0');
test("PGN mainline", $pgnGame->mainline() === ['e4', 'e5', 'Nf3', 'Nc6']);
test("PGN movetext token count", count($pgnGame->movetext()) === 5);
$pgnReader2 = new \shakmaty\pgn\Reader($pgnText);
test("PGN readAll count", count($pgnReader2->readAll()) === 1);

// SanPlus moved to the root namespace (no longer shakmaty\pgn\SanPlus)
$sanPlusMove = \shakmaty\SanPlus::fromAscii('Qxf7#');
test("SanPlus root namespace fromAscii", $sanPlusMove->san() === 'Qxf7' && $sanPlusMove->isCheckmate());
test("SanPlus toSan returns San", $sanPlusMove->toSan() instanceof \shakmaty\San);
test("SanPlus old pgn namespace removed", !class_exists('shakmaty\\pgn\\SanPlus'));

// ---- Step 13: Attacks ----
$occ = new \shakmaty\Bitboard(0x3f7f28802826f5b9);
test("Attacks bishopAttacks magic", \shakmaty\Attacks::bishopAttacks(\shakmaty\Square::D6, $occ)->toU64() === 0x0014001422010000);
test("Attacks rookAttacks magic", \shakmaty\Attacks::rookAttacks(\shakmaty\Square::D6, $occ)->toU64() === 0x0008370808000000);
test("Attacks queenAttacks = rook|bishop", \shakmaty\Attacks::queenAttacks(\shakmaty\Square::D6, $occ)->toU64() === (0x0008370808000000 | 0x0014001422010000));

$pawnAtt = \shakmaty\Attacks::pawnAttacks(\shakmaty\Color::WHITE, \shakmaty\Square::E2);
test("Attacks pawnAttacks count", $pawnAtt->count() === 2 && $pawnAtt->has(\shakmaty\Square::D3) && $pawnAtt->has(\shakmaty\Square::F3));

$knightAtt = \shakmaty\Attacks::knightAttacks(\shakmaty\Square::G1);
test("Attacks knightAttacks count", $knightAtt->count() === 3 && $knightAtt->has(\shakmaty\Square::H3));

test("Attacks kingAttacks count", \shakmaty\Attacks::kingAttacks(\shakmaty\Square::E1)->count() === 5);

$emptyBb = new \shakmaty\Bitboard(\shakmaty\Bitboard::EMPTY);
$rookPiece = new \shakmaty\Piece(\shakmaty\Color::WHITE, \shakmaty\Role::ROOK);
test("Attacks dispatch by piece", \shakmaty\Attacks::attacks(\shakmaty\Square::D6, $rookPiece, $occ)->toU64() === 0x0008370808000000);
$knightPiece = new \shakmaty\Piece(\shakmaty\Color::BLACK, \shakmaty\Role::KNIGHT);
test("Attacks knight ignores occupancy", \shakmaty\Attacks::attacks(\shakmaty\Square::E4, $knightPiece, $emptyBb)->toU64() === \shakmaty\Attacks::knightAttacks(\shakmaty\Square::E4)->toU64());

$rayBb = \shakmaty\Attacks::ray(\shakmaty\Square::E2, \shakmaty\Square::G4);
test("Attacks ray", $rayBb->has(\shakmaty\Square::F3) && $rayBb->has(\shakmaty\Square::G4));

$betweenBb = \shakmaty\Attacks::between(\shakmaty\Square::B1, \shakmaty\Square::B7);
test("Attacks between", $betweenBb->count() === 5 && $betweenBb->has(\shakmaty\Square::B2) && !$betweenBb->has(\shakmaty\Square::B7));

test("Attacks aligned true", \shakmaty\Attacks::aligned(\shakmaty\Square::A1, \shakmaty\Square::B2, \shakmaty\Square::C3));
test("Attacks aligned false", !\shakmaty\Attacks::aligned(\shakmaty\Square::A1, \shakmaty\Square::B2, \shakmaty\Square::C4));
test("Attacks out-of-range empty", \shakmaty\Attacks::knightAttacks(99)->isEmpty());

// ---- Step 16: Position (abstract base class) ----
$posRc = new ReflectionClass('shakmaty\\Position');
test("Position is abstract", $posRc->isAbstract());
$chessPos = new \shakmaty\Chess();
test("Chess instanceof Position", $chessPos instanceof \shakmaty\Position);
test("Position us count", $chessPos->us()->count() === 16);
test("Position them count", $chessPos->them()->count() === 16);
test("Position our pawns", $chessPos->our(\shakmaty\Role::PAWN)->count() === 8);
test("Position !isCheck", !$chessPos->isCheck());
test("Position !isGameOver", !$chessPos->isGameOver());
test("Position outcome *", $chessPos->outcome() === '*');
test("Position maybeEpSquare null", $chessPos->maybeEpSquare() === null);
test("Position !isVariantEnd", $chessPos->isVariantEnd() === false);

// ---- Phase 6: Variant & VariantPosition ----
$variant = new \shakmaty\Variant(\shakmaty\Variant::CRAZYHOUSE);
test("Variant value", $variant->value() === \shakmaty\Variant::CRAZYHOUSE);
test("Variant uci", $variant->uci() === 'crazyhouse');
test("Variant distinguishesPromoted", $variant->distinguishesPromoted() === true);
test("Variant fromUci", \shakmaty\Variant::fromUci('atomic')->value() === \shakmaty\Variant::ATOMIC);
test("Variant fromUci unknown null", \shakmaty\Variant::fromUci('nope') === null);
test("Variant fromAscii alias", \shakmaty\Variant::fromAscii('Chess960')->value() === \shakmaty\Variant::CHESS);
test("Variant all count", count(\shakmaty\Variant::all()) === 8);

$vp = new \shakmaty\VariantPosition(\shakmaty\Variant::ATOMIC);
test("VariantPosition variant", $vp->variant()->value() === \shakmaty\Variant::ATOMIC);
test("VariantPosition instanceof Position", $vp instanceof \shakmaty\Position);
test("VariantPosition legalMovesCount", $vp->legalMovesCount() === 20);
$vp->playSan('e4');
test("VariantPosition playSan turn", $vp->turn() === 0);
test("VariantPosition toFen", str_contains($vp->toFen(), '4P3'));
$vpDef = new \shakmaty\VariantPosition();
test("VariantPosition default chess", $vpDef->variant()->value() === \shakmaty\Variant::CHESS);

// ---- Phase 7: Pockets, RemainingChecks, drop moves ----
$rcChecks = new \shakmaty\RemainingChecks(3);
test("RemainingChecks value", $rcChecks->value() === 3);
test("RemainingChecks saturatingSub", $rcChecks->saturatingSub(2)->value() === 1);
test("RemainingChecks isZero", (new \shakmaty\RemainingChecks(0))->isZero() === true);

$counts = new \shakmaty\ByRole();
$counts->set(\shakmaty\Role::KNIGHT, 2);
test("ByRole get/set", $counts->get(\shakmaty\Role::KNIGHT) === 2 && $counts->knight === 2);
test("ByRole total", $counts->total() === 2);

$pockets = new \shakmaty\Pockets();
$pockets->setCount(\shakmaty\Color::WHITE, \shakmaty\Role::PAWN, 1);
test("Pockets count", $pockets->count(\shakmaty\Color::WHITE, \shakmaty\Role::PAWN) === 1);
test("Pockets total", $pockets->total() === 1);

$drop = \shakmaty\Move::fromPut(\shakmaty\Role::KNIGHT, \shakmaty\Square::F3);
test("Move::fromPut isPut", $drop->isPut() === true);
test("Move::fromPut toLong", $drop->toLong() === 'N@f3');

$czFen = "rnbqkbnr/ppp1pppp/8/8/8/8/PPPPPPPP/RNBQKBNR[P] w KQkq - 0 1";
$czPos = \shakmaty\VariantPosition::fromFen(\shakmaty\Variant::CRAZYHOUSE, $czFen, \shakmaty\CastlingMode::STANDARD);
test("Crazyhouse pockets", $czPos->pockets()->count(\shakmaty\Color::WHITE, \shakmaty\Role::PAWN) === 1);
$czPos->playUci('P@e4');
test("Crazyhouse drop", $czPos->turn() === 0 && $czPos->pockets()->isEmpty());

$tcPos = \shakmaty\VariantPosition::fromFen(\shakmaty\Variant::THREE_CHECK, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 2+3 0 1", \shakmaty\CastlingMode::STANDARD);
test("ThreeCheck remainingChecks", $tcPos->remainingChecks(\shakmaty\Color::WHITE)->value() === 2);
test("Chess no pockets", (new \shakmaty\Chess())->pockets() === null);
test("Chess no remainingChecks", (new \shakmaty\Chess())->remainingChecks(\shakmaty\Color::WHITE) === null);

// ---- Phase 8: Zobrist64 hashing ----
$zStart = (new \shakmaty\Chess())->zobristHash();
test("Zobrist64 instance", $zStart instanceof \shakmaty\Zobrist64);
test("Zobrist64 start hash", $zStart->toHex() === '463b96181691fc9c');
test("Zobrist64 fromHex roundtrip", \shakmaty\Zobrist64::fromHex($zStart->toHex())->equals($zStart));
test("Zobrist64 xor self is zero", $zStart->bitwiseXor($zStart)->isZero());
test("Zobrist64 legacy24 top bits", $zStart->legacy24() === 0x463b96);
test("Zobrist64 legacy24 Polyglot sample", \shakmaty\Zobrist64::fromHex('9d39247e33776d41')->legacy24() === 0x9d3924);
test("Zobrist64 legacy24 logical shift", \shakmaty\Zobrist64::fromHex('ffffffffffffffff')->legacy24() === 0xffffff);
test("Zobrist64 legacy24 zero", (new \shakmaty\Zobrist64())->legacy24() === 0);

$zPos = new \shakmaty\Chess();
$zPos->playUci('g1f3');
test("Zobrist changes after move", !$zPos->zobristHash()->equals($zStart));

$zP = new \shakmaty\Chess();
$zLegal = $zP->legalMoves();
$zNf3 = null;
$zE4 = null;
for ($i = 0; $i < $zLegal->count(); $i++) {
    if ($zLegal->get($i)->toLong() === 'Ng1f3') { $zNf3 = $zLegal->get($i); }
    if ($zLegal->get($i)->toLong() === 'e2e4') { $zE4 = $zLegal->get($i); }
}
test("Zobrist incremental update", $zP->updateZobristHash($zP->zobristHash(), $zNf3)->equals($zP->play($zNf3)->zobristHash()));
test("Zobrist update null for double push", $zP->updateZobristHash($zP->zobristHash(), $zE4) === null);

$zVp = new \shakmaty\VariantPosition(\shakmaty\Variant::CHESS);
test("VariantPosition zobrist matches Chess", $zVp->zobristHash()->equals($zStart));
$zTc = \shakmaty\VariantPosition::fromFen(\shakmaty\Variant::THREE_CHECK, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 2+3 0 1", \shakmaty\CastlingMode::STANDARD);
test("VariantPosition variant state in hash", !$zTc->zobristHash()->equals($zStart));

echo "\n==============================\n";
echo "Integration test results:\n";
echo "  Passed: $pass\n";
echo "  Failed: $fail\n";
echo "==============================\n";
if ($fail > 0) {
    exit(1);
}
echo "All integration tests passed!\n";
