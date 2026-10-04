# File, Rank & Square Reference

This document covers three coordinate classes in `shakmaty`: **File** (columns A–H), **Rank** (rows 1–8), and **Square** (individual board positions). All method names follow camelCase convention.

---

## File

### Overview
`File` represents a chess file — one of the eight vertical columns on the board, labeled `A` through `H`. It wraps an integer from `0..7`, where `0 = A` and `7 = H`. Use this class to work with column coordinates in move generation, attack detection, or FEN parsing.

### Constructor
```php
new \shakmaty\File(int $value)  // value must be 0–7 (A=0 … H=7); default: File::A (0)
```

| Value | Meaning   | Constant        |
|-------|-----------|-----------------|
| `0`   | A         | `\shakmaty\File::A` |
| `1`   | B         | `\shakmaty\File::B` |
| `2`   | C         | `\shakmaty\File::C` |
| `3`   | D         | `\shakmaty\File::D` |
| `4`   | E         | `\shakmaty\File::E` |
| `5`   | F         | `\shakmaty\File::F` |
| `6`   | G         | `\shakmaty\File::G` |
| `7`   | H         | `\shakmaty\File::H` |

### Methods

| Method              | Return Type     | Description                                              |
|---------------------|-----------------|----------------------------------------------------------|
| `value()`           | `int`           | Returns the underlying integer value (`0..7`).          |
| `toChar()`          | `string`        | Lowercase file character: `'a'`, …, `'h'`.              |
| `upperChar()`       | `string`        | Uppercase file character: `'A'`, …, `'H'`.              |
| `distance(File $other)` | `int`      | Absolute horizontal distance in files between two Files.  |
| `flipHorizontal(): File` | `\shakmaty\File` | Returns the mirrored file (a↔h, b↔g, c↔f, d↔e).     |

### Static Methods

| Method                          | Return Type    | Description                                              |
|---------------------------------|----------------|----------------------------------------------------------|
| `\shakmaty\File::fromChar(string $ch)` | `?\shakmaty\File` | Parses a file character (upper or lower case) into the corresponding File. Returns `null` if unrecognized. |

### Constants

All eight files are available as class constants:

```php
\shakmaty\File::A  // = 0
\shakmaty\File::B  // = 1
\shakmaty\File::C  // = 2
\shakmaty\File::D  // = 3
\shakmaty\File::E  // = 4
\shakmaty\File::F  // = 5
\shakmaty\File::G  // = 6
\shakmaty\File::H  // = 7
```

### Examples

```php
// Create a File instance using constants — preferred over raw integers
$e_file = new \shakmaty\File(\shakmaty\File::E);   // value: 4, toChar(): 'e'
echo $e_file->toChar();       // 'e' (lowercase)
echo $e_file->upperChar();    // 'E'

// Parse a character from FEN or user input — returns null on failure
$file = \shakmaty\File::fromChar('d');   // File(D), value: 3
var_dump($file);                       // object(\shakmaty\File) @… (value=3)

// Distance between two files
$dist_a_h = (new \shakmaty\File(0))->distance(new \shakmaty\File(7));   // int(7)
```

---

## Rank

### Overview
`Rank` represents a chess rank — one of the eight horizontal rows on the board, labeled `1` through `8`. It wraps an integer from `0..7`, where `0 = FIRST (rank 1)` and `7 = EIGHTH (rank 8)`. Use this class to work with row coordinates in move generation or FEN parsing.

### Constructor
```php
new \shakmaty\Rank(int $value)  // value must be 0–7; default: Rank::FIRST (0)
```

| Value | Meaning   | Constant            |
|-------|-----------|---------------------|
| `0`   | First     | `\shakmaty\Rank::FIRST`    |
| `1`   | Second    | `\shakmaty\Rank::SECOND`   |
| `2`   | Third     | `\shakmaty\Rank::THIRD`    |
| `3`   | Fourth    | `\shakmaty\Rank::FOURTH`   |
| `4`   | Fifth     | `\shakmaty\Rank::FIFTH`    |
| `5`   | Sixth     | `\shakmaty\Rank::SIXTH`    |
| `6`   | Seventh   | `\shakmaty\Rank::SEVENTH`  |
| `7`   | Eighth    | `\shakmaty\Rank::EIGHTH`   |

### Methods

| Method              | Return Type     | Description                                              |
|---------------------|-----------------|----------------------------------------------------------|
| `value()`           | `int`           | Returns the underlying integer value (`0..7`).          |
| `toChar()`          | `string`        | Rank digit character: `'1'`, …, `'8'`.                   |
| `distance(Rank $other)` | `int`       | Absolute vertical distance in ranks between two Ranks.   |
| `flipVertical(): Rank`  | `\shakmaty\Rank` | Returns the mirrored rank (first↔eighth, second↔seventh, …). |

