<?php
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

// FEN
$fen = \shakmaty\Fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
assert((string) $fen === "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
assert(\shakmaty\Fen::isValid("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"));
assert(!\shakmaty\Fen::isValid("not valid fen"));

// SAN
$san = new \shakmaty\San("Nf3");
assert($san->__toString() === "Nf3");

$bad_san = new \shakmaty\San("invalid!");
assert(!$bad_san->isValid());

// UCI
$uci = new \shakmaty\Uci("e2e4");
assert($uci->isValid());
assert($uci->__toString() === "e2e4");

$uci2 = new \shakmaty\Uci("e7e8q");
assert($uci2->isValid());

$bad_uci = new \shakmaty\Uci("xxx");
assert(!$bad_uci->isValid());

echo "Step 9-11: All FEN, SAN, UCI tests passed!\n";
