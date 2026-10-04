//! PHP bindings for the [`pgn-reader`](https://crates.io/crates/pgn-reader) crate.
//!
//! Exposes classes in the `shakmaty\pgn` namespace for parsing Portable Game
//! Notation (PGN) documents.

pub mod game;
pub mod nag;
pub mod reader;
pub mod token;
