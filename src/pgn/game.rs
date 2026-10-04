use ext_php_rs::prelude::*;
use shakmaty::Outcome;

use crate::pgn::token::{PhpToken, TokenKind};

/// Internal representation of a parsed PGN game.
///
/// Populated by the PGN reader's visitor and exposed to PHP through
/// [`PhpGame`].
#[derive(Default)]
pub struct GameData {
    /// Tag pairs in source order as `(name, value)`.
    pub tags: Vec<(String, String)>,
    /// Movetext tokens in source order, including variation delimiters.
    pub movetext: Vec<TokenKind>,
    /// Game termination marker, if one was present in the movetext.
    pub outcome: Option<Outcome>,
    /// SAN strings of the mainline moves (variations excluded).
    pub mainline: Vec<String>,
    /// Current variation nesting depth (used while collecting).
    pub variation_depth: usize,
}

/// PHP class: `shakmaty\pgn\Game`
///
/// A single game parsed from a PGN document: its tag pairs, movetext tokens
/// and game termination marker.
///
/// # Usage
///
/// ```php
/// $reader = new \shakmaty\pgn\Reader($pgn);
/// $game = $reader->read_game();
///
/// echo $game->tag('White');           // "Deep Blue"
/// echo $game->outcome();              // "1-0"
/// echo implode(' ', $game->mainline()); // "e4 e5 Nf3"
/// foreach ($game->movetext() as $token) { /* ... */ }
/// ```
///
/// # See Also
///
/// - [`PhpReader`](crate::pgn::reader::PhpReader)
/// - [`PhpToken`](crate::pgn::token::PhpToken)
#[php_class]
#[php(name = "shakmaty\\pgn\\Game")]
pub struct PhpGame {
    pub inner: GameData,
}

#[php_impl]
impl PhpGame {
    /// Creates an empty game (mainly useful for testing).
    #[php(constructor)]
    pub fn new() -> Self {
        PhpGame {
            inner: GameData::default(),
        }
    }

    /// Returns all tag pairs as an associative array mapping name to value.
    ///
    /// If a name occurs more than once, the last value wins.
    pub fn tags(&self) -> Vec<(String, String)> {
        self.inner.tags.clone()
    }

    /// Returns the value of the first tag with the given name, or `null`.
    pub fn tag(&self, name: String) -> Option<String> {
        self.inner
            .tags
            .iter()
            .find(|(key, _)| *key == name)
            .map(|(_, value)| value.clone())
    }

    /// Returns the movetext tokens in source order.
    pub fn movetext(&self) -> Vec<PhpToken> {
        self.inner
            .movetext
            .iter()
            .cloned()
            .map(|inner| PhpToken { inner })
            .collect()
    }

    /// Returns the game outcome: `"1-0"`, `"0-1"`, `"1/2-1/2"` or `"*"` if the
    /// game has no termination marker.
    pub fn outcome(&self) -> String {
        match self.inner.outcome {
            Some(outcome) => outcome.as_str().to_string(),
            None => "*".to_string(),
        }
    }

    /// Returns the SAN strings of the mainline moves (variations excluded).
    pub fn mainline(&self) -> Vec<String> {
        self.inner.mainline.clone()
    }
}