### Static Methods

| Method                          | Return Type    | Description                                              |
|---------------------------------|----------------|----------------------------------------------------------|
| `\shakmaty\Rank::fromChar(string $ch)` | `?\shakmaty\Rank` | Parses a rank digit character (`'1'..'8'`) into the corresponding Rank. Returns `null` if unrecognized. |

### Constants

All eight ranks are available as class constants:

```php
\shakmaty\Rank::FIRST    // = 0 (rank "1")
\shakmaty\Rank::SECOND   // = 1 (rank "2")
\shakmaty\Rank::THIRD    // = 2 (rank "3")
\shakmaty\Rank::FOURTH   // = 3 (rank "4")
\shakmaty\Rank::FIFTH    // = 4 (rank "5")
\shakmaty\Rank::SIXTH    // = 5 (rank "6")
\shakmaty\Rank::SEVENTH  // = 6 (rank "7")
\shakmaty\Rank::EIGHTH   // = 7 (rank "8")
```

### Examples

```php
// Create a Rank instance using constants — preferred over raw integers
$fourth = new \shakmaty\Rank(\shakmaty\Rank::FOURTH);   // value: 3, toChar(): '4'
echo $fourth->toChar();       // '4'

// Parse a digit from FEN or user input — returns null on failure
$rank = \shakmaty\Rank::fromChar('5');   // Rank(FIFTH), value: 4
var_dump($rank);                       // object(\shakmaty\Rank) @… (value=4)

// Distance between two ranks
$dist_1_8 = (new \shakmaty\Rank(0))->distance(new \shakmaty\Rank(7));   // int(7)

// Flip vertical — rank 8 becomes rank 1, etc.
$r8_flipped = new \shakmaty\Rank(\shakmaty\Rank::EIGHTH);
var_dump($r8_flipped->flipVertical()->value());   // int(0), i.e., FIRST (rank "1")
```

---

## Square

### Overview
`Square` represents a single position on the chess board — one of 64 squares identified by file + rank. The constant `Square::NS` (value 64) represents a null/undefined square. It wraps an integer from `0..63`, where bit layout is: **bits 7–5 = file** (A=0…H=7), **bits 2–0 = rank** (1st=0…8th=7). In other words, square index = `(rank << 3) | file`. Use this class as the primary coordinate type throughout `shakmaty` — it is used by `Bitboard`, `Board`, and move objects.

### Constructor
```php
new \shakmaty\Square(int $value)  // value must be 0–63 (A1=0 … H8=63); default: Square::A1 (0)
```

| Value | Meaning   | Constant          |
|-------|-----------|-------------------|
| `0`   | A1        | `\shakmaty\Square::A1`  |
| `28`  | E4        | `\shakmaty\Square::E4`  |
| `63`  | H8        | `\shakmaty\Square::H8`  |

> **Note:** All 64 squares are available as class constants (see Constants section below).

### Methods

| Method              | Return Type     | Description                                              |
|---------------------|-----------------|----------------------------------------------------------|
| `value()`           | `int`           | Returns the underlying integer value (`0..63`).          |
| `file(): File`      | `\shakmaty\File`  | The file (column) of this square.                        |
| `rank(): Rank`      | `\shakmaty\Rank`  | The rank (row) of this square.                           |
| `__toString()`        | `string`        | Algebraic notation: `'a1'`, …, `'h8'`. PHP magic method, also available via the `(string)` cast. |
| `isLight()`         | `bool`          | `true` if the square is a light-colored square.           |
| `isDark()`          | `bool`          | `true` if the square is a dark-colored square.            |
| `distance(Square $other)` | `int`     | Chebyshev distance (max of file/rank deltas) to another Square — 0 for same square, up to 7. |
| `flipHorizontal(): Square` | `\shakmaty\Square` | Mirrors the square horizontally: a↔h, b↔g, c↔f, d↔e (rank unchanged). |
| `flipVertical(): Square`   | `\shakmaty\Square` | Mirrors the square vertically: rank 1↔8, 2↔7, … (file unchanged). |

### Static Methods

| Method                                    | Return Type     | Description                                              |
|-------------------------------------------|-----------------|----------------------------------------------------------|
| `\shakmaty\Square::fromAscii(string $s)` | `?\shakmaty\Square` | Parses an algebraic square string (`'a1'`, …, `'h8'`) into a Square. Returns `null` if unrecognized (case-insensitive). |
| `\shakmaty\Square::fromCoords(File $file, Rank $rank)` | `\shakmaty\Square` | Constructs a Square from its File and Rank components.  |

