<?php
/** Step 14: shakmaty\pgn\Reader (integration) */
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

$pgn = <<<'PGN'
[Event "Test"]
[Site "?"]
[White "Alice"]
[Black "Bob"]
[Result "1-0"]

1. e4 e5 2. Nf3 (2. f4?! {King's gambit}) 2... Nc6 3. Bb5 a6 $1 1-0
PGN;

$reader = new \shakmaty\pgn\Reader($pgn);

assert($reader->hasMore() === true);
assert($reader->offset() === 0);

$game = $reader->readGame();
assert($game instanceof \shakmaty\pgn\Game);

// Tags
$tags = $game->tags();
assert($tags['Event'] === 'Test');
assert($tags['White'] === 'Alice');
assert($tags['Black'] === 'Bob');
assert($tags['Result'] === '1-0');
assert($game->tag('White') === 'Alice');
assert($game->tag('Missing') === null);

// Outcome + mainline
assert($game->outcome() === '1-0');
assert($game->mainline() === ['e4', 'e5', 'Nf3', 'Nc6', 'Bb5', 'a6']);

// Movetext tokens
$tokens = $game->movetext();
$kinds = array_map(fn($t) => $t->kind(), $tokens);
assert($kinds[0] === 'san');
assert($tokens[0]->san()->__toString() === 'e4');
assert(in_array('variation_begin', $kinds, true));
assert(in_array('variation_end', $kinds, true));

$commentFound = false;
$nagFound = false;
$outcomeFound = false;
foreach ($tokens as $token) {
    if ($token->kind() === 'comment' && $token->comment() === "King's gambit") {
        $commentFound = true;
    }
    if ($token->kind() === 'nag' && $token->nag()->value === 1) {
        $nagFound = true;
    }
    if ($token->kind() === 'outcome' && $token->outcome() === '1-0') {
        $outcomeFound = true;
    }
}
assert($commentFound);
assert($nagFound);
assert($outcomeFound);

// Exhaustion
assert($reader->readGame() === null);
assert($reader->hasMore() === false);

// ---- Multiple games ----
$multi = <<<'PGN'
[White "A"]

1. e4 *

[White "B"]

1. d4 1/2-1/2
PGN;

$r2 = new \shakmaty\pgn\Reader($multi);
$g1 = $r2->readGame();
assert($g1->tag('White') === 'A');
assert($g1->outcome() === '*');
$g2 = $r2->readGame();
assert($g2->tag('White') === 'B');
assert($g2->outcome() === '1/2-1/2');
assert($r2->readGame() === null);

// skipGame
$r3 = new \shakmaty\pgn\Reader($multi);
assert($r3->skipGame() === true);
$g = $r3->readGame();
assert($g->tag('White') === 'B');
assert($r3->skipGame() === false);

// readAll
$r4 = new \shakmaty\pgn\Reader($multi);
$all = $r4->readAll();
assert(count($all) === 2);
assert($all[0]->tag('White') === 'A');
assert($all[1]->tag('White') === 'B');

// Empty document
$r5 = new \shakmaty\pgn\Reader('');
assert($r5->hasMore() === false);
assert($r5->readGame() === null);
assert($r5->readAll() === []);

// reset
$r6 = new \shakmaty\pgn\Reader($multi);
$r6->readGame();
assert($r6->offset() > 0);
$r6->reset();
assert($r6->offset() === 0);
assert($r6->readGame()->tag('White') === 'A');

echo "Step 14 (pgn): Reader integration tests passed!\n";
