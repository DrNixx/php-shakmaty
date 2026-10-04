# Enum-Type Classes Reference

This document covers five enum-type value classes provided by `shakmaty`: **Color**, **Role**, **CastlingSide**, **CastlingMode**, and **EnPassantMode**. These are lightweight immutable wrappers around small integer values, designed for type-safe chess domain modeling. All method names follow camelCase convention (standard PHP style).

---

## Color

### Overview
`Color` represents the two sides in a game of chess: White or Black. It wraps an `int` value where `0 = BLACK`, `1 = WHITE`. Use this class instead of raw integers wherever you need to pass, store, or compare player colors — it makes intent explicit and prevents accidental misuse (e.g., passing role values as color).

### Constructor
```php
new \shakmaty\Color(int $value)  // default: Color::BLACK (0); valid range: 0–1
```

| Value | Meaning   | Constant          |
|-------|-----------|-------------------|
| `0`   | Black     | `\shakmaty\Color::BLACK` |
| `1`   | White     | `\shakmaty\Color::WHITE`  |

### Methods

| Method        | Return Type       | Description                                      |
|---------------|-------------------|--------------------------------------------------|
| `value()`     | `int`             | Returns the underlying integer value (`0` or `1`). |
| `isWhite()`   | `bool`            | `true` if this color is White.                   |
| `isBlack()`   | `bool`            | `true` if this color is Black.                   |
| `other()`     | `\shakmaty\Color` | Returns the opposite color (White ↔ Black).      |
| `toChar()`    | `string`          | Single-character representation: `'w'` or `'b'`.  |

### Constants

| Constant            | Value | Description        |
|---------------------|-------|--------------------|
| `\shakmaty\Color::WHITE`  | `1`   | White side.        |
| `\shakmaty\Color::BLACK`  | `0`   | Black side (default). |

### Examples

```php
// Create a Color instance — default is BLACK
$black = new \shakmaty\Color();          // value: 0, isBlack() === true
var_dump($black->isWhite());             // bool(false)

// Use constants for clarity in conditional logic
$c = new \shakmaty\Color(\shakmaty\Color::WHITE);
echo $c->toChar();                       // 'w'

// Get the opposite color — useful in move generation / turn logic
$opposite = (new \shakmaty\Color(1))->other();   // Color(BLACK), value: 0
var_dump($opposite->isBlack());           // bool(true)
```

---

## Role

### Overview
`Role` represents a chess piece type (pawn, knight, bishop, rook, queen, king). It wraps an `int` from `1..6`. Use this class to identify which kind of piece is on the board or in a move — it decouples role logic from color and square.

### Constructor
```php
new \shakmaty\Role(int $value)  // value must be 1–6 (PAWN=1 … KING=6); no default
```

| Value | Meaning   | Constant              |
|-------|-----------|------------------------|
| `1`   | Pawn      | `\shakmaty\Role::PAWN`    |
| `2`   | Knight    | `\shakmaty\Role::KNIGHT`  |
| `3`   | Bishop    | `\shakmaty\Role::BISHOP`  |
| `4`   | Rook      | `\shakmaty\Role::ROOK`    |
| `5`   | Queen     | `\shakmaty\Role::QUEEN`   |
| `6`   | King      | `\shakmaty\Role::KING`    |

### Methods

| Method        | Return Type       | Description                                              |
|---------------|--------------------|----------------------------------------------------------|
| `value()`     | `int`             | Returns the underlying integer value (`1..6`).          |
| `toChar()`    | `string`          | Lowercase piece character: `'p'`, `'n'`, …, `'k'`.      |
| `upperChar()` | `string`          | Uppercase piece character: `'P'`, `'N'`, …, `'K'`.      |

### Static Methods

| Method                          | Return Type     | Description                                              |
|---------------------------------|-----------------|----------------------------------------------------------|
| `\shakmaty\Role::fromChar(string $ch)` | `?\shakmaty\Role` | Parses a piece character (upper or lower case) into the corresponding Role. Returns `null` if unrecognized. |

### Constants

