use ext_php_rs::prelude::*;

/// PHP class: `shakmaty\Uci`
///
/// A wrapper around a Universal Chess Interface (UCI) move string.
/// UCI encodes moves as `from_square + to_square + optional promotion` (e.g., "e2e4", "e7e8q").
///
/// # Usage
///
/// ```php
/// $uci = new \shakmaty\Uci("e2e4");
/// var_dump($uci->isValid()); // true
/// echo $uci->__toString(); // "e2e4"
///
/// $promo = new \shakmaty\Uci("e7e8q");
/// var_dump($promo->isValid()); // true
/// ```
///
/// # See Also
///
/// - [`Chess::playUci()`](Chess)
/// - Original Rust type: [`shakmaty::uci::UciMove`](https://docs.rs/shakmaty/0.30/shakmaty/uci/struct.UciMove.html)
#[php_class]
#[php(name = "shakmaty\\Uci")]
pub struct PhpUci {
    inner: String,
}

#[php_impl]
impl PhpUci {
    /// Creates a new UCI move wrapper. Does not validate immediately.
    #[php(constructor)]
    pub fn new(uci: String) -> Self {
        PhpUci { inner: uci }
    }

    /// Returns the underlying UCI string.
    pub fn __to_string(&self) -> String {
        self.inner.clone()
    }

    /// Validates the UCI move string using the shakmaty parser.
    /// Returns `false` for strings that are not valid moves (e.g., "xyz").
    pub fn is_valid(&self) -> bool {
        shakmaty::uci::UciMove::from_ascii(self.inner.as_bytes()).is_ok()
    }
}
