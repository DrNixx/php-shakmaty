# Zobrist64 API Reference

Ports of the 64-bit Zobrist hashing from
[`shakmaty::zobrist`](https://docs.rs/shakmaty/0.30/shakmaty/zobrist/index.html).

## Zobrist64

`shakmaty\Zobrist64` is an opaque 64-bit hash value. Mirrors
`shakmaty::zobrist::Zobrist64`.

> PHP integers are **signed** 64-bit. The read-only `$value` property returns the raw bit pattern as a
> signed integer, so hashes with the top bit set appear negative. Use `toHex()`
> (or `__toString()`) when you need the exact unsigned value, e.g. as an array key
> or for logging.

### Properties

| Property | Type | Access | Description |
|----------|------|--------|-------------|
| `$value` | `int` | read-only | Raw bit pattern as a signed integer (may be negative) |

### Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `__construct(?int $value = null)` | `Zobrist64` | Creates a value from a raw 64-bit bit pattern (default `0`) |
| `Zobrist64::fromHex(string $hex)` | `Zobrist64` | Parses a hex string (optional `0x`); throws `"Invalid hex"` |
| `toHex()` | `string` | Unsigned 16-digit lowercase hex string |
| `__toString()` | `string` | Alias of `toHex()` (PHP magic method) |
| `isZero()` | `bool` | Whether the value is zero |
| `bitwiseXor(Zobrist64 $other)` | `Zobrist64` | Bitwise XOR with another hash |
| `equals(Zobrist64 $other)` | `bool` | Whether two hashes are equal |
| `legacy24()` | `int` | Top 24 bits (`(value >>> 40) & 0xFFFFFF`) — legacy `ZobristHash.php` compatibility |

### Example

```php
$h = new \shakmaty\Chess()->zobristHash();
echo $h->toHex();                         // e.g. "463b96181691fc9c"

$same = \shakmaty\Zobrist64::fromHex($h->toHex());
var_dump($h->equals($same));              // true
var_dump($h->bitwiseXor($h)->isZero());   // true
```

### Legacy 24-bit hashes

`legacy24()` returns the top 24 bits of the 64-bit value, shifted into the low 24 bits
(`(value >>> 40) & 0xFFFFFF`). This reproduces the hashes of the old `ZobristHash.php`
engine, which indexed its `eco` collection by the upper 24 bits of each Polyglot value
(e.g. `0x9d3924` is the top 24 bits of `0x9D39247E33776D41`). The shift is logical, so
the result is always in `0 ..= 0xFFFFFF`, even when the top bit of the value is set.

```php
$h = \shakmaty\Zobrist64::fromHex('9d39247e33776d41');
echo dechex($h->legacy24());                    // "9d3924"

$start = new \shakmaty\Chess();
echo dechex($start->zobristHash()->legacy24()); // "463b96"
```

The legacy `hashBase64` is reconstructed by concatenating `dechex()` of each legacy hash
and base64-encoding the result (lowercase hex, no padding):

```php
$hashes = [$start->zobristHash()->legacy24()];
$joined = implode('', array_map('dechex', $hashes));
echo base64_encode($joined);                    // "NDYzYjk2" for the start position alone
```

## Position hashing

`shakmaty\Position` (and therefore `Chess` and `VariantPosition`) exposes the
64-bit hash:

| Method | Returns | Description |
|--------|---------|-------------|
| `zobristHash(?int $mode = null)` | `Zobrist64` | Hash of the whole position, excluding halfmove/fullmove counters |
| `updateZobristHash(Zobrist64 $current, Move $m, ?int $mode = null)` | `?Zobrist64` | Incremental update after legal move `$m`, or `null` if unsupported |

`$mode` is the en passant mode: `0` = Legal (default), `1` = PseudoLegal,
`2` = Always.

`updateZobristHash()` returns `null` when no efficient incremental update is
implemented for the situation — for example when the position has an en passant
square, or for a double pawn push, a king move, a move touching a castling-right
square, castling, en passant or a Crazyhouse drop. In those cases recompute with
`zobristHash()`.

### Example

```php
$pos = new \shakmaty\Chess();
$hash = $pos->zobristHash();
echo $hash->toHex(); // "463b96181691fc9c"

// Incremental update for a simple knight move matches a full recomputation
$move = null;
foreach (/* ... */ as $candidate) {
    if ($candidate->toLong() === 'Ng1f3') { $move = $candidate; }
}
$next = $pos->updateZobristHash($hash, $move);
var_dump($next->equals($pos->play($move)->zobristHash())); // true
```

### Variant state

The hash covers variant-specific state too: Crazyhouse pockets and promoted
pieces, and Three-Check remaining checks. Two notes follow from the bit scheme:

- **Empty** Crazyhouse pockets and the **default** `3+3` remaining checks
  contribute zero, so the default Crazyhouse / Three-Check position hashes the
  same as standard chess.
- Any non-empty pocket or changed remaining-check value changes the hash.

## See also

- [`Position`](position.md) — `zobristHash()`, `updateZobristHash()`
- [`Move`](move.md) — moves passed to `updateZobristHash()`
