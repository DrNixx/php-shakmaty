# php_shakmaty

PHP binding for the [shakmaty](https://crates.io/crates/shakmaty) chess library. Provides native PHP classes in the `shakmaty\` namespace for working with chess positions, moves, and notations (FEN/SAN/UCI), backed by a Rust implementation via [ext-php-rs](https://ext-php.rs).

## Requirements

- **PHP** 8.2+
- **Rust toolchain** (edition 2024) with `cargo` available on PATH
- **OS:** Windows or Linux

## Installation

### Build from source

```bash
cd php_shakmaty
cargo build --release
```

The resulting shared library will be at:

| OS       | Path                                  |
|----------|---------------------------------------|
| Windows  | `target/release/php_shakmaty.dll`     |
| Linux    | `target/release/libphp_shakmaty.so`   |

### Load the extension

Add to your `php.ini`:

```ini
extension=php_shakmaty
; On Linux:
; extension=/path/to/target/release/libphp_shakmaty.so
```

Or load dynamically at runtime from a PHP script:

```php
dl('php_shakmaty.dll');   // Windows
// dl('/abs/path/libphp_shakmaty.so');  // Linux
```

## Quick Start

```php
<?php
// Ensure the extension is loaded
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll');
}

// Starting position
$pos = new \shakmaty\Chess();
echo $pos->legalMovesCount(); // 20

// Play a move via SAN notation
$pos->playSan('e4');
echo $pos->turn();            // 0 (Black)

// Play a UCI-encoded move
$pos->playUci('e7e5');

// Serialize to FEN
echo $pos->toFen();

// Check game state
var_dump($pos->isCheckmate());
```

## Class Reference

| Class         | Namespace              | Description                              |
|---------------|------------------------|------------------------------------------|
| Color         | `shakmaty\Color`       | Piece color (White/Black)                |
| Role          | `shakmaty\Role`        | Piece type (Pawn..King)                  |
| CastlingSide  | `shakmaty\CastlingSide`| Castling direction (King/Queen side)     |
| CastlingMode  | `shakmaty\CastlingMode`| Castling variant (Standard/Chess960)    |
| EnPassantMode | `shakmaty\EnPassantMode`| En passant legality mode               |
| File          | `shakmaty\File`        | Board file (A..H)                        |
| Rank          | `shakmaty\Rank`        | Board rank (First..Eighth)               |
| Square        | `shakmaty\Square`      | Board square (A1..H8)                    |
| Piece         | `shakmaty\Piece`       | Colored piece                            |
| Bitboard      | `shakmaty\Bitboard`    | 64-bit square set                        |
| Board         | `shakmaty\Board`       | Piece positions on the board             |
| Setup         | `shakmaty\Setup`       | Position setup (not necessarily legal)   |
| Move          | `shakmaty\Move`        | Chess move                               |
| MoveList      | `shakmaty\MoveList`    | Collection of moves                      |
| Chess         | `shakmaty\Chess`       | Main chess position class                |
| Position      | `shakmaty\Position`    | Abstract position base class (Chess extends it) |
| Attacks       | `shakmaty\Attacks`     | Attack and ray lookup tables             |
| Variant       | `shakmaty\Variant`     | Chess variant discriminant (Chess, Atomic, Antichess, KingOfTheHill, ThreeCheck, Crazyhouse, RacingKings, Horde) |
| VariantPosition | `shakmaty\VariantPosition` | Dynamically dispatched chess variant position |
| RemainingChecks | `shakmaty\RemainingChecks` | Three-Check remaining checks (0..3)        |
| ByRole        | `shakmaty\ByRole`      | Piece counts per role (`ByRole<u8>`)         |
| Pockets       | `shakmaty\Pockets`     | Crazyhouse pockets (`ByColor<ByRole<u8>>`)  |
| Zobrist64     | `shakmaty\Zobrist64`   | 64-bit Zobrist hash value                |
| Fen           | `shakmaty\Fen`         | Parsed FEN/EPD value object (static factories) |
| ParseFenError | `shakmaty\fen`         | FEN parse exceptions (base + variants)   |
| LossyFenError | `shakmaty\fen`         | Setup→FEN representability exception     |
| San           | `shakmaty\San`         | SAN notation string wrapper              |
| SanPlus       | `shakmaty\SanPlus`      | SAN move with check/checkmate suffix     |
| Uci           | `shakmaty\Uci`         | UCI notation string wrapper              |
| Reader        | `shakmaty\pgn`         | Streaming PGN document reader            |
| Game          | `shakmaty\pgn`         | A single parsed game (tags, movetext)    |
| Token         | `shakmaty\pgn`         | One element of the movetext stream       |
| Nag           | `shakmaty\pgn`         | Numeric Annotation Glyph (`$1`..`$255`)  |

## Advanced Usage

### Scholar's mate detection

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
echo $pos->outcome();          // "1-0"
```

### Bitboard operations

```php
// Single-square bitboard
$bb = \shakmaty\Bitboard::fromSquare(\shakmaty\Square::E4);
echo $bb->count();  // 1

// Combine with an entire rank via bitwise OR
$combined = $bb->bitwiseOr(\shakmaty\Bitboard::fromRank(\shakmaty\Rank::FIRST));
echo $combined->count();  // 9 (8 squares on the first rank + e4)
```

### Parsing PGN

Parse a Portable Game Notation document with `\shakmaty\pgn` — all method names use camelCase:

```php
$pgn = <<<'PGN'
[White "Alice"]
[Black "Bob"]
[Result "*"]

1. e4 $3 e5 2. Nf3 {Developing the knight} Bb5+ a6 Qxf7 *
PGN;

$reader = new \shakmaty\pgn\Reader($pgn);
while ($reader->hasMore()) {
    $game = $reader->readGame(); // null at end of input
    echo 'White: ',  $game->tag('White'), "\n";   // "Alice" — first occurrence wins; null if the tag is absent
    echo 'Outcome:', $game->outcome(), "\n";      // "*" (from the movetext marker) / "1-0", "0-1", "1/2-1/2"
    echo implode(' ', $game->mainline()), "\n";   // "e4 e5 Nf3 Bb5+ a6 Qxf7" — main moves only, variations excluded
}
```

See [docs/examples/pgn.md](docs/examples/pgn.md) for the full walkthrough (movetext tokens and comments, `readAll()` / `skipGame()`).

## Documentation

- [`docs/index.md`](docs/index.md) — full documentation index
- [`docs/api/`](docs/api/) — API reference for all classes
- [`docs/examples/`](docs/examples/) — complete usage examples
- [`stubs/`](stubs/) — PHP stub files (`.php`) for IDE autocompletion

## License

GPL-3.0-or-later. See [library/shakmaty/COPYING](library/shakmaty/COPYING) for the full license text of the underlying Rust crate.
