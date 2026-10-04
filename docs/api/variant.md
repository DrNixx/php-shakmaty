# Variant and VariantPosition API Reference

Ports of [`shakmaty::variant`](https://docs.rs/shakmaty/0.30/shakmaty/variant/index.html):
the `Variant` discriminant and the dynamically dispatched `VariantPosition`.

## Variant

`shakmaty\Variant` identifies one of the eight variants. It mirrors the Rust
`shakmaty::variant::Variant` enum.

### Constants

| Constant | Value | UCI name |
|----------|-------|----------|
| `Variant::CHESS` | 0 | `chess` |
| `Variant::ATOMIC` | 1 | `atomic` |
| `Variant::ANTICHESS` | 2 | `antichess` |
| `Variant::KING_OF_THE_HILL` | 3 | `kingofthehill` |
| `Variant::THREE_CHECK` | 4 | `3check` |
| `Variant::CRAZYHOUSE` | 5 | `crazyhouse` |
| `Variant::RACING_KINGS` | 6 | `racingkings` |
| `Variant::HORDE` | 7 | `horde` |

### Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `__construct(int $value)` | `Variant` | Creates a variant; values `0..7`, out-of-range falls back to `Chess` |
| `value()` | `int` | Numeric discriminant |
| `uci()` | `string` | Canonical UCI name |
| `__toString()` | `string` | Alias of `uci()` (PHP magic method) |
| `distinguishesPromoted()` | `bool` | `true` only for `Crazyhouse` |
| `Variant::fromUci(string $name)` | `?Variant` | Selects by exact UCI name; `null` if unknown |
| `Variant::fromAscii(string $name)` | `?Variant` | Selects by name/alias (e.g. `Chess960`, `King of the Hill`); `null` if unknown |
| `Variant::all()` | `Variant[]` | All eight variants |

### Example

```php
$v = new \shakmaty\Variant(\shakmaty\Variant::ATOMIC);
echo $v->uci();                 // "atomic"

$v2 = \shakmaty\Variant::fromUci('3check');
echo $v2->value();              // 4

var_dump(\shakmaty\Variant::fromUci('nope')); // NULL
```

## VariantPosition

`shakmaty\VariantPosition` is a dynamically dispatched position that can hold
any of the eight variants. It extends the abstract [`Position`](position.md)
and implements its full contract, delegating to `shakmaty::Position`. A single
class wraps all variants; `variant()` reports which one.

### Constructors and factories

| Method | Returns | Description |
|--------|---------|-------------|
| `__construct(?int $variant = null)` | `VariantPosition` | Starting position for `$variant` (default `Chess`) |
| `VariantPosition::fromSetup(int $variant, Setup $setup, ?int $mode = null)` | `VariantPosition` | Builds from a `Setup`; throws `"Invalid position"`. `$mode`: `0` = Standard, `1` = Chess960 |
| `VariantPosition::fromFen(int $variant, string $fen, ?int $mode = null)` | `VariantPosition` | Builds from FEN; throws `"Invalid FEN"` / `"Invalid position"` |
| `variant()` | `Variant` | The concrete variant |
| `swapTurn()` | `VariantPosition` | Swaps the side to move; throws `"Swap failed"` if invalid |

### Position contract

All methods of [`Position`](position.md) are implemented (`board`, `turn`,
`legalMoves`, `isCheck`, `isCheckmate`, `isGameOver`, `outcome`, `play`, `us`,
`them`, `captureMoves`, `toSetup`, ...). `play(Move $m)` returns a new
`VariantPosition` and leaves the original unchanged.

Like every concrete position, `VariantPosition` supports [`zobristHash()`](zobrist.md) and [`updateZobristHash(...)`](zobrist.md), inherited from the abstract [`Position`](position.md). See [Zobrist64 API Reference](zobrist.md).

### VariantPosition-specific helpers

| Method | Returns | Description |
|--------|---------|-------------|
| `toFen()` | `string` | FEN of the current position |
| `legalMovesCount()` | `int` | Number of legal moves |
| `playSan(string $san)` | `bool` | Plays a SAN move (mutates); throws on invalid/illegal |
| `playUci(string $uci)` | `bool` | Plays a UCI move (mutates); throws on invalid/illegal |

### Example

```php
$pos = new \shakmaty\VariantPosition(\shakmaty\Variant::CRAZYHOUSE);
echo $pos->variant()->uci();      // "crazyhouse"
echo $pos->legalMovesCount();     // 20

$pos->playSan('e4');
echo $pos->turn();                // 0 (Black)

// King of the Hill ends when a king reaches the center
$hill = \shakmaty\VariantPosition::fromFen(
    \shakmaty\Variant::KING_OF_THE_HILL,
    "8/8/8/3K4/8/8/8/7k w - - 0 1",
    \shakmaty\CastlingMode::STANDARD
);
var_dump($hill->isVariantEnd());  // true
echo $hill->variantOutcome();     // "1-0"
```

### Variant state (Crazyhouse & Three-Check)

`VariantPosition` exposes variant-specific data through `pockets()` and `remainingChecks(color)` — both return **null** for variants that don't use them, so guard with a null check before dereferencing. See [Pockets, RemainingChecks & ByRole](pockets.md).

```php
// Crazyhouse pockets (one black pawn is off the board → in White's pocket)
$cz = \shakmaty\VariantPosition::fromFen(
    \shakmaty\Variant::CRAZYHOUSE,
    "rnbqkbnr/ppp1pppp/8/8/8/8/PPPPPPPP/RNBQKBNR[P] w KQkq - 0 1",
    \shakmaty\CastlingMode::STANDARD
);
echo $cz->pockets()->count(\shakmaty\Color::WHITE, \shakmaty\Role::PAWN); // 1

$cz->playUci('P@e4');                 // legal drop consumes the pocket pawn
var_dump($cz->pockets()->isEmpty());  // true

// Three-Check remaining checks (3+3 at start)
$tc = \shakmaty\VariantPosition::fromFen(
    \shakmaty\Variant::THREE_CHECK,
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 3+3 0 1",
    \shakmaty\CastlingMode::STANDARD
);
echo $tc->remainingChecks(\shakmaty\Color::WHITE)->value(); // 3

// Variants without this state return null:
var_dump((new \shakmaty\VariantPosition(\shakmaty\Variant::CHESS))->pockets());          // NULL
```

## Notes

`ParseVariantError` from the Rust source has no PHP counterpart: the factory
methods `fromUci()` / `fromAscii()` return `null` for unknown names, consistent
with `Role::fromChar()`.

The concrete variant types (`Atomic`, `Antichess`, ...) are not exposed as
separate PHP classes — they are all reached through `VariantPosition`.

## See also

- [`Position`](position.md) — abstract base class
- [`Chess`](chess.md) — standard chess position
