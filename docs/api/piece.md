# Piece Reference

This document covers the `Piece` class — a composite value object that pairs a **Color** (White or Black) with a **Role** (Pawn through King). Use this class to represent any individual piece on the board, in move objects, or when parsing FEN / SAN notation. All method names follow camelCase convention.

---

## Overview
`Piece` is an immutable value object combining two other `shakmaty` classes:
- **Color** — White (`1`) or Black (`0`). Determines the piece's side and its character case (upper = white, lower = black).
- **Role** — Pawn through King. Determines which type of chessman it is.

Together they uniquely identify one of 12 possible pieces on a standard board: `K Q R B N P k q r b n p`. The class also provides Unicode glyph rendering for display purposes (♔ ♕ ♖ ♗ ♘ ♙ / ♚ ♛ ♜ ♝ ♞ ♟).

---

## Constructor
```php
new \shakmaty\Piece(int $color, int $role)
//   color: 0 = Black (\shakmaty\Color::BLACK), 1 = White (\shakmaty\Color::WHITE)
//   role : 1..6 (PAWN=1 … KING=6; see \shakmaty\Role constants)
```

| Parameter | Type    | Valid Values          | Description                              |
|-----------|---------|-----------------------|------------------------------------------|
| `$color`  | `int`   | `0`, `1`              | Side of the piece. Use `\shakmaty\Color::WHITE` or `\shakmaty\Color::BLACK`. |
| `$role`   | `int`   | `1..6`                | Piece type. Use a `\shakmaty\Role::*` constant for clarity.                  |

> **Tip:** Prefer constants over raw integers:
> ```php
> $white_king = new \shakmaty\Piece(\shakmaty\Color::WHITE,  \shakmaty\Role::KING);   // 'K' ♔
> $black_pawn = new \shakmaty\Piece(\shakmaty\Color::BLACK,  \shakmaty\Role::PAWN);   // 'p' ♟
> ```

---

## Properties

| Property | Type              | Access    | Description                          |
|----------|-------------------|-----------|--------------------------------------|
| `$color` | `\shakmaty\Color` | read-only | Side of the piece (White or Black).  |
| `$role`  | `\shakmaty\Role`  | read-only | Piece type (Pawn through King).      |

---

## Methods

| Method        | Return Type       | Description                                              |
|---------------|--------------------|----------------------------------------------------------|
| `toChar()`    | `string`           | Single-character representation: `'K'`, …, `'p'`. Uppercase = White, lowercase = Black. |

### Static Methods

| Method                              | Return Type     | Description                                              |
|-------------------------------------|-----------------|----------------------------------------------------------|
| `\shakmaty\Piece::fromChar(string $ch)` | `?\shakmaty\Piece` | Parses a single piece character (upper or lower case) into the corresponding Piece. Returns `null` if unrecognized.  |
| `\shakmaty\Piece::legacyCode(\shakmaty\Color $color, \shakmaty\Role $role)` | `int` | Converts a color+role pair to the legacy 12-code encoding (White `1..6`, Black `9..14`). |
| `\shakmaty\Piece::fromLegacyCode(int $code)` | `?\shakmaty\Piece` | Decodes a legacy 12-code into a `Piece`; returns `null` for `NOPIECE` (7) and any invalid code. |

---

## Character & Unicode Reference Table

The table below shows all 12 possible pieces with their standard FEN/SAN characters and Unicode glyphs:

| Color   | Role     | Char (`toChar()`) | Unicode Glyph |
|---------|----------|--------------------|---------------|
| White   | Pawn     | `P`                | ♙             |
| White   | Knight   | `N`                | ♘             |
| White   | Bishop   | `B`                | ♗             |
| White   | Rook     | `R`                | ♖             |
| White   | Queen    | `Q`                | ♕             |
| White   | King     | `K`                | ♔             |
| Black   | Pawn     | `p`                | ♟             |
| Black   | Knight   | `n`                | ♞             |
| Black   | Bishop   | `b`                | ♝             |
| Black   | Rook     | `r`                | ♜             |
| Black   | Queen    | `q`                | ♛             |
| Black   | King     | `k`                | ♚             |

> **Note:** The Unicode glyphs are for display only. Use `toChar()` when working with FEN, SAN, or UCI notation — those formats use the standard ASCII characters above.

---

## Legacy 12-code Encoding

The extension can convert a `Piece` to and from the legacy 12-code integer (as used by `common\chess\Piece`):

| Color | King | Queen | Rook | Bishop | Knight | Pawn |
|-------|------|-------|------|--------|--------|------|
| White | `1`  | `2`   | `3`  | `4`    | `5`    | `6`  |
| Black | `9`  | `10`  | `11` | `12`   | `13`   | `14` |

Encoding: `code = (color === BLACK ? 1 : 0) << 3 | (7 - role)`.
Decoding: `role = 7 - (code & 7)`, `color = (code & 8) ? BLACK : WHITE`. The value `7` means **NOPIECE** (empty square).

```php
$wk = new \shakmaty\Piece(\shakmaty\Color::WHITE, \shakmaty\Role::KING);
echo \shakmaty\Piece::legacyCode($wk->color, $wk->role); // 1

$bp = \shakmaty\Piece::fromLegacyCode(14);              // Black Pawn ('p')
$none = \shakmaty\Piece::fromLegacyCode(7);             // null (NOPIECE)
```

---

## Examples

```php
$wk = new \shakmaty\Piece(\shakmaty\Color::WHITE, \shakmaty\Role::KING);
echo $wk->toChar(); // 'K'
echo $wk->toUnicode(); // ♔

$bp = \shakmaty\Piece::fromChar('p');
echo $bp->color->isBlack(); // true
```
