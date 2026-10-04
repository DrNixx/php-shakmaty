# Chess API Reference

Main chess position class — `shakmaty\Chess`. This is the central entry point for representing and manipulating a chess game state: piece placement, side to move, castling rights, en passant square, and move counters. A single instance mutates in place as you play moves via SAN or UCI notation. It extends the abstract [`Position`](position.md) base class, which declares the full position contract; Chess implements every method of that contract.

## Constructor & Factory Methods

### Standard Starting Position

```php
new \shakmaty\Chess()  // standard initial position (White to move)
```

Initializes the board with all pieces on their starting squares, White to move, castling rights `KQkq`, no en passant square, full-move counter = 1, half-move clock = 0.

### From FEN String (Static Factory)

```php
\shakmaty\Chess::fromFen(string $fen): \shakmaty\Chess
```

Parses a syntactically valid FEN string into a new `Chess` instance reflecting the described position. **Throws** an exception if the FEN is malformed (invalid piece placement, bad side-to-move token, invalid castling rights, etc.). Always wrap in try/catch when parsing user-supplied input.

## Position Information Methods

Read-only accessors that expose current state without mutating anything:

| Method | Return Type | Description |
|--------|-------------|-------------|
| `turn()` | `int` | Side to move: 1 = White, 0 = Black |
| `fullmoves()` | `int` | Full-move counter (starts at 1; increments after each pair of half-moves) |
| `halfmoves()` | `int` | Half-move clock: plies since last capture or pawn advance (fifty-move rule) |
| `board()` | `\shakmaty\Board` | Current piece placement on all 64 squares |
| `outcome()` | `string` | Game result: `"1-0"`, `"0-1"`, `"1/2-1/2"` (draw), or `"*"` (in progress) |
| `epSquare(?int $mode = null)` | `?int` | En passant target square index, or null if none available (`0` Legal [default], `1` PseudoLegal, `2` Always) |
| `castlingRights()` | `\shakmaty\Bitboard` | Castling rights still available as a bitboard mask |

## State Check Methods

Boolean predicates for common game-state questions:

| Method | Return Type | Description |
|--------|-------------|-------------|
| `isCheck()` | `bool` | True if the side not to move is currently in check |
| `isCheckmate()` | `bool` | True if the position is a legal checkmate (no escape for mated king) |
| `isStalemate()` | `bool` | True if the side to move has no legal moves and its king is not in check |
| `isInsufficientMaterial()` | `bool` | True if neither side can possibly deliver mate (e.g. K vs K, KB/KR vs K with no pawns) |

> These terminal states are mutually exclusive: at most one of `checkmate`, `stalemate` applies per position. `insufficient material` is an independent draw condition that may coexist with any other state — it does not imply check or mate by itself, but combined with zero legal moves produces a drawn outcome via `outcome()`.

## Move Execution Methods

Both methods mutate the position in place and return on success; they **throw** rather than returning an error code when given invalid input.

### Play SAN (Standard Algebraic Notation)

```php
\shakmaty\Chess::playSan(string $san): bool
```

Plays a legal move expressed in Standard Algebraic notation, e.g. `"e4"`, `"Nf3"`, `"Qxf7#"`. Disambiguation prefixes (file/rank) are accepted when required by the position context. Returns `true` on success; throws if `$san` is not a legal move or cannot be parsed as SAN at all.

### Play UCI (Universal Chess Interface Notation)

```php
\shakmaty\Chess::playUci(string $uci): bool
```

Plays a legal move expressed in UCI long-algebraic notation, e.g. `"e2e4"`, `"g1f3"`. Castling is encoded as the king's two-square jump (e.g. `"e1g1"`). Promotion suffixes (`q`,`r`,`b`,`n`) are appended to the target square when applicable (e.g. `"e7e8q"`). Returns `true` on success; throws if `$uci` is not a legal move or cannot be parsed as UCI at all.

> **Tip:** Use SAN for human-facing input and UCI when interfacing with engines that emit long-algebraic strings natively (Stockfish, Leela Chess Zero). Both methods are equivalent — they differ only in notation parsing; after execution the resulting position is identical regardless of which method you used to play a given move.

## Move Enumeration Methods

| Method | Return Type | Description |
|--------|-------------|-------------|
| `legalMovesCount()` | `int` | Number of legal moves available (0 if checkmated or stalemated) |
| `legalMoves()` | `\shakmaty\MoveList` | Ordered list of all currently-legal moves — see [move.md](move.md) for full API details |