| Constant              | Value | Description   |
|-----------------------|-------|---------------|
| `\shakmaty\Role::PAWN`    | `1`   | Pawn          |
| `\shakmaty\Role::KNIGHT`  | `2`   | Knight        |
| `\shakmaty\Role::BISHOP`  | `3`   | Bishop        |
| `\shakmaty\Role::ROOK`    | `4`   | Rook          |
| `\shakmaty\Role::QUEEN`   | `5`   | Queen         |
| `\shakmaty\Role::KING`    | `6`   | King          |

### Examples

```php
// Create a Role instance using constants — preferred over raw integers
$queen = new \shakmaty\Role(\shakmaty\Role::QUEEN);  // value: 5
echo $queen->toChar();       // 'q' (lowercase)
echo $queen->upperChar();    // 'Q'

// Parse a character from FEN or user input — returns null on failure
$role = \shakmaty\Role::fromChar('N');   // Role(KNIGHT), value: 2
var_dump($role);                       // object(\shakmaty\Role) @… (value=2)

// Invalid character → null, no exception thrown
echo var_export(\shakmaty\Role::fromChar('?'), true);  // NULL
```

---

## CastlingSide

### Overview
`CastlingSide` identifies which side of the board a castling right belongs to: King-side (kingside / short) or Queen-side (queenside / long). It wraps an `int` where `0 = KING_SIDE`, `1 = QUEEN_SIDE`. Use this class when tracking, setting, or clearing individual castling rights.

### Constructor
```php
new \shakmaty\CastlingSide(int $value)  // default: CastlingSide::KING_SIDE (0); valid range: 0–1
```

| Value | Meaning     | Constant                    |
|-------|-------------|-----------------------------|
| `0`   | King-side   | `\shakmaty\CastlingSide::KING_SIDE`   |
| `1`   | Queen-side  | `\shakmaty\CastlingSide::QUEEN_SIDE`  |

### Methods

| Method            | Return Type          | Description                                       |
|-------------------|----------------------|----------------------------------------------------|
| `value()`         | `int`                | Returns the underlying integer value (`0` or `1`). |
| `isKingSide()`    | `bool`               | `true` if this is King-side (short) castling.      |
| `isQueenSide()`   | `bool`               | `true` if this is Queen-side (long) castling.      |
| `other()`         | `\shakmaty\CastlingSide` | Returns the opposite side (King ↔ Queen).     |

### Constants

| Constant                    | Value | Description       |
|-----------------------------|-------|--------------------|
| `\shakmaty\CastlingSide::KING_SIDE`   | `0`   | King-side / short castling.  |
| `\shakmaty\CastlingSide::QUEEN_SIDE`  | `1`   | Queen-side / long castling.  |

### Examples

```php
// Create a CastlingSide instance — default is KING_SIDE
$ks = new \shakmaty\CastlingSide();          // value: 0, isKingSide() === true
var_dump($ks->isQueenSide());                // bool(false)

// Use constants for clarity in conditional logic
$qside = new \shakmaty\CastlingSide(\shakmaty\CastlingSide::QUEEN_SIDE);
if ($qside->other()->value() === 0) {        // other side is King-side → true
    echo "Opposite of Queen-side is King-side\n";   // printed
}

// Iterate over both sides (useful when clearing rights after a king move)
foreach ([\shakmaty\CastlingSide::KING_SIDE, \shakmaty\CastlingSide::QUEEN_SIDE] as $sideVal) {
    // …clear bit for side in castling-rights Bitboard…
}
```

---

## CastlingMode

### Overview
`CastlingMode` selects the rule set used to validate and execute a castle move. It wraps an `int`: `0 = STANDARD` (standard FIDE rules) or `1 = CHESS960` (Fischer Random / Chess960, where rooks may start on non-corner files). Use this class when configuring the engine's castling behavior — it is typically passed to move-generation and legality-checking routines.

### Constructor
```php
new \shakmaty\CastlingMode(int $value)  // default: CastlingMode::STANDARD (0); valid range: 0–1
```

