# Fen API Reference

Parsed value object for Forsyth-Edwards Notation (FEN) and EPD strings. It wraps
the Rust `shakmaty::fen::Fen`, so an instance is always syntactically valid once
constructed. Use it to parse/normalize notation, validate cheaply, and convert
between `Setup`, positions and FEN.

## Construction

The public constructor has been removed. Create instances with the static factories:

| Factory | Returns | Description |
|---------|---------|-------------|
| `Fen::empty()` | `Fen` | Returns the empty FEN (`8/8/8/8/8/8/8/8 w - - 0 1`). Useful as a neutral starting value without parsing a string. |
| `Fen::parse(string $fen)` | `Fen` | Parses a FEN or EPD string. Missing fields are filled with defaults (the board field is required). Throws a `shakmaty\fen\ParseFenError` subclass on invalid input. |
| `Fen::fromPosition(Position $pos, ?int $mode = null)` | `Fen` | Builds a FEN from a `Chess` or `VariantPosition`. `$mode`: 0 = Legal (default), 1 = PseudoLegal, 2 = Always. Throws when `$pos` is not a `Position`. |
| `Fen::fromSetup(Setup $setup)` | `Fen` | Builds a FEN from a `Setup`. Throws `shakmaty\fen\LossyFenError` if the setup cannot be represented losslessly. |

## Methods

| Method | Return Type | Description |
|--------|-------------|-------------|
| `__toString()` | `string` | Returns the (normalized) FEN string. PHP magic method — also available via the `(string)` cast. |
| `isValid(string $fen)` | `bool` | **Static.** Whether the string is a syntactically valid FEN/EPD. Never throws. |
| `getSetup()` | `Setup` | Returns the FEN contents as a `Setup`. |
| `getPosition(?int $variant = null, ?int $mode = null)` | `VariantPosition` | Converts to a `VariantPosition`. `$variant`: 0 = Chess (default) … 7 = Horde. `$mode`: 0 = Standard (default), 1 = Chess960. Throws on an invalid position. |

## Exceptions

| Exception | Thrown when |
|-----------|-------------|
| `shakmaty\fen\ParseFenError` | Base class for all FEN parse errors (extends `\Exception`). |
| `shakmaty\fen\InvalidFen` | No valid board part / extra fields. |
| `shakmaty\fen\InvalidBoard` | Malformed board part. |
| `shakmaty\fen\InvalidPocket` | Invalid Crazyhouse pocket part. |
| `shakmaty\fen\InvalidTurn` | Invalid side-to-move field. |
| `shakmaty\fen\InvalidCastling` | Invalid castling-rights field. |
| `shakmaty\fen\InvalidEpSquare` | Invalid en passant square field. |
| `shakmaty\fen\InvalidRemainingChecks` | Invalid remaining-checks field (Three-Check). |
| `shakmaty\fen\InvalidHalfmoveClock` | Invalid halfmove clock field. |
| `shakmaty\fen\InvalidFullmoves` | Invalid fullmove number field. |
| `shakmaty\fen\LossyFenError` | `fromSetup()` on a setup that cannot be represented losslessly. `getCode()` is a bitmask of `LossyFenError::PROMOTED` (1), `CASTLING_RIGHTS` (2), `POCKETS` (4). |

## Example

```php
use shakmaty\Fen;

$fen = Fen::parse('rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1');
echo (string) $fen;

var_dump(Fen::isValid('invalid')); // false

$setup = $fen->getSetup();          // shakmaty\Setup
$pos   = $fen->getPosition();       // shakmaty\VariantPosition (Chess)
$chess = Fen::fromPosition(new \shakmaty\Chess());

try {
    Fen::parse('not a fen');
} catch (\shakmaty\fen\InvalidBoard $e) {
    echo $e->getMessage();
}
```

## See Also

| Topic | Reference |
|-------|-----------|
| `Chess::fromFen()` / `toFen()` | [chess.md](chess.md) |
| `VariantPosition::fromFen()` / `toFen()` | [variant.md](variant.md) |
| `Setup` | [setup.md](setup.md) |
