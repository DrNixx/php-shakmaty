# PGN API Reference (`shakmaty\pgn`)

Streaming parser for [Portable Game Notation (PGN)](https://www.chess.com/terms/what-is-pgn) documents, backed by the [`pgn-reader`](https://crates.io/crates/pgn-reader) crate. The namespace `shakmaty\pgn` contains four classes:

| Class | Purpose |
|-------|---------|
| `\shakmaty\pgn\Reader` | Streaming reader over a PGN document (one or more games) |
| `\shakmaty\pgn\Game` | A single parsed game: tag pairs, movetext tokens and outcome |
| `\shakmaty\pgn\Token` | One element of the movetext stream (move, NAG, comment, variation delimiter or outcome) |
| `\shakmaty\pgn\Nag` | Numeric Annotation Glyph (`$1`..`$255`, e.g. `$4` = blunder) |

> **Note:** All method names in this namespace are camelCase (e.g., `readGame()`); the magic `__toString()` is available on value objects, consistent with the rest of the extension's classes such as `\shakmaty\Chess`.

## Reader

A streaming reader for PGN documents. The document is supplied to the constructor; games can be read one at a time (`hasMore()` / `readGame()`) or all at once (`readAll()`). Irrecoverable parse errors throw PHP exceptions.

### Constructor

```php
new \shakmaty\pgn\Reader(string $pgn)  // full PGN document as a string, may contain multiple games
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `$pgn` | `string` | The complete PGN text to parse. Parsing is lazy — nothing is read until you call one of the reading methods |

### Methods

| Method | Return Type | Description |
|--------|-------------|-------------|
| `setSupportedTagLineLength(int $bytes)` | `void` | Configures the buffer for tag lines *at least* as long as `$bytes`. Values below 255 are clamped to 255. Default: 255 bytes |
| `setSupportedCommentLength(int $bytes)` | `void` | Same idea for comments; values below 255 are clamped to 255 (default). Oversized comments arrive as chunks from the underlying reader but are merged by this binding into a single comment token, so callers always see one logical comment per `{ ... }` block |
| `readGame()` | `\shakmaty\pgn\Game \| null` | Reads and returns the next game, or `null` when the document is exhausted. Advances the internal offset. Throws on irrecoverable parse errors |
| `hasMore()` | `bool` | Returns whether another game follows in the document, without parsing it fully |
| `skipGame()` | `bool` | Skips past the next game (useful to jump ahead cheaply). Returns `true` if a game was skipped, `false` at end of input. Throws on irrecoverable parse errors |
| `readAll()` | `\shakmaty\pgn\Game[]` | Reads all remaining games into an array and returns it (equivalent to looping over `hasMore()` / `readGame()`) |
| `offset()` | `int` | Current byte offset of the reader within the document — useful for resuming or reporting progress on large files |
| `reset()` | `void` | Rewinds the reader back to the beginning of the document so it can be read again from scratch |

### Example

```php
$reader = new \shakmaty\pgn\Reader($pgn);

while ($reader->hasMore()) {
    $game = $reader->readGame(); // null at end of input
    echo $game->tag('White'), ' vs ', $game->tag('Black') . "\n";
}
```

## Game

A single game parsed from a PGN document: its tag pairs, movetext tokens and outcome. Instances are normally produced by `Reader::readGame()` / `Reader::readAll()`.

### Constructor

```php
new \shakmaty\pgn\Game()  // creates an empty game (no tags, no moves) — mainly useful for testing
```

No parameters.

### Methods

| Method | Return Type | Description |
|--------|-------------|-------------|
| `tags()` | `array` | All tag pairs as an associative array mapping name to value (`['White' => 'Alice', ...]`). If a tag name occurs more than once, the last occurrence wins in this map. Use `tag($name)` when you need first-occurrence semantics instead |
| `tag(string $name)` | `string \| null` | Returns the value of the first tag with the given name (e.g., `'White'`, `'Result'`), or `null` if no such tag exists. Tag names are case-sensitive as written in the source document |
| `movetext()` | `\shakmaty\pgn\Token[]` | The movetext tokens in source order, including variation delimiters (`variation_begin` / `variation_end`) and outcome markers — iterate with a loop over `$token->kind()` to dispatch on token type |
| `outcome()` | `string` | Game termination marker found in the movetext: `"1-0"`, `"0-1"` or `"1/2-1/2"`. Returns `"*"` when there is no explicit outcome marker (or an unknown-result one) — note this is independent of what the `[Result ...]` tag says, since tags are not cross-checked against the movetext |
| `mainline()` | `string[]` | SAN strings of the mainline moves only; any move inside a `( ... )` variation is excluded. Handy for replaying or indexing games without walking the token stream |

### Example

```php
$reader = new \shakmaty\pgn\Reader($pgn);
$game   = $reader->readGame();

echo $game->tag('White');              // "Alice"
var_dump($game->tags());               // ['Event' => ..., 'Site' => ..., ...]
echo implode(' ', $game->mainline());  // "e4 e5 Nf3 d6 Bb5+ c6 Qc7 ..."
```

## Token

A single element of PGN movetext, produced by the `Reader` and exposed through `Game::movetext()`. There is **no public constructor** — tokens are created only while parsing. Use `kind()` to discriminate a token and then call the matching accessor (`san()`, `nag()`, `comment()`, `outcome()`); accessors that do not apply return `null` rather than throwing, so they can be called unconditionally if you prefer an early-return style over a big switch on `kind()`.

### Methods

| Method | Return Type | Description |
|--------|-------------|-------------|
| `kind()` | `string` | One of `"san"`, `"nag"`, `"comment"`, `"variation_begin"`, `"variation_end"` or `"outcome"` — the token's type, used to decide which accessor below is meaningful for this instance |
| `san()` | `\shakmaty\SanPlus \| null` | The SAN move if this token is a move (`kind() === "san"`), otherwise `null`. Returns a full `SanPlus`, so you can inspect the check/checkmate suffix separately from the bare SAN string via `$token->san()->suffix()` / `isCheck()` etc. |
| `nag()` | `\shakmaty\pgn\Nag \| null` | The NAG if this token is an annotation glyph (`kind() === "nag"`), otherwise `null`. Read the numeric value with the read-only `$token->nag()->value` property, or a human-readable form via `__toString()` / `glyph()` on that object. |
| `comment()` | `string \| null` | The comment text if this token is a `{ ... }` annotation (`kind() === "comment"`), otherwise `null`. Oversized comments are already merged into one string by the reader, so you never need to reassemble chunks yourself here. |
| `outcome()` | `string \| null` | `"1-0"`, `"0-1"`, `"1/2-1/2"` or `"*"` if this token is a game termination marker (`kind() === "outcome"`), otherwise `null`. This mirrors what the whole-game accessor `Game::outcome()` reports, but scoped to one position in the movetext stream. |
| `__toString()` | `string` | A printable representation of whatever kind of token this is: the SAN move (e.g., `"Nf3+"`) for moves, `$n` form for NAGs, raw text for comments, literal parentheses (`"("`, `")"`) for variation delimiters and the outcome string itself — useful for logging or re-serializing a movetext stream without inspecting each token's type first. |

### Example

```php
foreach ($game->movetext() as $token) {
    switch ($token->kind()) {
        case 'san':             echo (string)$token, ' '; break; // "Nf3+" etc.
        case 'nag':             echo '[', $token->nag()->glyph(), ']'; break;
        case 'comment':         echo '{', $token->comment(), '}';     break;
        case 'variation_begin': echo '(';                              break;
        case 'variation_end':   echo ') ';                             break;
        case 'outcome':         echo (string)$token;             break; // "1-0" / "*" ...
    }
}
```

## Nag

A Numeric Annotation Glyph (NAG) — the `$n` annotations PGN uses to mark move quality, such as `!`, `??` or a purely numeric code like `$42`. Values are clamped into the range 0..=255 at construction time.

### Constructor

```php
new \shakmaty\pgn\Nag(int $value)  // values outside 0..=255 are silently clamped, not rejected
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `$value` | `int` | The numeric NAG code (e.g., `\shakmaty\pgn\Nag::BLUNDER`). Out-of-range values do **not** throw — they are clamped to the nearest valid value in 0..=255, so this constructor is safe for untrusted input if you only care about a well-formed NAG object rather than exact fidelity. |

### Constants

The six well-known NAG constants, their numeric values and the symbolic glyph each one returns from `glyph()`:

| Constant | Value | Glyph (`glyph()`) | Meaning |
|----------|-------|-------------------|---------|
| `\shakmaty\pgn\Nag::GOOD_MOVE`        | 1 | `"!"`  | Good move (positive annotation) |
| `\shakmaty\pgn\Nag::MISTAKE`          | 2 | `"?"`  | Mistake — weaker than the best available line |
| `\shakmaty\pgn\Nag::BRILLIANT_MOVE`   | 3 | `"!!"` | Brilliant move (strongest positive annotation) |
| `\shakmaty\pgn\Nag::BLUNDER`          | 4 | `"??"` | Blunder — a serious mistake losing material or the game |
| `\shakmaty\pgn\Nag::SPECULATIVE_MOVE` | 5 | `"!?"` | Speculative move — risky but may have a point |
| `\shakmaty\pgn\Nag::DUBIOUS_MOVE`     | 6 | `"?!"` | Dubious/questionable choice |

### Properties

| Property | Type | Access | Description |
|----------|------|--------|-------------|
| `$value` | `int` | read-only | The numeric NAG value in range 0..=255 (already clamped by the constructor). Values 7–255 have no symbolic glyph and are only meaningful as raw codes. |

### Methods

| Method | Return Type | Description |
|--------|-------------|-------------|
| `__toString()` | `string` | The NAG in its PGN text form, e.g., `"$4"` or `"$42"`. Use this when re-serializing tokens back into movetext. |
| `glyph()` | `string \| null` | The symbolic glyph for the six well-known values 1–6 (`"!"`, `"?"`, `"!!"`, `"??"`, `"!?"`, `"?!"`). Returns **null** (not an empty string) for purely numeric annotations like `$42`. |
| `static fromAscii(string $text)` | `\shakmaty\pgn\Nag \| null` | Parses a NAG from symbolic form (`"??"` etc.) or PGN text form (`"$24"`). Returns **null** instead of throwing on unrecognized input — contrast with `\shakmaty\SanPlus::fromAscii()`, which throws. |

### Example

```php
$nag = new \shakmaty\pgn\Nag(\shakmaty\pgn\Nag::BLUNDER); // value 4; out-of-range values are clamped, not rejected
echo $nag->value;     // "4" (int) — the raw numeric code
echo (string)$nag; // "$4"   — its PGN text form between moves in a movetext stream
echo $nag->glyph();     // "??"  — distinct from MISTAKE's single "?"

$parsed = \shakmaty\pgn\Nag::fromAscii('$24'); // numeric form; null for unrecognized input (e.g. 'xyz')
echo $parsed->value;  // "24" (int)
```

## See Also

| Topic | Reference |
|-------|-----------|
| `San` (base namespace) — the value object returned by `\shakmaty\SanPlus::toSan()` and usable with `\shakmaty\Chess::playSan()` for replaying parsed moves into a live position. | [san.md](san.md) |
| Full multi-game parsing walkthrough, including movetext token dispatch and `readAll()` / `skipGame()`. | [../examples/pgn.md](../examples/pgn.md) |