| Value | Meaning    | Constant                  |
|-------|------------|---------------------------|
| `0`   | Standard   | `\shakmaty\CastlingMode::STANDARD` |
| `1`   | Chess960   | `\shakmaty\CastlingMode::CHESS960`  |

### Methods

| Method          | Return Type | Description                                          |
|-----------------|-------------|------------------------------------------------------|
| `value()`       | `int`       | Returns the underlying integer value (`0` or `1`).   |
| `isStandard()`  | `bool`      | `true` if using standard FIDE castling rules.        |
| `isChess960()`  | `bool`      | `true` if using Chess960 (Fischer Random) rules.     |

### Constants

| Constant                  | Value | Description                              |
|---------------------------|-------|------------------------------------------|
| `\shakmaty\CastlingMode::STANDARD` | `0`   | Standard FIDE castling (default).        |
| `\shakmaty\CastlingMode::CHESS960`  | `1`   | Chess960 / Fischer Random castling.      |

### Examples

```php
// Default mode is STANDARD — suitable for classical chess games
$mode = new \shakmaty\CastlingMode();          // value: 0, isStandard() === true
var_dump($mode->isChess960());                 // bool(false)

// Chess960 mode enables rook-on-any-file castling logic
$c960 = new \shakmaty\CastlingMode(\shakmaty\CastlingMode::CHESS960);
if ($c960->isChess960()) {
    echo "Using Fischer Random rules\n";       // printed
}

// Branch on mode when generating castling moves — avoids duplicating logic
$mode = new \shakmaty\CastlingMode(\shakmaty\CastlingMode::STANDARD);
if ($mode->isStandard()) { /* standard path */ } else { /* c960 path */ }
```

---

## EnPassantMode

### Overview
`EnPassantMode` controls how the engine treats en passant capture opportunities. It wraps an `int`: `0 = LEGAL`, `1 = PSEUDO_LEGAL`, or `2 = ALWAYS`. Use this class when configuring move generation — it determines whether a pseudo-legal ep square is validated against full legality rules before being offered as a legal move, and how the engine handles edge cases (e.g., stalemate-preventing captures).

### Constructor
```php
new \shakmaty\EnPassantMode(int $value)  // default: EnPassantMode::LEGAL (0); valid range: 0–2
```

| Value | Meaning       | Constant                      |
|-------|---------------|-------------------------------|
| `0`   | Legal         | `\shakmaty\EnPassantMode::LEGAL`        |
| `1`   | Pseudo-legal  | `\shakmaty\EnPassantMode::PSEUDO_LEGAL` |
| `2`   | Always        | `\shakmaty\EnPassantMode::ALWAYS`       |

### Methods

| Method    | Return Type | Description                                    |
|-----------|-------------|-------------------------------------------------|
| `value()` | `int`       | Returns the underlying integer value (`0`, `1`, or `2`). |

### Constants

| Constant                      | Value | Description                                          |
|-------------------------------|-------|------------------------------------------------------|
| `\shakmaty\EnPassantMode::LEGAL`        | `0`   | En passant is only legal if the full move passes all legality checks (default). |
| `\shakmaty\EnPassantMode::PSEUDO_LEGAL` | `1`   | The ep square is recorded as pseudo-legal; final validation happens at search time.  |
| `\shakmaty\EnPassantMode::ALWAYS`       | `2`   | En passant capture is always permitted when the geometric condition holds, regardless of other rules (useful for debugging / analysis). |

### Examples

```php
// Default mode — full legality check applied to en passant moves
$mode = new \shakmaty\EnPassantMode();          // value: 0, LEGAL
var_dump($mode->value());                       // int(0)

// Pseudo-legal mode is common in fast move generators (e.g., for UCI output)
$pseudo = new \shakmaty\EnPassantMode(\shakmaty\EnPassantMode::PSEUDO_LEGAL);
if ($pseudo->value() === 1) {
    echo "Ep square recorded without full legality check\n";   // printed
}

// ALWAYS mode — useful for debugging: ep is always available when geometry allows it
$always = new \shakmaty\EnPassantMode(\shakmaty\EnPassantMode::ALWAYS);
var_dump($always->value());                     // int(2)
```
