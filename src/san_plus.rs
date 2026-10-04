use ext_php_rs::prelude::*;
use shakmaty::san::{San, SanPlus, Suffix};

use crate::san::PhpSan;

/// PHP class: `shakmaty\SanPlus`
///
/// A Standard Algebraic Notation (SAN) move together with its optional check
/// or checkmate suffix, as produced by the PGN reader.
///
/// # Usage
///
/// ```php
/// $move = \shakmaty\SanPlus::fromAscii('Qxf7#');
/// echo $move->__toString(); // "Qxf7#"
/// echo $move->san();       // "Qxf7"
/// echo $move->suffix();    // "#"
/// var_dump($move->isCheckmate()); // true
/// ```
///
/// # See Also
///
/// - [`PhpSan`](crate::san::PhpSan)
/// - Original Rust type: [`shakmaty::san::SanPlus`](https://docs.rs/shakmaty/0.30/shakmaty/san/struct.SanPlus.html)
#[php_class]
#[php(name = "shakmaty\\SanPlus")]
pub struct PhpSanPlus {
    pub inner: SanPlus,
}

#[php_impl]
impl PhpSanPlus {
    /// Parses a move written in SAN with an optional check/checkmate suffix.
    ///
    /// # Errors
    ///
    /// Throws a PHP exception if `text` is not syntactically valid SAN.
    pub fn from_ascii(text: String) -> Result<Self, String> {
        SanPlus::from_ascii(text.as_bytes())
            .map(|inner| PhpSanPlus { inner })
            .map_err(|_| "Invalid SAN".to_string())
    }

    /// Returns the full notation, including any suffix (e.g. `"Qxf7#"`).
    pub fn __to_string(&self) -> String {
        self.inner.to_string()
    }

    /// Returns only the SAN part, without the suffix (e.g. `"Qxf7"`).
    pub fn san(&self) -> String {
        self.inner.san.to_string()
    }

    /// Returns `"+"` for check, `"#"` for checkmate, or `null` otherwise.
    pub fn suffix(&self) -> Option<String> {
        self.inner.suffix.map(|suffix| match suffix {
            Suffix::Check => "+".to_string(),
            Suffix::Checkmate => "#".to_string(),
        })
    }

    /// Whether this token denotes a check.
    pub fn is_check(&self) -> bool {
        self.inner.suffix == Some(Suffix::Check)
    }

    /// Whether this token denotes a checkmate.
    pub fn is_checkmate(&self) -> bool {
        self.inner.suffix == Some(Suffix::Checkmate)
    }

    /// Whether this token is a null move (`"--"` / `"Z0"`).
    pub fn is_null(&self) -> bool {
        matches!(self.inner.san, San::Null)
    }

    /// Returns the SAN part wrapped in a `shakmaty\San` instance.
    pub fn to_san(&self) -> PhpSan {
        PhpSan::new(self.inner.san.to_string())
    }
}
