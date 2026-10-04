# PGN Parsing — Complete Example

This example demonstrates parsing Portable Game Notation (PGN) documents with the `\shakmaty\pgn` classes (`Reader`, `Game`, `Token`). Method names follow PHP conventions (camelCase). The fixture below is illustrative: PGN movetext is parsed as notation only — move legality against a position is never checked by the reader (use `\shakmaty\Chess::playSan()` for that).

## Sample Document

The examples parse a document containing two games: one annotated with comments, NAGs and a variation; another short game. Both carry an explicit termination marker in their movetext (`*` = unknown result / abandoned, `0-1`).

```php
$pgn = <<<'PGN'
[Event "Example Game"]
[Site "?"]
[White "Alice"]
[Black "Bob"]
[Result "*"]

{Opening} 1. e4 $3 e5 (2... c6) 2. Nf3 {Developing the knight} Nc6 Bb5+ a6 Qxf7 * 

[Event "Second Game"]
[White "Carol"]
[Black "Dave"]
[Result "0-1"]

1. d4 d5 2. e4 dxe4 0-1
PGN;
```

## Reading Games One by One

`Reader::hasMore()` / `readGame()` stream the document game by game without loading everything into memory at once:

```php
$reader = new \shakmaty\pgn\Reader($pgn);

while ($reader->hasMore()) {
    $game = $reader->readGame(); // null when the document is exhausted

    echo 'White: ',  $game->tag('White'), "\n";   // "Alice" / "Carol" — tag() returns the first occurrence of a name, or null if absent
    echo 'Black: ',  $game->tag('Black'), "\n";   // "Bob" / "Dave"

    foreach ($game->tags() as $name => $value) {
        printf("  [%s] %s\n", $name, $value);      // all tag pairs; on duplicate names the last occurrence wins in this map
    }

    echo 'Outcome: ', $game->outcome(), "\n";     // "*" / "0-1" — from the movetext marker, not cross-checked against [Result]
    echo 'Mainline:', implode(' ', $game->mainline()), "\n";  // SAN strings of main moves only (variation excluded)
}
```

Expected output:

```
White: Alice
Black: Bob
  [Event] Example Game
  [Site] ?
  [White] Alice
  [Black] Bob
  [Result] *
Outcome: *
Mainline:e4 e5 Nf3 Nc6 Bb5+ a6 Qxf7

White: Carol
Black: Dave
  [Event] Second Game
  ...
Outcome: 0-1
Mainline:d4 d5 e4 dxe4
```

> **Note:** `Game::outcome()` reports the termination marker found in the movetext — `"1-0"`, `"0-1"` or `"1/2-1/2"`; it returns `"*` when there is no such marker (or an explicit unknown-result one). It does not cross-check against the `[Result ...]` tag — read that with `$game->tag('Result')`.

## Walking the Movetext Tokens

`Game::movetext()` returns every element of the movetext in source order: SAN moves, NAGs (`$n`), comments (`{ ... }`), variation delimiters and outcome markers. Dispatch on `Token::kind()`:

```php
foreach ($game->movetext() as $token) {
    switch ($token->kind()) {
        case 'san':             echo (string)$token, ' '; break; // "Nf3+" etc.; inspect the suffix via $token->san()->isCheck()/isCheckmate()
        case 'nag':  {          $glyph = $token->nag()->glyph();     // symbolic glyph for values 1-6, null otherwise (e.g. "$42")
                                echo ($glyph !== null ? '[' . $glyph . ']' : (string)$token), ' '; break; } // "[??]" for a blunder / "$3" as-is
        case 'comment':         echo '{', $token->comment(), '}';     break;      // oversized comments are already merged into one token by the reader
        case 'variation_begin': echo '(';                              break;   // start of a ( ... ) variation line
        case 'variation_end':   echo ') ';                             break;   // end of that variation
        case 'outcome':         echo (string)$token;             break;    // "1-0" / "0-1" / "1/2-1/2", or "*" for an unknown result marker
    }
}
```

For a single move token you can also drill into its `SanPlus` value:

```php
foreach ($game->movetext() as $token) {
    $san = $token->san(); // null for non-move tokens — safe to call unconditionally on any kind of token
    if ($san !== null && $san->isCheck()) {          // or isCheckmate() when you only care about moves ending in "#"
        echo 'Checking move: ', (string)$san, "\n";   // "Bb5+" (suffix included); $san->san() gives the bare "Bb5"
    }
}
```

## Reading All Games at Once, or Skipping Ahead

`readAll()` collects every remaining game into an array; `skipGame()` jumps past a single game cheaply when you only need later ones:

```php
$reader = new \shakmaty\pgn\Reader($pgn);

// Option A — everything at once (equivalent to looping hasMore()/readGame()):
$games = $reader->readAll(); // array of shakmaty\pgn\Game, in document order
echo 'Games: ', count($games), "\n";                 // 2
foreach ($games as $i => $game) {
    echo sprintf("%d. %s vs %s — %s\n", $i + 1, $game->tag('White'), $game->tag('Black'), $game->outcome());
}

// Option B — skip the first game and read only the second:
$reader = new \shakmaty\pgn\Reader($pgn);
var_dump($reader->skipGame()); // true — one game was skipped past without being materialized into a Game object; false at end of input
echo $reader->offset(), "\n";   // byte offset after skipping (useful for progress reporting on large files)

$second = $reader->readAll();  // [Game] containing only "Carol vs Dave"
var_dump($reader->hasMore());  // false — document exhausted; readGame() would now return null
```

## Resetting the Reader

`reset()` rewinds to byte offset `0`, so a single reader instance can be replayed:

```php
$reader = new \shakmaty\pgn\Reader($pgn);
var_dump(count($reader->readAll())); // 2 — document now exhausted at some non-zero offset()
echo $reader->offset(), "\n";         // byte position after the last game (near end of input)

$reader->reset();                     // back to the beginning; offset() is 0 again
var_dump($reader->hasMore());        // true — readGame()/readAll() work as before
```

## See Also

| Topic | Reference |
|-------|-----------|
| Full API reference for `Reader`, `Game`, `Token` and `Nag`. | [../api/pgn.md](../api/pgn.md) |
| `SanPlus` (root namespace `\shakmaty\SanPlus`) — SAN move with check/checkmate suffix. | [../api/san.md](../api/san.md) |
| Replaying parsed SAN moves into a live position via `\shakmaty\Chess::playSan()`. | [chess.md](../api/chess.md), [san.md](../api/san.md) |
