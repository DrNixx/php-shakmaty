#![cfg_attr(windows, feature(abi_vectorcall))]
//! PHP binding for the [shakmaty](https://crates.io/crates/shakmaty) chess library.
//!
//! This crate provides PHP classes in the `shakmaty\` namespace that wrap
//! the Rust [shakmaty](https://docs.rs/shakmaty) chess library via
//! [ext-php-rs](https://ext-php.rs).
//!
//! # Classes
//!
//! | Module | PHP Class | Description |
//! |--------|-----------|-------------|
//! | `enums` | `shakmaty\Color`, `shakmaty\Role`, `shakmaty\CastlingSide`, `shakmaty\CastlingMode`, `shakmaty\EnPassantMode` | Fundamental enum types |
//! | `square` | `shakmaty\File`, `shakmaty\Rank`, `shakmaty\Square` | Board coordinates |
//! | `piece` | `shakmaty\Piece` | Colored chess piece |
//! | `bitboard` | `shakmaty\Bitboard` | 64-bit square set |
//! | `board` | `shakmaty\Board` | Piece positions |
//! | `setup` | `shakmaty\Setup` | Position setup |
//! | `move_` | `shakmaty\Move`, `shakmaty\MoveList` | Chess moves |
//! | `position` | `shakmaty\Position` | Abstract position base class |
//! | `chess_pos` | `shakmaty\Chess` | Main position class |
//! | `variant_position` | `shakmaty\\VariantPosition` | Dynamically dispatched variant position |
//! | `fen` | `shakmaty\Fen` | FEN notation |
//! | `fen_errors` | `shakmaty\fen\ParseFenError` and subclasses, `shakmaty\fen\LossyFenError` | FEN parse/representability exceptions |
//! | `san` | `shakmaty\San`, `shakmaty\SanPlus` | SAN notation |
//! | `uci` | `shakmaty\Uci` | UCI notation |
//! | `attacks` | `shakmaty\Attacks` | Attack and ray lookup tables |
//! | `remaining_checks` | `shakmaty\RemainingChecks` | Three-Check remaining checks value |
//! | `by_role` | `shakmaty\ByRole` | `ByRole<u8>` piece counts |
//! | `pockets` | `shakmaty\Pockets` | `ByColor<ByRole<u8>>` Crazyhouse pockets |
//! | `zobrist64` | `shakmaty\Zobrist64` | 64-bit Zobrist hash value |
//! | `variant` | `shakmaty\Variant` | Chess variant discriminant |
//! | `pgn` | `shakmaty\pgn\Reader`, `shakmaty\pgn\Game`, `shakmaty\pgn\Token`, `shakmaty\pgn\Nag` | Streaming PGN reader (tags, movetext tokens, comments, variations, NAGs) |
//!
//! For detailed documentation, see the [project docs](https://github.com/...).

use ext_php_rs::prelude::*;
use ext_php_rs::{info_table_end, info_table_row, info_table_start};
use ext_php_rs::zend::ModuleEntry;

mod enums;
mod square;
mod piece;
mod bitboard;
mod board;
mod setup;
mod move_;
    mod position;
    mod chess_pos;
    mod variant_position;
mod fen;
mod fen_errors;
mod san;
mod san_plus;
mod uci;
mod attacks;
mod variant;
mod remaining_checks;
mod by_role;
    mod pockets;
    mod zobrist64;
    mod pgn;

/// Registers all PHP classes with the Zend engine.
///
/// This function is called automatically when the PHP extension is loaded.
    /// It registers all 42 PHP classes in the `shakmaty\` namespace.
#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .name("shakmaty")
        .class::<enums::PhpColor>()
        .class::<enums::PhpRole>()
        .class::<enums::PhpCastlingSide>()
        .class::<enums::PhpCastlingMode>()
        .class::<enums::PhpEnPassantMode>()
        .class::<square::PhpFile>()
        .class::<square::PhpRank>()
        .class::<square::PhpSquare>()
        .class::<piece::PhpPiece>()
        .class::<bitboard::PhpBitboard>()
        .class::<board::PhpBoard>()
        .class::<setup::PhpSetup>()
        .class::<move_::PhpMove>()
        .class::<move_::PhpMoveList>()
        .class::<position::PhpPosition>()
        .class::<chess_pos::PhpChess>()
        .class::<variant_position::PhpVariantPosition>()
        .class::<fen::PhpFen>()
        .class::<fen_errors::PhpParseFenError>()
        .class::<fen_errors::PhpInvalidFen>()
        .class::<fen_errors::PhpInvalidBoard>()
        .class::<fen_errors::PhpInvalidPocket>()
        .class::<fen_errors::PhpInvalidTurn>()
        .class::<fen_errors::PhpInvalidCastling>()
        .class::<fen_errors::PhpInvalidEpSquare>()
        .class::<fen_errors::PhpInvalidRemainingChecks>()
        .class::<fen_errors::PhpInvalidHalfmoveClock>()
        .class::<fen_errors::PhpInvalidFullmoves>()
        .class::<fen_errors::PhpLossyFenError>()
        .class::<san::PhpSan>()
        .class::<uci::PhpUci>()
        .class::<attacks::PhpAttacks>()
        .class::<remaining_checks::PhpRemainingChecks>()
        .class::<variant::PhpVariant>()
        .class::<pgn::nag::PhpNag>()
        .class::<san_plus::PhpSanPlus>()
        .class::<pgn::token::PhpToken>()
        .class::<pgn::game::PhpGame>()
        .class::<by_role::PhpByRole>()
        .class::<pockets::PhpPockets>()
        .class::<pgn::reader::PhpReader>()
        .class::<zobrist64::PhpZobrist64>()
        .info_function(php_module_info)
    }

/// Used by the `phpinfo()` function and when you run `php -i`.
pub extern "C" fn php_module_info(_module: *mut ModuleEntry) {
    info_table_start!();
    info_table_row!("PGN reader", "enabled");
    info_table_end!();
}
