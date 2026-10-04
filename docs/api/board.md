# Board Reference

This document covers `Board` — a snapshot of piece placement on the chessboard. A `Board` stores which pieces occupy each square but does **not** track game state (turn, castling rights, en passant). For full position tracking use [`Setup`](setup.md) instead. All method names follow camelCase convention.

---

## Overview
A **Board** is a read-only view of the 64-square chessboard at any given moment:
- Which piece character (`K`, `Q`, …, `p`) sits on each square (or nothing).
- Bitboards for fast set queries — all occupied squares, white pieces only, black pawns only, etc.

The default constructor produces the **standard starting position** of a chess game. Use `\shakmaty\Board::empty()` to get an empty board with no pieces at all (useful as a base when building custom positions).

> **Note:** `pieceAt()`, `roleAt()`, and `colorAt()` accept raw square index integers (`0..63`). Pass Square constants directly — e.g. `\shakmaty\Square::E1` (= 4) rather than the literal integer, for readability.

---

## Constructor & Static Factory Methods

| Method                          | Return Type     | Description                                              |
|---------------------------------|-----------------|----------------------------------------------------------|
| `new \shakmaty\Board()`         | `\shakmaty\Board`  | Creates a Board in the **standard starting position** (32 pieces: white on ranks 1–2, black on ranks 7–8). |
| `\shakmaty\Board::empty(): static` | `static`   | Returns an empty board with no pieces placed. All squares are unoccupied; useful as a base for building custom positions or testing edge cases. |

---

## Methods — Piece Lookup by Square

These methods query the piece on a specific square and return primitive values (integers / strings) rather than object references, making them fast to call in tight loops:

| Method              | Return Type     | Description                                              |
|---------------------|-----------------|----------------------------------------------------------|
| `pieceAt(int $sq)`  | `?string`       | Returns the piece character at square `$sq`: `'K'`, …, `'p'`. Uppercase = White, lowercase = Black. Returns **null** if the square is empty. Use Square constants: e.g. `\shakmaty\Square::E1`.  |
| `roleAt(int $sq)`   | `?int`          | Returns the role value at square `$sq`: `1`=Pawn … `6`=King (see Role constants). Returns **null** if empty. Use Square constants: e.g. `\shakmaty\Square::E8`.  |
| `colorAt(int $sq)`  | `?int`          | Returns the color value at square `$sq`: `0`=Black, `1`=White (see Color constants). Returns **null** if empty. Use Square constants: e.g. `\shakmaty\Square::E8`.   |

---

## Methods — Bitboard Queries

These methods return [`Bitboard`](bitboard.md) objects for fast set-based queries over the entire board at once, rather than iterating square by square:

| Method                    | Return Type     | Description                                              |
|---------------------------|-----------------|----------------------------------------------------------|
| `occupied()`              | `\shakmaty\Bitboard`  | Bitboard of all squares that have a piece on them (both colors). In the starting position, `count() === 32`.   |
| `byColor(int $colorValue)` | `\shakmaty\Bitboard` | Bitboard containing only pieces belonging to color `$colorValue`: pass `0` for Black or `1` for White. Use Color constants: e.g. `\shakmaty\Color::WHITE`. In the starting position, each side has 16 pieces (`count() === 16`).   |
| `byRole(int $roleValue)` | `\shakmaty\Bitboard`  | Bitboard containing only squares occupied by a piece of role `$roleValue`: pass Role constants — e.g. `\shakmaty\Role::PAWN`, `\shakmaty\Role::KING`. In the starting position: pawns → `count() === 16`; knights → `4`; kings → `2`.   |

---

## Examples

```php
$board = new \shakmaty\Board();
echo $board->pieceAt(\shakmaty\Square::E1); // 'K'
echo $board->byColor(1)->count();           // 16 white pieces
echo $board->byRole(\shakmaty\Role::PAWN)->count(); // 16 pawns

// Empty squares return null — no exception thrown
var_dump($board->pieceAt(\shakmaty\Square::E4));   // NULL (e4 is empty in starting position)

// Bitboard queries for fast set operations
$occupied = $board->occupied();
echo $occupied->count();    // int(32): 16 white + 16 black pieces

// All White pieces — useful when generating moves or checking attacks from one side only
$white_pieces = $board->byColor(\shakmaty\Color::WHITE);   // byColor(int) accepts the constant value directly (=== 1)
echo $white_pieces->count();    // int(16): all white pieces

// All pawns on both sides — useful for en passant or promotion logic
$pawns = $board->byRole(\shakmaty\Role::PAWN);   // byRole(int) accepts the constant value directly (=== 1)
echo $pawns->count();    // int(16): 8 white pawns + 8 black pawns

// Role and color at a specific square — useful for move legality checks
$role_e1 = $board->roleAt(\shakmaty\Square::E1);   // \shakmaty\Role::KING (=== 6)
var_dump($role_e1 === \shakmaty\Role::KING);         // true

// Empty board — no pieces at all; useful for testing or building custom positions from scratch
$empty = \shakmaty\Board::empty();
echo $empty->occupied()->count();   // int(0): nothing on the board
var_dump($empty->pieceAt(\shakmaty\Square::E4));    // NULL (all squares empty)
```
