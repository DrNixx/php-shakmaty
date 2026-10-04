# shakmaty Documentation

Welcome to the **shakmaty** documentation — a PHP extension that provides bindings to the [shakmaty](https://crates.io/crates/shakmaty) chess library (v0.30) via [ext-php-rs](https://ext-php.rs).

## Getting Started

- [Getting Started](getting-started.md) — installation, loading the extension, basic usage

## API Reference

### Value Types / Enums
- [Color, Role, CastlingSide, CastlingMode, EnPassantMode](api/enums.md) — fundamental enum types
- [File, Rank, Square](api/square.md) — board coordinates
- [Piece](api/piece.md) — a chess piece (color + role)

### Board Logic
- [Bitboard](api/bitboard.md) — 64-bit set of squares with bitwise operations
- [Board](api/board.md) — piece positions on the board
- [Setup](api/setup.md) — a not necessarily legal position setup
- [Move, MoveList](api/move.md) — chess move representation and collections
- [Position (abstract)](api/position.md) — abstract base class for chess positions
- [Chess](api/chess.md) — main chess position class with move execution
- [Variant, VariantPosition](api/variant.md) — chess variant discriminant and dynamically dispatched variant position
- [Pockets, RemainingChecks, ByRole](api/pockets.md) — Crazyhouse pockets and Three-Check counters
- [Zobrist64](api/zobrist.md) — 64-bit Zobrist position hashing
- [Attacks](api/attacks.md) — attack and ray lookup tables

### Notation
- [Fen](api/fen.md) — parsed FEN/EPD value object with static factories and exceptions
- [San, SanPlus](api/san.md) — SAN wrapper and SAN move with check/checkmate suffix
- [Uci](api/uci.md) — Universal Chess Interface notation wrapper
- [Reader, Game, Token, Nag](api/pgn.md) — streaming Portable Game Notation (PGN) parser

## Examples

- [Scholar's Mate](examples/scholar-mate.md) — complete walkthrough of the Scholar's Mate
- [Perft](examples/perft.md) — move count performance test
- [PGN Parsing](examples/pgn.md) — reading games, tags, movetext tokens and comments

## Project

- [README](../README.md) — project overview, installation, quick start
- [License](../README.md#license) — GPL-3.0-or-later
