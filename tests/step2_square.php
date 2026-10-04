<?php
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

// File tests
$f = new \shakmaty\File(\shakmaty\File::E);
assert($f->value() === 4);
assert($f->toChar() === 'e');
assert($f->upperChar() === 'E');

$fa = \shakmaty\File::fromChar('a');
assert($fa->value() === 0);

$fh = \shakmaty\File::fromChar('H');
assert($fh->value() === 7);

$dist = $fa->distance($fh);
assert($dist === 7);

// Rank tests
$r = new \shakmaty\Rank(\shakmaty\Rank::FIRST);
assert($r->value() === 0);
assert($r->toChar() === '1');

$r4 = new \shakmaty\Rank(\shakmaty\Rank::FOURTH);
assert($r4->value() === 3);
assert($r4->toChar() === '4');

$r8 = new \shakmaty\Rank(\shakmaty\Rank::EIGHTH);
assert($r8->value() === 7);
assert($r8->flipVertical()->value() === 0);

// Square tests
$sq = new \shakmaty\Square(\shakmaty\Square::E4);
assert($sq->value() === 28);

$sq2 = \shakmaty\Square::fromAscii("e4");
assert($sq2->value() === 28);
assert($sq2->file()->value() === \shakmaty\File::E);
assert($sq2->rank()->value() === \shakmaty\Rank::FOURTH);

$sq_a1 = \shakmaty\Square::fromAscii("a1");
assert($sq_a1->value() === 0);
assert($sq_a1->isDark());
assert(!$sq_a1->isLight());

$sq_h8 = new \shakmaty\Square(\shakmaty\Square::H8);
assert($sq_h8->value() === 63);
assert($sq_h8->flipHorizontal()->value() === 56); // H8->A8
assert($sq_h8->flipVertical()->value() === 7);    // H8->H1

// fromCoords
$sq3 = \shakmaty\Square::fromCoords(
    new \shakmaty\File(\shakmaty\File::D),
    new \shakmaty\Rank(\shakmaty\Rank::FIFTH)
);
assert($sq3->value() === 35);
assert($sq3->__toString() === 'd5');

// Null square
assert(\shakmaty\Square::NS === 64);

echo "Step 2: All square tests passed!\n";
