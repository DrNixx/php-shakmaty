<?php
/** Step 13: shakmaty\pgn\Game */
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

assert(class_exists('shakmaty\\pgn\\Game'));

$game = new \shakmaty\pgn\Game();
assert($game->tags() === []);
assert($game->tag('White') === null);
assert($game->movetext() === []);
assert($game->outcome() === '*');
assert($game->mainline() === []);

$rc = new ReflectionClass('shakmaty\\pgn\\Game');
foreach (['tags', 'tag', 'movetext', 'outcome', 'mainline'] as $method) {
    assert($rc->hasMethod($method));
}

echo "Step 13 (pgn): Game class tests passed!\n";
