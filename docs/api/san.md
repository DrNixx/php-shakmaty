# San & SanPlus API Reference

Lightweight value object for storing and validating SAN (Standard Algebraic Notation) strings without constructing or mutating a full `Chess` instance. Use it when you need to check whether an arbitrary string is syntactically valid as a chess move descriptor before passing it downstream (e.g., into `\shakmaty\Chess::playSan()`).

## Constructor

```php
new \shakmaty\San(string $san)  // stores the SAN string verbatim; no validation performed at construction time
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `$san` | `string` | Any arbitrary string. The constructor does **not** validate it — call `isValid()` explicitly if you need to confirm the stored value is a well-formed SAN move before using it downstream (e.g., passing to `\shakmaty\Chess::playSan()`) |

> Because construction never throws, this class is safe for use in hot paths where you want cheap syntactic pre-checks without paying the cost of full position parsing. If `isValid()` returns `false`, do **not** pass that string to `\shakmaty\Chess::playSan()`.

## Methods

| Method | Return Type | Description |
|--------|-------------|-------------|
| `__toString()` | `string` | Returns the exact SAN string as it was passed into the constructor (no normalization, trimming, or reformatting is applied) — useful for round-tripping through serialization layers that expect a plain PHP scalar (PHP magic method, also available via the `(string)` cast). |
| `isValid()` | `bool` | Validates whether the stored string conforms to standard SAN syntax: valid piece letter (`N`, `B`, `R`, `Q`, `K`) followed by optional disambiguation (file/rank), capture indicator (`x` if capturing), target square, and optional promotion suffix or check/mate marker. Returns `true` iff all structural checks pass; otherwise returns `false`. |

> **Note:** Validation via `San::isValid()` checks *syntactic* correctness only — it does not verify that the described move is legal in a specific position (that would require full board-state analysis and is out of scope for this class). For semantic validation, parse into `Chess` via `\shakmaty\Chess::playSan()` instead.

## Example

```php
$san = new \shakmaty\San('Nf3');
echo $san->__toString(); // "Nf3"
var_dump($san->isValid()); // true
```

## See Also

| Topic | Reference |
|-------|-----------|
| `Chess::playSan()` / `toFen()` | [chess.md](chess.md) — plays a validated SAN move into the current position (throws on illegal input, unlike this class which merely reports validity), and serializes the resulting position back out to FEN for downstream use or storage |


### Move conversion (SAN ⇄ Move)

These methods convert between a SAN string and concrete `\shakmaty\Move` objects. The `$pos` argument is a `\shakmaty\Chess` or `\shakmaty\VariantPosition` instance.

| Method | Return Type | Description |
|--------|-------------|-------------|
| `static fromMove(\shakmaty\Position $pos, Move $move)` | `\shakmaty\San` | Builds SAN for a legal `$move` in `$pos`, adding origin file/rank disambiguation only when required. Throws if `$pos` is not a `\shakmaty\Position`. |
| `toMove(\shakmaty\Position $pos)` | `\shakmaty\Move` | Resolves this SAN to the **unique** legal move in `$pos`. Throws `"Invalid SAN"` (the stored string is not valid SAN), `"Illegal SAN"` (no legal move matches), or `"Ambiguous SAN"` (several legal moves match). |
| `findMove(MoveList $moves)` | `\shakmaty\Move \| null` | Searches an explicit `\shakmaty\MoveList` for the unique move matching this SAN (in any position). Returns `null` when nothing matches, when the match is ambiguous, or when the stored string is not valid SAN — unlike `toMove()`, it never throws. |
| `matches(Move $move)` | `bool` | Whether this SAN can match `$move` in any position (position context is ignored). Returns `false` for an invalid stored string. |

```php
$pos  = new \shakmaty\Chess();
$move = (new \shakmaty\San('Nf3'))->toMove($pos);
echo $move->toLong();                                       // "Ng1f3"

echo \shakmaty\San::fromMove($pos, $move)->__toString();    // "Nf3"

$moves = $pos->legalMoves();
var_dump((new \shakmaty\San('Nf3'))->findMove($moves) !== null); // true
var_dump((new \shakmaty\San('Nf3'))->matches($move));            // true
```


## SanPlus

A Standard Algebraic Notation (SAN) move together with its optional check or checkmate suffix, as produced by the PGN reader. It lives in the **root namespace** (`\shakmaty\SanPlus`), not in `shakmaty\pgn`. There is **no public constructor** — instances are created via `SanPlus::fromAscii()` and returned from `\shakmaty\pgn\Token::san()`.

### Static Methods

| Method | Return Type | Description |
|--------|-------------|-------------|
| `static fromAscii(string $text)` | `\shakmaty\SanPlus` | Parses a SAN move with an optional suffix (e.g., `"Qxf7#"`, `"Nf3+"`). **Throws** a PHP exception if `$text` is not syntactically valid SAN — wrap in `try/catch` when parsing untrusted input, or prefer the null-returning `\shakmaty\pgn\Token::san()` path from an already-parsed game. |

### Methods

| Method | Return Type | Description |
|--------|-------------|-------------|
| `__toString()` | `string` | The full notation including any suffix, e.g., `"Qxf7#"` — the exact form you would expect to see in a PGN movetext stream. Round-trips with what was passed into `fromAscii()`. |
| `san()` | `string` | Only the SAN part without the check/checkmate suffix (e.g., `"Qxf7"`) — useful when feeding moves back into `\shakmaty\Chess::playSan()` or comparing bare move identities. |
| `suffix()` | `string \| null` | The trailing marker: `"+"` for a checking move, `"#"` for checkmate, or `null` when the move carries no such suffix (the common case). |
| `isCheck()` | `bool` | Whether this token denotes a checking move (`suffix() === "+"`). Always false for checkmate moves — use `isCheckmate()` separately. |
| `isCheckmate()` | `bool` | Whether this token denotes a checking move that ends the game (`suffix() === "#"`). Mutually exclusive with `isCheck()`. |
| `isNull()` | `bool` | Whether this token is a null move (`"--"` / `"Z0"`) — an explicit no-move placeholder rather than a regular SAN move. |
| `toSan()` | `\shakmaty\San` | The SAN part wrapped in a plain `\shakmaty\San` instance (suffix dropped), for use anywhere the base-namespace value object is expected. |

### Example

```php
$move = \shakmaty\SanPlus::fromAscii('Qxf7#'); // throws on invalid SAN, e.g. 'Xf7' or ''
echo (string)$move;             // "Qxf7#"
echo $move->san();              // "Qxf7"
var_dump($move->suffix());      // "#"
var_dump($move->isCheckmate()); // true
```

