# Attacks Reference

This document covers the `Attacks` class — a static-only utility that exposes the attack and ray lookup tables of the underlying `shakmaty` engine. Use it to query which squares a piece attacks from a given square, to compute blocking-aware sliding attacks (bishop, rook, queen), and to reason about the alignment of squares. All method names follow camelCase convention.

---

## Overview

`Attacks` has no instance state: every lookup is a static call. Squares are passed as zero-based indices (`A1` = 0 … `H8` = 63); use the `\shakmaty\Square::*` constants for readability.

Two kinds of queries are provided:

- **Piece attacks** — `pawnAttacks`, `knightAttacks`, `kingAttacks`, and the sliding `bishopAttacks`, `rookAttacks`, `queenAttacks`. The `attacks()` dispatcher takes a `Piece` and routes to the matching method.
- **Square geometry** — `ray`, `between` and `aligned` describe rank / file / diagonal relationships between squares.

> **Out-of-range arguments never throw:** a square index outside `0..63` yields an empty `Bitboard` (or `false` for `aligned`).

---

## Constructor

```php
new \shakmaty\Attacks()
//   Constructs the (stateless) utility object. Every method is static, so
//   instantiating the class is optional.
```

---

## Piece Attack Methods (Static)

| Method                                                               | Return Type          | Description |
|----------------------------------------------------------------------|----------------------|-------------|
| `\shakmaty\Attacks::pawnAttacks(int $color, int $sq)`                | `\shakmaty\Bitboard` | Squares attacked by a pawn of `$color` (`0` = Black, `1` = White; use `\shakmaty\Color::*`) standing on `$sq`. |
| `\shakmaty\Attacks::knightAttacks(int $sq)`                          | `\shakmaty\Bitboard` | Squares attacked by a knight on `$sq`. |
| `\shakmaty\Attacks::kingAttacks(int $sq)`                            | `\shakmaty\Bitboard` | Squares attacked by a king on `$sq`. |
| `\shakmaty\Attacks::bishopAttacks(int $sq, Bitboard $occupied)`      | `\shakmaty\Bitboard` | Diagonal attacks of a bishop on `$sq`, blocked by `$occupied`. |
| `\shakmaty\Attacks::rookAttacks(int $sq, Bitboard $occupied)`        | `\shakmaty\Bitboard` | Orthogonal attacks of a rook on `$sq`, blocked by `$occupied`. |
| `\shakmaty\Attacks::queenAttacks(int $sq, Bitboard $occupied)`       | `\shakmaty\Bitboard` | Combined rook + bishop attacks from `$sq` (`= rookAttacks \| bishopAttacks`). |
| `\shakmaty\Attacks::attacks(int $sq, Piece $piece, Bitboard $occupied)` | `\shakmaty\Bitboard` | Dispatches by the `$piece` role; knight/king attacks ignore `$occupied`, sliding pieces honor it. |

> **Occupancy semantics:** pass the set of occupied squares in `$occupied` to block the sliding rays. The `ray`/`between`/`aligned` methods take no occupancy argument.

---

## Geometry Methods (Static)

| Method                                              | Return Type          | Description |
|-----------------------------------------------------|----------------------|-------------|
| `\shakmaty\Attacks::ray(int $a, int $b)`            | `\shakmaty\Bitboard` | The full rank, file or diagonal line through both `$a` and `$b` (both endpoints included). Empty if the squares are not aligned. |
| `\shakmaty\Attacks::between(int $a, int $b)`        | `\shakmaty\Bitboard` | Squares strictly between `$a` and `$b`, endpoints excluded. Empty if the squares are not on a common rank, file or diagonal. |
| `\shakmaty\Attacks::aligned(int $a, int $b, int $c)`| `bool`               | `true` if all three squares lie on a single rank, file or diagonal. `false` for any out-of-range index. |

---

## Examples

```php
// Pawn attacks
$wp = \shakmaty\Attacks::pawnAttacks(\shakmaty\Color::WHITE, \shakmaty\Square::E2);
echo $wp->count(); // 2 (D3, F3)

// Knight attacks
$kn = \shakmaty\Attacks::knightAttacks(\shakmaty\Square::G1);
echo $kn->count(); // 3 (E2, F3, H3)

// King attacks
$kg = \shakmaty\Attacks::kingAttacks(\shakmaty\Square::E1);
echo $kg->count(); // 5

// Sliding attacks with occupancy
$occ    = new \shakmaty\Bitboard(0x3f7f28802826f5b9);
$rook   = \shakmaty\Attacks::rookAttacks(\shakmaty\Square::D6, $occ);
$bishop = \shakmaty\Attacks::bishopAttacks(\shakmaty\Square::D6, $occ);
$queen  = \shakmaty\Attacks::queenAttacks(\shakmaty\Square::D6, $occ);
var_dump($queen->toU64() === ($rook->toU64() | $bishop->toU64())); // true

// Dispatch by piece
$piece = new \shakmaty\Piece(\shakmaty\Color::WHITE, \shakmaty\Role::ROOK);
var_dump(\shakmaty\Attacks::attacks(\shakmaty\Square::D6, $piece, $occ)->toU64() === $rook->toU64()); // true

// Geometry
$ray = \shakmaty\Attacks::ray(\shakmaty\Square::E2, \shakmaty\Square::G4);
var_dump($ray->has(\shakmaty\Square::F3)); // true

$between = \shakmaty\Attacks::between(\shakmaty\Square::B1, \shakmaty\Square::B7);
echo $between->count(); // 5

var_dump(\shakmaty\Attacks::aligned(\shakmaty\Square::A1, \shakmaty\Square::B2, \shakmaty\Square::C3)); // true
var_dump(\shakmaty\Attacks::aligned(\shakmaty\Square::A1, \shakmaty\Square::B2, \shakmaty\Square::C4)); // false

// Out-of-range is safe
var_dump(\shakmaty\Attacks::knightAttacks(99)->isEmpty()); // true
```

---

## See Also

- Original Rust module: [`shakmaty::attacks`](https://docs.rs/shakmaty/0.30/shakmaty/attacks/index.html)
- [Bitboard](bitboard.md) — the returned square sets
- [Square](square.md) — coordinate constants used as arguments
- [Piece](piece.md) — argument to the `attacks()` dispatcher
