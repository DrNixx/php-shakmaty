# Uci API Reference

Lightweight value object for storing and validating UCI (Universal Chess Interface) long-algebraic notation strings without constructing or mutating a full `Chess` instance. Use it when you need to check whether an arbitrary string is syntactically valid as a chess move descriptor before passing it downstream (e.g., into `\shakmaty\Chess::playUci()`).

## Constructor

```php
new \shakmaty\Uci(string $uci)  // stores the UCI string verbatim; no validation performed at construction time
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `$uci` | `string` | Any arbitrary string. The constructor does **not** validate it — call `isValid()` explicitly if you need to confirm the stored value is a well-formed UCI move before using it downstream (e.g., passing to `\shakmaty\Chess::playUci()`) |

> Because construction never throws, this class is safe for use in hot paths where you want cheap syntactic pre-checks without paying the cost of full position parsing. If `isValid()` returns `false`, do **not** pass that string to `\shakmaty\Chess::playUci()`.

## Methods

| Method | Return Type | Description |
|--------|-------------|-------------|
| `__toString()` | `string` | Returns the exact UCI string as it was passed into the constructor (no normalization, trimming, or reformatting is applied) — useful for round-tripping through serialization layers that expect a plain PHP scalar (PHP magic method, also available via the `(string)` cast). |
| `isValid()` | `bool` | Validates whether the stored string conforms to standard UCI long-algebraic syntax: source square (`a1`–`h8`) followed by target square, with an optional promotion suffix (`q`, `r`, `b`, or `n`). Returns `true` iff all structural checks pass; otherwise returns `false`. |

> **Note:** Validation via `Uci::isValid()` checks *syntactic* correctness only — it does not verify that the described move is legal in a specific position (that would require full board-state analysis and is out of scope for this class). For semantic validation, parse into `Chess` via `\shakmaty\Chess::playUci()` instead.

## Example

```php
$uci = new \shakmaty\Uci('e2e4');
var_dump($uci->isValid()); // true

$promo = new \shakmaty\Uci('e7e8q');
var_dump($promo->isValid()); // true — promotion suffix is valid UCI syntax

$bad = new \shakmaty\Uci('xxx');
var_dump($bad->isValid()); // false — fails structural validation immediately; do NOT pass this to Chess::playUci()
```

## See Also

| Topic | Reference |
|-------|-----------|
| `Chess::playUci()` / `toFen()` | [chess.md](chess.md) — plays a validated UCI move into the current position (throws on illegal input, unlike this class which merely reports validity), and serializes the resulting position back out to FEN for downstream use or storage |
