# Scholar's Mate — Complete Example

This example demonstrates Scholar's Mate (also known as the "Four-Move Checkmate")
using the `shakmaty` extension.

## Full Script

```php
<?php
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}
$pos = new \shakmaty\Chess();
```

Execute the moves one by one, printing the FEN position after each move.

```php
echo "Initial position:\n";
echo $pos->toFen() . "\n";
echo "Legal moves: " . $pos->legalMovesCount() . "\n";
```

### Step-by-step Move Execution

```php
// 1. e4
$pos->playSan('e4');
echo "After 1. e4:\n";
echo $pos->toFen() . "\n";

// 1... e5
$pos->playSan('e5');
echo "After 1... e5:\n";
echo $pos->toFen() . "\n";

// 2. Qh5
$pos->playSan('Qh5');
echo "After 2. Qh5:\n";
echo $pos->toFen() . "\n";

// 2... Nc6
$pos->playSan('Nc6');
echo "After 2... Nc6:\n";
echo $pos->toFen() . "\n";

// 3. Bc4
$pos->playSan('Bc4');
echo "After 3. Bc4:\n";
echo $pos->toFen() . "\n";

// 3... Nf6
$pos->playSan('Nf6');
echo "After 3... Nf6:\n";
echo $pos->toFen() . "\n";

// 4. Qxf7#
$pos->playSan('Qxf7');
echo "After 4. Qxf7#:\n";
echo $pos->toFen() . "\n";
```

### Final Position Analysis

```php
echo "\n=== Final Analysis ===\n";
echo "Checkmate: " . ($pos->isCheckmate() ? 'true' : 'false') . "\n";
echo "Check: " . ($pos->isCheck() ? 'true' : 'false') . "\n";
echo "Stalemate: " . ($pos->isStalemate() ? 'true' : 'false') . "\n";
echo "Outcome: " . $pos->outcome() . "\n";
echo "Turn: " . ($pos->turn() === 1 ? 'White' : 'Black') . "\n";
```

Expected output:

```
Checkmate: true
Check: true
Stalemate: false
Outcome: 1-0
Turn: Black
```

### Same Example Using UCI Notation

```php
$pos = new \shakmaty\Chess();
$pos->playUci('e2e4');
$pos->playUci('e7e5');
$pos->playUci('d1h5');  // Qh5 in UCI
$pos->playUci('b8c6');  // Nc6
$pos->playUci('f1c4');  // Bc4
$pos->playUci('g8f6');  // Nf6
$pos->playUci('h5f7');  // Qxf7#
echo "UCI Scholar's mate: " . $pos->outcome() . "\n";
```

### FEN Round-trip

```php
$fenStr = $pos->toFen();
$posRestored = \shakmaty\Chess::fromFen($fenStr);
echo "Restored checkmate: " . ($posRestored->isCheckmate() ? 'true' : 'false') . "\n";
```

### Using MoveList

```php
$moves = $pos->legalMoves();
echo "Legal moves in final position: " . $moves->count() . "\n";
for ($i = 0; $i < $moves->count(); $i++) {
    $move = $moves->get($i);
    echo "  {$i}: {$move->toLong()}\n";
}
```

## Full Script

Combine all the code blocks above into a single PHP script to run the complete example.
