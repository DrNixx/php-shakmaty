# Getting Started with shakmaty

## Installation

### Prerequisites

- PHP 8.2 or later
- Rust toolchain (edition 2024) — install from [rustup.rs](https://rustup.rs/)
- C compiler (MSVC on Windows, GCC on Linux)

### Build from Source

```bash
git clone <repository-url>
cd php_shakmaty
cargo build --release
```

After a successful build, the extension binary will be at:

- **Windows:** `target/release/php_shakmaty.dll`
- **Linux:** `target/release/libphp_shakmaty.so`

### Loading the Extension

Copy the binary to your PHP extensions directory, then add to `php.ini`:

```ini
extension=php_shakmaty
```

Alternatively, load it at runtime in your script:

```php
<?php
if (!extension_loaded('shakmaty')) {
    dl('php_shakmaty.dll'); // Windows
    // dl('php_shakmaty.so'); // Linux
}
```

Verify the extension is loaded:

```php
var_dump(extension_loaded('shakmaty')); // bool(true)
```

## Basic Usage

### Creating a Position

```php
// Standard starting position
$pos = new \shakmaty\Chess();
echo $pos->legalMovesCount(); // 20
```

### Making Moves via SAN

```php
$pos = new \shakmaty\Chess();
$pos->playSan('e4');    // 1. e4
$pos->playSan('e5');    // 1. ... e5
$pos->playSan('Qh5');   // 2. Qh5

echo $pos->turn();      // 0 (Black's turn)
echo $pos->toFen();     // rnbqkbnr/pppp1ppp/8/4p3/4P2Q/8/PPPP1PPP/RNB1KBNR b KQkq - 1 1
```

### Making Moves via UCI

```php
$pos = new \shakmaty\Chess();
$pos->playUci('e2e4');
$pos->playUci('e7e5');
$pos->playUci('d1h5');  // Qh5 in UCI notation
```

### Working with FEN

```php
// Create from FEN
$pos = \shakmaty\Chess::fromFen(
    'r1bqkb1r/pppp1Qpp/2n2n2/4p3/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 4'
);
var_dump($pos->isCheckmate()); // true

// Export to FEN
echo $pos->toFen();
```

### Validating Notation Strings

```php
$fen = \shakmaty\Fen::parse('rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1');
echo (string) $fen;
var_dump(\shakmaty\Fen::isValid('not a fen')); // false

$san = new \shakmaty\San('Nf3');
echo (string)$san; // "Nf3"

$uci = new \shakmaty\Uci('e2e4');
var_dump($uci->isValid()); // true
```

### Analyzing the Position

```php
$pos = new \shakmaty\Chess();

var_dump($pos->isCheck());       // false
var_dump($pos->isCheckmate());   // false
var_dump($pos->isStalemate());   // false
echo $pos->outcome();            // "*" (game in progress)
echo $pos->turn();               // 1 (White)
echo $pos->fullmoves();          // 1
echo $pos->halfmoves();          // 0
```

### Enumerating Legal Moves

```php
$pos = new \shakmaty\Chess();
$moves = $pos->legalMoves();
echo $moves->count(); // 20

// Access individual moves
$first = $moves->get(0);
echo $first->toLong(); // e.g., "a2a3"
```

## Next Steps

- Browse the [API Reference](api/enums.md) for detailed class documentation
- Check the [Examples](examples/scholar-mate.md) for complete usage scenarios
