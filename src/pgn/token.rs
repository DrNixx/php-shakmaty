use ext_php_rs::prelude::*;
use pgn_reader::Nag;
use shakmaty::san::SanPlus;
use shakmaty::Outcome;

use crate::pgn::nag::PhpNag;
use crate::san_plus::PhpSanPlus;

/// Internal representation of a single PGN movetext token.
///
/// The PGN reader reports movetext as a flat sequence of tokens; this enum
/// preserves that order, including variation delimiters.
#[derive(Clone)]
pub enum TokenKind {
    San(SanPlus),
    Nag(Nag),
    Comment(String),
    VariationBegin,
    VariationEnd,
    Outcome(Outcome),
}

/// PHP class: `shakmaty\pgn\Token`
///
/// A single element of PGN movetext, produced by
/// [`Reader`](crate::pgn::reader::PhpReader) and exposed through
/// [`Game::movetext()`](crate::pgn::game::PhpGame::movetext).
///
/// Use [`Token::kind()`](#method.kind) to discriminate and the corresponding
/// accessor (`san()`, `nag()`, `comment()`, `outcome()`) to read the payload.
///
/// # Usage
///
/// ```php
/// foreach ($game->movetext() as $token) {
///     if ($token->kind() === 'san') {
///         echo $token->san()->__toString();
///     }
/// }
/// ```
#[php_class]
#[php(name = "shakmaty\\pgn\\Token")]
pub struct PhpToken {
    pub inner: TokenKind,
}

#[php_impl]
impl PhpToken {
    /// Returns the token kind: `"san"`, `"nag"`, `"comment"`,
    /// `"variation_begin"`, `"variation_end"` or `"outcome"`.
    pub fn kind(&self) -> String {
        match &self.inner {
            TokenKind::San(_) => "san",
            TokenKind::Nag(_) => "nag",
            TokenKind::Comment(_) => "comment",
            TokenKind::VariationBegin => "variation_begin",
            TokenKind::VariationEnd => "variation_end",
            TokenKind::Outcome(_) => "outcome",
        }
        .to_string()
    }

    /// Returns the SAN move if this token is a move, otherwise `null`.
    pub fn san(&self) -> Option<PhpSanPlus> {
        match &self.inner {
            TokenKind::San(san_plus) => Some(PhpSanPlus { inner: *san_plus }),
            _ => None,
        }
    }

    /// Returns the NAG if this token is an annotation glyph, otherwise `null`.
    pub fn nag(&self) -> Option<PhpNag> {
        match &self.inner {
            TokenKind::Nag(nag) => Some(PhpNag { inner: *nag }),
            _ => None,
        }
    }

    /// Returns the comment text if this token is a comment, otherwise `null`.
    pub fn comment(&self) -> Option<String> {
        match &self.inner {
            TokenKind::Comment(text) => Some(text.clone()),
            _ => None,
        }
    }

    /// Returns the outcome (`"1-0"`, `"0-1"`, `"1/2-1/2"`, `"*"`) if this
    /// token is an outcome, otherwise `null`.
    pub fn outcome(&self) -> Option<String> {
        match &self.inner {
            TokenKind::Outcome(outcome) => Some(outcome.as_str().to_string()),
            _ => None,
        }
    }

    /// Returns a printable representation of the token.
    pub fn __to_string(&self) -> String {
        match &self.inner {
            TokenKind::San(san_plus) => san_plus.to_string(),
            TokenKind::Nag(nag) => nag.to_string(),
            TokenKind::Comment(text) => text.clone(),
            TokenKind::VariationBegin => "(".to_string(),
            TokenKind::VariationEnd => ")".to_string(),
            TokenKind::Outcome(outcome) => outcome.as_str().to_string(),
        }
    }
}
