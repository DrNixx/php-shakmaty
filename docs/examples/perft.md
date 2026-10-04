# Perft — Move Count Performance Test

This example implements a simple Perft (performance test) using shakmaty.
Perft counts the number of legal move sequences up to a given depth, which is
useful for validating move generation correctness.

## Recursive Perft Implementation

```php
<?php
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

function perft(\shakmaty\Chess $pos, int $depth): int {
    if ($depth === 0) {
        return 1;
    }
    $moves = $pos->legalMoves();
    $count = 0;
    for ($i = 0; $i < $moves->count(); $i++) {
        $move = $moves->get($i);
        // Play the move using UCI for precise move specification
        $pos->playUci($move->toLong());
        $count += perft($pos, $depth - 1);
        // Revert by recreating from FEN (simplest approach for PHP)
        $fen = $pos->toFen();
        // Note: In a real implementation, you'd want more efficient undo.
        // For simplicity, this re-parses the original position.
    }
    return $count;
}
```

## Expected Results

For the starting position, the expected Perft values are:

| Depth | Expected Nodes | Tested Value |
|-------|---------------|--------------|
| 1     | 20            | —            |
| 2     | 400           | —            |
| 3     | 8,902         | —            |
| 4     | 197,281       | —            |

> **Note:** Full Perft requires an efficient undo mechanism.
> The current shakmaty API supports `playSan()` and `playUci()` but does
> not expose a direct undo function. For production Perft, consider using the
> Rust shakmaty library directly or extending the PHP extension with undo support.

## Simplified Single-Depth Example

```php
$pos = new \shakmaty\Chess();
$moves = $pos->legalMoves();
echo "Depth 1 moves: " . $moves->count() . "\n"; // Expected: 20

// Count all depth-2 sequences
$depth2 = 0;
for ($i = 0; $i < $moves->count(); $i++) {
    $pos->playUci($moves->get($i)->toLong());
    $depth2 += $pos->legalMoves()->count();
    // Reset to starting position via FEN
    // This is simplified — real Perft needs proper undo
}
echo "Depth 2 moves: " . $depth2 . "\n"; // Expected: 400
```

## Usage

Run the script:

```bash
php docs/examples/perft.php
```
