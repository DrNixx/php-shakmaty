use ext_php_rs::prelude::*;
use pgn_reader::Nag;

/// PHP class: `shakmaty\pgn\Nag`
///
/// A Numeric Annotation Glyph (NAG), such as `!`, `??` or `$42`, attached to a
/// move in PGN movetext.
///
/// # Constants
///
/// - `Nag::GOOD_MOVE` = 1 (`!`)
/// - `Nag::MISTAKE` = 2 (`?`)
/// - `Nag::BRILLIANT_MOVE` = 3 (`!!`)
/// - `Nag::BLUNDER` = 4 (`??`)
/// - `Nag::SPECULATIVE_MOVE` = 5 (`!?`)
/// - `Nag::DUBIOUS_MOVE` = 6 (`?!`)
///
/// # Usage
///
/// ```php
/// $nag = new \shakmaty\pgn\Nag(\shakmaty\pgn\Nag::BLUNDER);
/// echo $nag->value;     // 4
/// echo $nag->__toString(); // "$4"
/// echo $nag->glyph();     // "??"
///
/// $parsed = \shakmaty\pgn\Nag::from_ascii('$24');
/// echo $parsed->value;  // 24
/// ```
///
/// # Properties
///
/// - `$nag->value` (`int`, read-only) — numeric NAG value in `0..=255`.
///
/// # See Also
///
/// Original Rust type: [`pgn_reader::Nag`](https://docs.rs/pgn-reader/0.29/pgn_reader/struct.Nag.html)
#[php_class]
#[php(name = "shakmaty\\pgn\\Nag")]
pub struct PhpNag {
    pub inner: Nag,
}

#[php_impl]
impl PhpNag {
    /// Creates a `Nag` from a numeric value.
    ///
    /// Values outside `0..=255` are clamped.
    #[php(constructor)]
    pub fn new(value: i32) -> Self {
        PhpNag { inner: Nag(value.clamp(0, 255) as u8) }
    }

    /// Numeric NAG value (`0`..`255`).
    ///
    /// Backs the read-only PHP property `$value`.
    #[php(getter)]
    pub fn get_value(&self) -> i32 {
        i32::from(self.inner.0)
    }

    /// Returns the NAG in numeric form, e.g. `"$42"`.
    pub fn __to_string(&self) -> String {
        self.inner.to_string()
    }

    /// Returns the symbolic glyph for well-known NAGs (`"!"`, `"??"`, ...),
    /// or `null` for purely numeric annotations.
    pub fn glyph(&self) -> Option<String> {
        let glyph = match self.inner.0 {
            1 => "!",
            2 => "?",
            3 => "!!",
            4 => "??",
            5 => "!?",
            6 => "?!",
            _ => return None,
        };
        Some(glyph.to_string())
    }

    /// Parses a NAG from its ASCII representation (`"??"`, `"$24"`, ...).
    ///
    /// Returns `null` if the input is not a valid NAG.
    pub fn from_ascii(text: String) -> Option<Self> {
        Nag::from_ascii(text.as_bytes()).ok().map(|inner| PhpNag { inner })
    }

    pub const GOOD_MOVE: i32 = 1;
    pub const MISTAKE: i32 = 2;
    pub const BRILLIANT_MOVE: i32 = 3;
    pub const BLUNDER: i32 = 4;
    pub const SPECULATIVE_MOVE: i32 = 5;
    pub const DUBIOUS_MOVE: i32 = 6;
}