> **Performance note:** Both methods generate the move set on demand and do not cache results across calls. If you need to iterate multiple times over the same list without replaying generation logic, store the returned `MoveList` locally (e.g. after a mutation via `playSan`, prior cached lists are stale).

## Methods Inherited from Position

`Chess` also exposes the full [`Position`](position.md) contract:

| Method | Return Type | Description |
|--------|-------------|-------------|
| `us()` | `\shakmaty\Bitboard` | Squares occupied by the side to move |
| `our(int $role)` | `\shakmaty\Bitboard` | Squares occupied by a role of the side to move |
| `them()` | `\shakmaty\Bitboard` | Squares occupied by the opponent |
| `their(int $role)` | `\shakmaty\Bitboard` | Squares occupied by a role of the opponent |
| `promoted()` | `\shakmaty\Bitboard` | Tracked promoted pieces (always empty for Chess) |
| `maybeEpSquare()` | `?int` | En passant square after a double pawn push, unconditionally |
| `checkers()` | `\shakmaty\Bitboard` | Pieces currently giving check |
| `isGameOver()` | `bool` | Checkmate, stalemate, insufficient material, or variant end |
| `isVariantEnd()` | `bool` | Variant-specific end (always `false` for Chess) |
| `hasInsufficientMaterial(int $color)` | `bool` | Whether a side has insufficient material |
| `variantOutcome()` | `string` | Variant outcome (always `"*"` for Chess) |
| `pseudoLegalEpSquare()` | `?int` | En passant square for a pseudo-legal capture |
| `legalEpSquare()` | `?int` | En passant square for a legal capture |
| `captureMoves()` | `\shakmaty\MoveList` | Capture moves |
| `promotionMoves()` | `\shakmaty\MoveList` | Promotion moves |
| `enPassantMoves()` | `\shakmaty\MoveList` | En passant moves |
| `castlingMoves(int $side)` | `\shakmaty\MoveList` | Castling moves (`0` king-side, `1` queen-side) |
| `sanCandidates(int $role, int $to)` | `\shakmaty\MoveList` | SAN candidate moves |
| `kingAttackers(int $square, int $attacker, Bitboard $occupied)` | `\shakmaty\Bitboard` | Attackers of a king square |
| `isIrreversible(Move $m)` | `bool` | Whether a move is irreversible |
| `isLegal(Move $m)` | `bool` | Whether a move is legal |
| `play(Move $m)` | `\shakmaty\Chess` | Plays a legal move, returning a new position |
| `playUnchecked(Move $m)` | `void` | Plays a move without legality checks (mutates) |
| `toSetup(int $mode)` | `\shakmaty\Setup` | Converts the position to a Setup |

## Serialization Methods

### To FEN String

```php
\shakmaty\Chess::toFen(): string
```

Returns a complete syntactically-valid FEN representation of the current position: piece placement + side-to-move + castling rights + en passant square + half/full move counters. The output can be round-tripped back through `fromFen()` without loss. See [fen.md](fen.md) for the standalone validation wrapper class.

## Complete Example — Scholar's Mate (White wins in 4 moves)

Full game line from start to checkmate, including FEN serialization and deserialization:

```php
$pos = new \shakmaty\Chess();
$pos->playSan('e4');
$pos->playSan('e5');
$pos->playSan('Qh5');
$pos->playSan('Nc6');
$pos->playSan('Bc4');
$pos->playSan('Nf6');
$pos->playSan('Qxf7');
var_dump($pos->isCheckmate()); // true
echo $pos->outcome(); // "1-0"

// FEN round-trip: parse the serialized position back into a fresh instance and confirm state matches
$pos2 = \shakmaty\Chess::fromFen($pos->toFen());
var_dump($pos2->isCheckmate()); // true — same terminal condition recovered from serialized form

// Enumerate legal moves in the mated position (zero for Black, who is checkmated and has no escape)
echo $pos2->legalMovesCount(); // 0
```

## See Also

| Topic | Reference |
|-------|-----------|
| Move & MoveList API | [move.md](move.md) — full method signatures and iteration patterns |
| FEN wrapper class | [fen.md](fen.md) — standalone `Fen` value object for validation without a Chess instance |
| SAN / UCI notation wrappers | [san.md](san.md), [uci.md](uci.md) — lightweight string validators usable outside of playSan/playUci contexts |
| Abstract position base class | [position.md](position.md) — the Position contract implemented by Chess |
