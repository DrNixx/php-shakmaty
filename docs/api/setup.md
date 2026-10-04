# Setup Reference

This document covers `Setup` — a complete game-position object that combines piece placement, turn indicator, castling rights, en passant state, and move counters. Use this class as the primary container for any chess position you need to store, pass between functions, or serialize (e.g., FEN export). Method names follow the camelCase convention; state is read and written through PHP properties. `Setup` also carries variant-specific state — tracked promoted pieces (`$promoted`), Crazyhouse pockets (`$pockets`) and Three-Check remaining checks (`remainingChecks(color)`); see [Pockets & RemainingChecks](pockets.md) for the data types.

---

## Overview
A **Setup** is a full snapshot of an in-progress game:
- The current piece placement on the board ([`Board`](board.md)) — read-only property `$board`.
- Which side to move (`$turn` — White or Black) — writable property.
- Castling rights still available (`$castlingRights`, a [`Bitboard`](bitboard.md) with up to 4 bits set, one per rook corner square in standard chess: A1/H1/A8/H8) — read-only property.
- The en passant target square index if a double pawn push just occurred (`$epSquare`), or `null` otherwise — writable property.
- Halfmove clock (`$halfmoves`, consecutive halfmoves without a capture or pawn move — used for the 50-move rule) and fullmove number (`$fullmoves`, incremented after Black's turn, starts at 1) — writable properties.

The default constructor produces the **standard starting position** of chess: White to move, all four castling rights available, no en passant square, halfmoves = 0, fullmoves = 1. Use `\shakmaty\Setup::empty()` for an empty board with White-to-move and no other state (useful as a base when building custom positions). Use `\shakmaty\Setup::fromFen($fen)` to parse an existing FEN/EPD string into a Setup without any legality check.

> **Note:** `$setup->epSquare` accepts raw square index integers (`0..63`). Pass Square constants directly — e.g., `\shakmaty\Square::E3` (= 20) rather than the literal integer, for readability and maintainability.

> **Note:** The object-valued properties `$board`, `$castlingRights`, `$promoted` and `$pockets` are **read-only** — assigning to them throws. Mutate them through the corresponding `setBoard()`, `setCastlingRights()`, `setPromoted()` and `setPockets()` methods (and `clearPockets()`).

---

## Constructor & Static Factory Methods

| Method                          | Return Type     | Description                                              |
|---------------------------------|-----------------|----------------------------------------------------------|
| `new \shakmaty\Setup()`         | `\shakmaty\Setup`  | Creates a Setup in the **standard starting position**: White to move, all four castling rights available (A1/H1/A8/H8), no en passant square, halfmoves = 0, fullmoves = 1.   |
| `\shakmaty\Setup::empty(): static` | `static`    | Returns a Setup with an empty board and White to move; all other state fields are at their zero/null defaults (no castling rights, no ep square). Useful as a base for building custom positions or testing edge cases.   |
| `\shakmaty\Setup::fromFen(string $fen): static` | `static`    | Parses a FEN/EPD string into a Setup **without any legality check** — piece placement, turn, castling rights, en passant square, move counters and variant state (pockets / remaining checks) are taken as-is, so the result may describe an illegal position. The board field is required; missing trailing fields get defaults. Throws a subclass of `\shakmaty\fen\ParseFenError` on invalid input.   |

---

## Properties

Scalar state is exposed as **writable** properties; object-valued state as **read-only** properties (write it through the setter methods listed below).

| Property             | Type                        | Writable | Description                                              |
|----------------------|-----------------------------|----------|----------------------------------------------------------|
| `$turn`              | `int`                       | yes      | Side to move: **1 = White**, **0 = Black**. In a default Setup this is 1.   |
| `$halfmoves`         | `int`                       | yes      | **Halfmove clock** — halfmoves since the last capture or pawn move. Used for the 50-move rule (claimable draw at 100). In a default Setup this is 0.   |
| `$fullmoves`         | `int`                       | yes      | **Fullmove number** — starts at 1 and increments after Black's move (once per full round). In a default Setup this is 1.   |
| `$epSquare`         | `?int`                      | yes      | En passant target square index (`0..63`) if a double pawn push just occurred and an ep capture is available, or **null**. In a default Setup this is null. Pass a Square constant, e.g. `\shakmaty\Square::E3`.   |
| `$board`             | `\shakmaty\Board`           | no       | Current piece-placement board. In a default Setup this is the standard starting position (32 pieces). Write via `setBoard()`.   |
| `$castlingRights`   | `\shakmaty\Bitboard`        | no       | Castling-rights Bitboard. In standard chess up to four corner bits are set — A1 (White O-O-O), H1 (White O-O), A8 (Black o-o-o), H8 (Black o-o). In a default Setup `count() === 4`. Write via `setCastlingRights()`.   |
| `$promoted`          | `\shakmaty\Bitboard`        | no       | Bitboard of tracked promoted pieces (Crazyhouse); empty by default. Write via `setPromoted()`.   |
| `$pockets`           | `\shakmaty\Pockets \| null` | no       | Crazyhouse pockets, or **null** when not set. Write via `setPockets()`; clear via `clearPockets()`.   |

Remaining checks are **not** exposed as a property — use the methods below.

---

## Methods

### Board, Castling Rights & Variant Setters

These methods back the read-only properties above.

| Method                            | Return Type | Description                                              |
|-----------------------------------|-------------|----------------------------------------------------------|
| `setBoard(Board $board)`          | `void`      | Sets the board (backs the read-only `$board` property).   |
| `setCastlingRights(Bitboard $bb)` | `void`      | Sets castling rights (backs the read-only `$castlingRights` property).   |
| `setPromoted(Bitboard $bb)`       | `void`      | Sets the promoted-pieces bitboard (Crazyhouse).   |
| `setPockets(Pockets $pockets)`    | `void`      | Sets the Crazyhouse pockets.   |
| `clearPockets()`                  | `void`      | Clears the Crazyhouse pockets (back to null).   |

### Remaining checks (Three-Check)

| Method              | Return Type     | Description                                              |
|---------------------|-----------------|----------------------------------------------------------|
| `remainingChecks(int $color)` | `\shakmaty\RemainingChecks \| null`  | Checks still needed for `$color` (`1`=White, `0`=Black), or **null** when unset   |
| `setRemainingChecks(int $color, RemainingChecks $checks)` | `void`     | Sets remaining checks for a color (creates the default 3+3 pair if absent)    |
| `clearRemainingChecks()` | `void`      | Clears both colors' remaining-checks counters    |

> **Note:** In Crazyhouse the global material limit counts board **and** both pockets together, so a pocket pawn requires that matching pawn to be missing from the board. See [pockets.md](pockets.md).

---

## Examples

Writable scalar properties and read-only object properties:

```php
$setup = new \shakmaty\Setup();
echo $setup->turn; // 1 (White)
$setup->turn = 0;
echo $setup->turn; // 0 (Black)
$setup->epSquare = \shakmaty\Square::E3;
echo $setup->epSquare; // 20 (E3)

// Board in starting position — verify a few key squares via the read-only property
$board = $setup->board;
echo $board->pieceAt(\shakmaty\Square::E1);   // 'K' (White King on e1)
var_dump($board->occupied()->count());          // int(32): 16 white + 16 black

// Castling rights in starting position — all four corners available
$castling = $setup->castlingRights;
echo $castling->count();    // int(4)
var_dump($castling->has(\shakmaty\Square::A1));   // true: White queen-side right (O-O-O)
var_dump($castling->has(\shakmaty\Square::H8));   // true: Black king-side  right (o-o)

// No en passant square in starting position — no double push has occurred yet
$setup2 = new \shakmaty\Setup();
var_dump($setup2->epSquare);    // NULL: ep not available at game start

// Empty Setup — empty board, White to move; useful as a base for custom positions
$empty = \shakmaty\Setup::empty();
echo $empty->board->occupied()->count();   // int(0): nothing on the board
var_dump($empty->turn);                    // 1

// Parse a FEN into a Setup (no legality check, unlike Chess::fromFen())
$parsed = \shakmaty\Setup::fromFen('rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1');
echo $parsed->turn;                       // int(1): White to move
echo $parsed->castlingRights->count();    // int(4)
echo (string) \shakmaty\Fen::fromSetup($parsed); // round-trips to the same FEN

// Update move counters as game progresses
$setup2->halfmoves = 10;     // e.g., after 5 full rounds without a capture or pawn push
echo $setup2->halfmoves;      // int(10)

$setup2->fullmoves = 6;      // e.g., we are now in the middle of move #6 (White's turn on move 6)
echo $setup2->fullmoves;     // int(6)

// Object-valued properties are read-only — mutate via setters:
$setup2->setPockets(new \shakmaty\Pockets());
$setup2->setCastlingRights(new \shakmaty\Bitboard(0)); // no castling rights
```

Variant-specific state — build a Crazyhouse or Three-Check `Setup` directly, then read it back:

```php
use shakmaty\EnPassantMode;   // toSetup() takes an en-passant mode (not a castling mode)

// Build a Crazyhouse Setup with one pawn already in White's pocket.
$cz = new \shakmaty\Setup();
$pockets = new \shakmaty\Pockets();
$pockets->setCount(\shakmaty\Color::WHITE, \shakmaty\Role::PAWN, 1);   // Black is missing that pawn on the board
$cz->setPockets($pockets);

echo $cz->pockets->count(\shakmaty\Color::WHITE, \shakmaty\Role::PAWN); // 1
var_dump($cz->promoted);    // Bitboard (empty at start)

// Three-Check Setup — both sides need three checks to win.
$tc = new \shakmaty\Setup();
$tc->setRemainingChecks(\shakmaty\Color::WHITE,  new \shakmaty\RemainingChecks(3));
$tc->setRemainingChecks(\shakmaty\Color::BLACK, new \shakmaty\RemainingChecks(3));

echo $tc->remainingChecks(\shakmaty\Color::WHITE)->value(); // 3

// The same state round-trips through a variant position: toSetup() -> fromSetup().
$czPos = \shakmaty\VariantPosition::fromFen(
    \shakmaty\Variant::CRAZYHOUSE,
    "rnbqkbnr/ppp1pppp/8/8/8/8/PPPPPPPP/RNBQKBNR[P] w KQkq - 0 1", // Black missing d7 pawn → in White's pocket
    \shakmaty\CastlingMode::STANDARD
);
$czSetup = $czPos->toSetup(\shakmaty\EnPassantMode::LEGAL);   // note: en-passant mode, not castling mode
echo $czSetup->pockets->count(\shakmaty\Color::WHITE, \shakmaty\Role::PAWN); // 1

// Standard chess has none of this state.
var_dump((new \shakmaty\Chess())->pockets());          // NULL
```
