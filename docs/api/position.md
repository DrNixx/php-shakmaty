# Position API Reference (abstract)

`shakmaty\Position` is an abstract base class that mirrors the Rust
[`shakmaty::Position`](https://docs.rs/shakmaty/0.30/shakmaty/trait.Position.html)
trait. It declares the full position contract; concrete classes such as
[`Chess`](chess.md) extend it and implement every method.

`Position` **cannot be instantiated** — its constructor is protected and the
class is abstract. Use it only for type checks:

```php
$pos = new \shakmaty\Chess();
var_dump($pos instanceof \shakmaty\Position); // true

// new \shakmaty\Position(); // Error: cannot instantiate abstract class
```

## Implementations

| Class | Description |
|-------|-------------|
| [`Chess`](chess.md) | Standard chess position |
| [VariantPosition](variant.md) | Dynamically dispatched variant position |

## Methods

### Required (every concrete position implements these)

| Method | Returns | Description |
|--------|---------|-------------|
| `board()` | `\shakmaty\Board` | Piece positions on the board |
| `promoted()` | `\shakmaty\Bitboard` | Tracked promoted pieces (empty for standard chess) |
| `pockets()` | `\shakmaty\Pockets \| null` | Crazyhouse pockets, or `null` when the position has none |
| `remainingChecks(int $color)` | `\shakmaty\RemainingChecks \| null` | Remaining checks for a color (`1`=White, `0`=Black), or `null` |
| `zobristHash(?int $mode = null)` | `\shakmaty\Zobrist64` | 64-bit Zobrist hash of the position (excludes move counters) |
| `updateZobristHash(Zobrist64 $current, Move $m, ?int $mode = null)` | `\shakmaty\Zobrist64 \| null` | Incremental hash update after a move, or `null` if unsupported |
| `turn()` | `int` | Side to move: `1` = White, `0` = Black |
| `castlingRights()` | `\shakmaty\Bitboard` | Castling rights as a bitboard |
| `maybeEpSquare()` | `?int` | En passant target square after a double pawn push, unconditionally |
| `halfmoves()` | `int` | Half-move clock since the last capture or pawn move |
| `fullmoves()` | `int` | Move number (starts at `1`) |
| `legalMoves()` | `\shakmaty\MoveList` | All legal moves |
| `isVariantEnd()` | `bool` | Variant-specific end condition (always `false` for standard chess) |
| `hasInsufficientMaterial(int $color)` | `bool` | Whether a side has insufficient winning material |
| `variantOutcome()` | `string` | Variant outcome (`"*"` for standard chess) |
| `playUnchecked(Move $m)` | `void` | Plays a move without legality checks (mutates) |

### Provided (default behaviour from the trait)

| Method | Returns | Description |
|--------|---------|-------------|
| `us()` | `\shakmaty\Bitboard` | Squares occupied by the side to move |
| `our(int $role)` | `\shakmaty\Bitboard` | Squares occupied by a role of the side to move |
| `them()` | `\shakmaty\Bitboard` | Squares occupied by the opponent |
| `their(int $role)` | `\shakmaty\Bitboard` | Squares occupied by a role of the opponent |
| `checkers()` | `\shakmaty\Bitboard` | Pieces giving check |
| `isCheck()` | `bool` | King of the side to move is in check |
| `isCheckmate()` | `bool` | Checkmate |
| `isStalemate()` | `bool` | Stalemate |
| `isInsufficientMaterial()` | `bool` | Both sides have insufficient material |
| `isGameOver()` | `bool` | Game over (checkmate, stalemate, insufficient material, or variant end) |
| `outcome()` | `string` | `"1-0"`, `"0-1"`, `"1/2-1/2"`, or `"*"` |
| `epSquare(?int $mode = null)` | `?int` | En passant square (`0` = Legal (default), `1` = PseudoLegal, `2` = Always) |
| `pseudoLegalEpSquare()` | `?int` | En passant square for a pseudo-legal capture |
| `legalEpSquare()` | `?int` | En passant square for a legal capture |
| `captureMoves()` | `\shakmaty\MoveList` | Capture moves |
| `promotionMoves()` | `\shakmaty\MoveList` | Promotion moves |
| `enPassantMoves()` | `\shakmaty\MoveList` | En passant moves |
| `castlingMoves(int $side)` | `\shakmaty\MoveList` | Castling moves (`0` = king-side, `1` = queen-side) |
| `sanCandidates(int $role, int $to)` | `\shakmaty\MoveList` | SAN candidate moves |
| `kingAttackers(int $square, int $attacker, Bitboard $occupied)` | `\shakmaty\Bitboard` | Attackers of a king square |
| `isIrreversible(Move $m)` | `bool` | Whether a move is irreversible |
| `isLegal(Move $m)` | `bool` | Whether a move is legal |
| `play(Move $m)` | `\shakmaty\Chess` | Plays a legal move, returning a **new** position (original unchanged); throws if illegal |
| `toSetup(int $mode)` | `\shakmaty\Setup` | Converts the position to a `Setup` |

## Notes on default implementations

The Rust trait provides default bodies for the "Provided" methods above. Due
to an ext-php-rs limitation — concrete methods registered only on a
Rust-defined parent class are not dispatched on child instances — the defaults
are exposed by the concrete subclasses (e.g. `Chess`), which delegate to
`shakmaty::Position`. `Position` therefore declares the whole contract as
abstract methods, and each subclass re-exposes the defaults.

## Example

```php
$pos = new \shakmaty\Chess();

echo $pos->turn();                       // 1 (White)
echo $pos->us()->count();                // 16
echo $pos->legalMoves()->count();        // 20
var_dump($pos->isCheck());               // false
var_dump($pos->isGameOver());            // false
echo $pos->outcome();                    // "*"

$moves = $pos->legalMoves();
$e4 = $moves->get(0);
$next = $pos->play($e4);                 // new position; $pos unchanged
echo $next->turn();                      // 0 (Black)
```

## See also

- [`Chess`](chess.md) — the concrete standard-chess position
