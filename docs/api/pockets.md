# Pockets, RemainingChecks and ByRole API Reference

Data types that round out the Crazyhouse and Three-Check variants: piece
pockets, remaining-checks counters, and the underlying count containers.

## RemainingChecks

`shakmaty\RemainingChecks` is the number of checks a side still needs to give
to win a Three-Check game. Mirrors
[`shakmaty::RemainingChecks`](https://docs.rs/shakmaty/0.30/shakmaty/struct.RemainingChecks.html).

### Constants

| Constant | Value |
|----------|-------|
| `RemainingChecks::MAX` | `3` |

### Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `__construct(?int $value = null)` | `RemainingChecks` | Creates a value; clamped into `0..=3`; defaults to `3` |
| `value()` | `int` | Number of remaining checks (`0..=3`) |
| `isZero()` | `bool` | Whether no checks are left |
| `saturatingSub(int $n)` | `RemainingChecks` | Copy with `n` subtracted (saturating at zero) |

### Example

```php
$checks = new \shakmaty\RemainingChecks(3);
echo $checks->value();                    // 3
echo $checks->saturatingSub(2)->value();  // 1
var_dump((new \shakmaty\RemainingChecks(0))->isZero()); // true
```

## ByRole

`shakmaty\ByRole` holds an integer count for each piece role. Mirrors
`shakmaty::ByRole<u8>`. It is the per-color half of `Pockets`.

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `$pawn` / `$knight` / `$bishop` / `$rook` / `$queen` / `$king` | `int` | Read-only count for that role |

### Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `__construct()` | `ByRole` | All counts zero |
| `get(int $role)` | `int` | Count for a role (`1`=Pawn … `6`=King); `0` for an invalid role |
| `set(int $role, int $count)` | `void` | Sets a count (clamped to `0..=255`); invalid roles ignored |
| `total()` | `int` | Sum of all counts |
| `isEmpty()` | `bool` | Whether every count is zero |
| `toArray()` | `int[]` | `[pawn, knight, bishop, rook, queen, king]` |
| `copy()` | `ByRole` | Independent copy |

## Pockets

`shakmaty\Pockets` is a per-color container of `ByRole` counts — the Crazyhouse
pockets. Mirrors `shakmaty::ByColor<shakmaty::ByRole<u8>>`.

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `$white` / `$black` | `ByRole` | Per-role counts for a color (read-only) |

### Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `__construct()` | `Pockets` | Empty pockets |
| `Pockets::empty()` | `Pockets` | Empty pockets (alias) |
| `get(int $color)` | `ByRole` | Per-role counts for a color (`1`=White, `0`=Black) |
| `set(int $color, ByRole $counts)` | `void` | Replaces the counts for a color |
| `count(int $color, int $role)` | `int` | Count of a role for a color; `0` for an invalid role |
| `setCount(int $color, int $role, int $count)` | `void` | Sets a count (clamped `0..=255`) |
| `total()` | `int` | Total pieces in both pockets |
| `isEmpty()` | `bool` | Whether both pockets are empty |
| `copy()` | `Pockets` | Independent copy |

`get()` returns a copy; modify it and pass it back through `set()` to persist
changes:

```php
$pockets = new \shakmaty\Pockets();
$pockets->setCount(\shakmaty\Color::WHITE, \shakmaty\Role::PAWN, 1);

$white = $pockets->get(\shakmaty\Color::WHITE);
$white->set(\shakmaty\Role::KNIGHT, 1);
$pockets->set(\shakmaty\Color::WHITE, $white);

echo $pockets->total(); // 2
```

## Attaching pockets and checks to a position

`Setup` carries the same state, and `Position::toSetup()` / `fromSetup()` move it
between `Setup` and variant positions:

```php
// Crazyhouse with one pawn in White's pocket (note: one black pawn is off the board)
$cz = \shakmaty\VariantPosition::fromFen(
    \shakmaty\Variant::CRAZYHOUSE,
    "rnbqkbnr/ppp1pppp/8/8/8/8/PPPPPPPP/RNBQKBNR[P] w KQkq - 0 1",
    \shakmaty\CastlingMode::STANDARD
);
echo $cz->pockets()->count(\shakmaty\Color::WHITE, \shakmaty\Role::PAWN); // 1

$cz->playUci('P@e4');                 // legal drop
var_dump($cz->pockets()->isEmpty());  // true

// Three-Check
$tc = \shakmaty\VariantPosition::fromFen(
    \shakmaty\Variant::THREE_CHECK,
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 3+3 0 1",
    \shakmaty\CastlingMode::STANDARD
);
echo $tc->remainingChecks(\shakmaty\Color::WHITE)->value(); // 3
```

`pockets()` returns `null` for variants without pockets; `remainingChecks()`
returns `null` for variants without the Three-Check counter. In Crazyhouse the
global material limit counts board **and** both pockets together (≤16 pawns,
≤4 knights/bishops/rooks, ≤2 queens), so a pocket pawn requires the matching
pawn to be missing from the board.

## See also

- [`Setup`](setup.md) — carries `promoted`, `pockets`, `remainingChecks`
- [`Move`](move.md) — Crazyhouse drop moves (`fromPut`, `isPut`)
- [`Position`](position.md) — `pockets()`, `remainingChecks(color)`
- [`VariantPosition`](variant.md) — variant positions