### Constants — All 64 Squares Grouped by Rank

**Rank 1 (value = file):**
| Constant   | Value | Constant   | Value | Constant   | Value | Constant   | Value |
|------------|-------|------------|-------|------------|-------|------------|-------|
| `A1`       | `0`   | `B1`       | `1`   | `C1`       | `2`   | `D1`       | `3`   |
| `E1`       | `4`   | `F1`       | `5`   | `G1`       | `6`   | `H1`       | `7`   |

**Rank 2 (value = file + 8):**
| Constant   | Value | Constant   | Value | Constant   | Value | Constant   | Value |
|------------|-------|------------|-------|------------|-------|------------|-------|
| `A2`       | `8`   | `B2`       | `9`   | `C2`       | `10`  | `D2`       | `11`  |
| `E2`       | `12`  | `F2`       | `13`  | `G2`       | `14`  | `H2`       | `15`  |

**Rank 3 (value = file + 16):**
| Constant   | Value | Constant   | Value | Constant   | Value | Constant   | Value |
|------------|-------|------------|-------|------------|-------|------------|-------|
| `A3`       | `16`  | `B3`       | `17`  | `C3`       | `18`  | `D3`       | `19`  |
| `E3`       | `20`  | `F3`       | `21`  | `G3`       | `22`  | `H3`       | `23`  |

**Rank 4 (value = file + 24):**
| Constant   | Value | Constant   | Value | Constant   | Value | Constant   | Value |
|------------|-------|------------|-------|------------|-------|------------|-------|
| `A4`       | `24`  | `B4`       | `25`  | `C4`       | `26`  | `D4`       | `27`  |
| `E4`       | `28`  | `F4`       | `29`  | `G4`       | `30`  | `H4`       | `31`  |

**Rank 5 (value = file + 32):**
| Constant   | Value | Constant   | Value | Constant   | Value | Constant   | Value |
|------------|-------|------------|-------|------------|-------|------------|-------|
| `A5`       | `32`  | `B5`       | `33`  | `C5`       | `34`  | `D5`       | `35`  |
| `E5`       | `36`  | `F5`       | `37`  | `G5`       | `38`  | `H5`       | `39`  |

**Rank 6 (value = file + 40):**
| Constant   | Value | Constant   | Value | Constant   | Value | Constant   | Value |
|------------|-------|------------|-------|------------|-------|------------|-------|
| `A6`       | `40`  | `B6`       | `41`  | `C6`       | `42`  | `D6`       | `43`  |
| `E6`       | `44`  | `F6`       | `45`  | `G6`       | `46`  | `H6`       | `47`  |

**Rank 7 (value = file + 48):**
| Constant   | Value | Constant   | Value | Constant   | Value | Constant   | Value |
|------------|-------|------------|-------|------------|-------|------------|-------|
| `A7`       | `48`  | `B7`       | `49`  | `C7`       | `50`  | `D7`       | `51`  |
| `E7`       | `52`  | `F7`       | `53`  | `G7`       | `54`  | `H7`       | `55`  |

**Rank 8 (value = file + 56):**
| Constant   | Value | Constant   | Value | Constant   | Value | Constant   | Value |
|------------|-------|------------|-------|------------|-------|------------|-------|
| `A8`       | `56`  | `B8`       | `57`  | `C8`       | `58`  | `D8`       | `59`  |
| `E8`       | `60`  | `F8`       | `61`  | `G8`       | `62`  | `H8`       | `63`  |

**Null square:**
| Constant   | Value | Description                     |
|------------|-------|---------------------------------|
| `NS`       | `64`  | Null square — undefined/off-board square. |

### Examples

```php
$sq = \shakmaty\Square::fromAscii('e4');
echo $sq->file()->toChar(); // 'e'
echo $sq->rank()->toChar(); // '4'
var_dump($sq->isLight());   // true

$sq2 = \shakmaty\Square::fromCoords(new \shakmaty\File(\shakmaty\File::D), new \shakmaty\Rank(\shakmaty\Rank::FIFTH));
echo $sq2->__toString();      // 'd5'

// Create directly from a constant — A1 is the bottom-left corner
$sq3 = new \shakmaty\Square(\shakmaty\Square::A1);   // value: 0
var_dump($sq3->isDark());                  // true (a1 is dark)

// Flip horizontally: H8 → A8 (file mirrored, rank unchanged)
$h8 = new \shakmaty\Square(\shakmaty\Square::H8);     // value: 63
var_dump($h8->flipHorizontal()->value() === \shakmaty\Square::A8);   // true

// Flip vertically: H8 → H1 (rank mirrored, file unchanged)
var_dump($h8->flipVertical()->value() === \shakmaty\Square::H1);     // true
```
