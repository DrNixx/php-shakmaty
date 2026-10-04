# Bitboard Reference

This document covers `Bitboard` — a 64-bit integer wrapper used throughout `shakmaty` to represent sets of squares efficiently. A bitboard uses one bit per square (bit *i* corresponds to the square with index *i*), enabling fast set operations via bitwise logic. All method names follow camelCase convention.

---

## Overview
A **Bitboard** is a 64-bit value where each bit position maps directly to a chess board square:
- Bit `0` → Square A1 (index 0)
- Bit `28` → Square E4 (index 28)
- …
- Bit `63` → Square H8 (index 63)

This representation allows the engine to perform set operations — union, intersection, difference, complement — in a single CPU instruction rather than iterating over individual squares. Use this class for:
- **Occupancy tracking** (`Board::occupied()`)
- **Color/role masks** (`Board::byColor(1)`, `Board::byRole(\shakmaty\Role::PAWN)`)
- **Castling rights** (4 bits, one per rook corner square: A1/H1/A8/H8)
- **Attack and move generation** in the engine internals

---

## Constructor
```php
new \shakmaty\Bitboard(?int $rawValue = 0)
//   rawValue: a signed PHP int representing the 64-bit value (default: EMPTY / all bits clear)
```

> **Note:** Because PHP integers are platform-dependent in width, `toU64()` returns the unsigned representation. For most use cases you will construct Bitboards via factory methods rather than passing raw values directly.

---

## Factory Methods (Static)

| Method                                    | Return Type     | Description                                              |
|-------------------------------------------|-----------------|----------------------------------------------------------|
| `\shakmaty\Bitboard::fromSquare(int $sq)`   | `static`  | Creates a Bitboard with exactly one bit set — the square at index `$sq`. Use Square constants: e.g. `\shakmaty\Square::E4`. |
| `\shakmaty\Bitboard::fromRank(int $rank)`   | `static`  | Creates a Bitboard covering all 8 squares on rank `$rank` (0=first … 7=eighth). Use Rank constants: e.g. `\shakmaty\Rank::FIRST`. |
| `\shakmaty\Bitboard::fromFile(int $file)`   | `static`  | Creates a Bitboard covering all 8 squares on file `$file` (0=A … 7=H). Use File constants: e.g. `\shakmaty\File::A`. |

---

## Query Methods

| Method          | Return Type     | Description                                              |
|-----------------|-----------------|----------------------------------------------------------|
| `count()`       | `int`           | Population count — the number of set bits (0–64).        |
| `any()`         | `bool`          | Returns `true` if at least one bit is set.               |
| `isEmpty()`     | `bool`          | Returns `true` if no bits are set (`count() === 0`).     |
| `has(int $squareIndex)` | `bool`   | Tests whether the square at index `$squareIndex` (use a Square constant) is present in this Bitboard. |
| `toU64()`       | `int`           | Returns the underlying unsigned 64-bit value as an int. Useful for serialization or debugging. |
| `__toString()`    | `string`        | Returns a 64-character binary string (bit 0 = leftmost char). All `'1'`s and `'0'`s — useful for visual inspection in debug output. PHP magic method, also available via the `(string)` cast. |

---

## Bitwise Operations

All bitwise operations return **new** Bitboard instances; the original is never mutated.

| Method                    | Return Type     | Description                                              |
|---------------------------|-----------------|----------------------------------------------------------|
| `bitwiseAnd(Bitboard $other)`  | `static`  | Intersection — bits set in both `$this` and `$other`.    |
| `bitwiseOr(Bitboard $other)`   | `static`  | Union — bits set in either `$this` or `$other`.          |
| `bitwiseXor(Bitboard $other)`  | `static`  | Symmetric difference — bits set in exactly one of the two. |
| `bitwiseNot()`           | `static`        | Complement within 64 bits (inverts all squares).         |

### Shift Operations

| Method              | Return Type     | Description                                              |
|---------------------|-----------------|----------------------------------------------------------|
| `shift(int $offset)` | `static`       | Returns a new Bitboard with all set bits shifted by `$offset` square indices. **Positive offset** shifts toward higher-index squares (toward Black's side / rank up). Bits that shift beyond bit 63 are discarded; no wrap-around occurs. Negative offsets shift in the opposite direction and may produce undefined behavior — use only non-negative values unless you understand the underlying implementation. |

> **Example:** `Bitboard::fromSquare(\shakmaty\Square::E4)->shift(8)` produces a Bitboard with E5 set (one rank up). A shift of 16 moves two ranks, etc.

---

## Constants

Predefined bitboards for common board regions:

| Constant                          | Description                                              |
|-----------------------------------|----------------------------------------------------------|
| `\shakmaty\Bitboard::EMPTY`       | All bits clear — no squares set (`count() === 0`).        |
| `\shakmaty\Bitboard::ALL`         | All 64 bits set — every square on the board.              |
| `\shakmaty\Bitboard::CORNERS`     | The four corner squares: A1, H1, A8, H8 (`count() === 4`). Used for castling rights in standard chess. |
| `\shakmaty\Bitboard::BACKRANKS`   | All squares on ranks 1 and 8 (the two back ranks). `count() === 16`. Useful as a mask when checking promotion or king safety. |
| `\shakmaty\Bitboard::LIGHT_SQUARES`   | All light-colored squares (`count() === 32`). Used in bishop attack generation.    |
| `\shakmaty\Bitboard::DARK_SQUARES`    | All dark-colored squares (`count() === 32`). Used in bishop attack generation.   |

---

## Examples

```php
$bb = \shakmaty\Bitboard::fromSquare(\shakmaty\Square::E4);
echo $bb->count(); // 1
$combined = $bb->bitwiseOr(\shakmaty\Bitboard::fromRank(\shakmaty\Rank::FIRST));
echo $combined->count(); // 9
var_dump($bb->shift(8)->has(\shakmaty\Square::E5)); // true

// Test membership with has() — A1 is not in the E4-only bitboard
$e4_only = \shakmaty\Bitboard::fromSquare(\shakmaty\Square::E4);
var_dump($e4_only->has(\shakmaty\Square::A1));   // false

// Use constants for common board regions
$corners  = new \shakmaty\Bitboard(\shakmaty\Bitboard::CORNERS);
echo $corners->count();    // int(4): A1, H1, A8, H8 — standard castling rights

$backranks = new \shakmaty\Bitboard(\shakmaty\Bitboard::BACKRANKS);
var_dump($backranks->has(\shakmaty\Square::E1)); // true (rank 1 is a back rank)
```
