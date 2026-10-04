<?php
/** Step 12: shakmaty\pgn\Token (registration + API surface) */
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

assert(class_exists('shakmaty\\pgn\\Token'));

$rc = new ReflectionClass('shakmaty\\pgn\\Token');
foreach (['kind', 'san', 'nag', 'comment', 'outcome', '__toString'] as $method) {
    assert($rc->hasMethod($method));
}

echo "Step 12 (pgn): Token class registered!\n";
