<?php
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

// MoveList
$list = new \shakmaty\MoveList();
assert($list->count() === 0);

echo "Step 7: All move tests passed!\n";
