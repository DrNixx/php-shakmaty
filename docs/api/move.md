# Move & MoveList API Reference

## Move class

Constructor exists but creates a placeholder — `Move` objects are obtained from `Chess::legalMoves()`.

### Methods

| Method | Return Type | Description |
|--------|-------------|-------------|
| `role()` | `int` | Role of moving piece (1=Pawn, 2=Knight, 3=Bishop, 4=Rook, 5=Queen, 6=King) |
| `fromSq()` | `?int` | Source square index or null |
| `toSq()` | `int` | Target square index |
| `capture()` | `?int` | Captured piece role or null if no capture |
| `isCapture()` | `bool` | Whether this move captures a piece |
| `isCastle()` | `bool` | Whether this is a castling move |
| `isEnPassant()` | `bool` | Whether the capture was en passant |
| `isPromotion()` | `bool` | Whether a pawn promotes on this move |
| `promotion()` | `?int` | Promotion role or null if not promoting |
| `toLong()` | `string` | Long algebraic notation (e.g. `"e2e4"`); Crazyhouse drops render as `"N@f3"` / `"@e4"` |
| `isPut()` | `bool` | Whether this is a Crazyhouse piece drop (e.g. `N@f3`) |
| `Move::fromPut(int $role, int $to)` | `\shakmaty\Move` | Creates a Crazyhouse drop move; throws on invalid role/square |

## MoveList class

Constructor: `new \shakmaty\MoveList()` — empty list. Normally obtained from `Chess::legalMoves()`.

### Methods

| Method | Return Type | Description |
|--------|-------------|-------------|
| `count()` | `int` | Number of moves in the list |
| `get(int $index)` | `\shakmaty\Move \| null` | Move at index or null if out of range (0-based) |

## Example

```php
$pos = new \shakmaty\Chess();
$moves = $pos->legalMoves();
echo $moves->count(); // 20
$first = $moves->get(0);
echo $first->role(); // 1 (Pawn)
echo $first->toLong(); // e.g. "a2a3"
```

### Crazyhouse piece drops (Crazyhouse)

Drops are created directly with `Move::fromPut()` or read from a position's legal moves; they report via `isPut()`.

```php
$drop = \shakmaty\Move::fromPut(\shakmaty\Role::KNIGHT, \shakmaty\Square::F3);
echo $drop->toLong();      // "N@f3"
var_dump($drop->isPut());  // true

$pawnDrop = \shakmaty\Move::fromPut(\shakmaty\Role::PAWN, \shakmaty\Square::E4);
echo $pawnDrop->toLong();  // "@e4" (pawns drop without a letter)
```
