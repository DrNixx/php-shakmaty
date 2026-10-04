use ext_php_rs::prelude::*;
use shakmaty::RemainingChecks;

/// PHP class: `shakmaty\RemainingChecks`
///
/// Number of checks a side still needs to give in order to win a game of
/// Three-Check. Mirrors the Rust [`shakmaty::RemainingChecks`] value type
/// (an integer in `0..=3`, default `3`).
///
/// # Usage
///
/// ```php
/// $checks = new \shakmaty\RemainingChecks(3);
/// echo $checks->value();                    // 3
/// echo $checks->saturatingSub(1)->value();  // 2
/// var_dump($checks->isZero());              // false
/// ```
///
/// # See Also
///
/// Original Rust type: [`shakmaty::RemainingChecks`](https://docs.rs/shakmaty/0.30/shakmaty/struct.RemainingChecks.html)
#[php_class]
#[php(name = "shakmaty\\RemainingChecks")]
pub struct PhpRemainingChecks {
    pub inner: RemainingChecks,
}

#[php_impl]
impl PhpRemainingChecks {
    /// Creates a `RemainingChecks` value.
    ///
    /// Accepts an integer; values are clamped into `0..=3`. Defaults to `3`.
    #[php(constructor)]
    pub fn new(value: Option<i32>) -> Self {
        let n = value.unwrap_or(PhpRemainingChecks::MAX).clamp(0, PhpRemainingChecks::MAX) as u32;
        PhpRemainingChecks {
            inner: RemainingChecks::new(n),
        }
    }

    /// Returns the number of remaining checks (`0..=3`).
    pub fn value(&self) -> i32 {
        u32::from(self.inner) as i32
    }

    /// Whether the side has no checks left.
    pub fn is_zero(&self) -> bool {
        self.inner.is_zero()
    }

    /// Returns a copy with `n` checks subtracted (saturating at zero).
    pub fn saturating_sub(&self, n: i32) -> Self {
        PhpRemainingChecks {
            inner: self.inner.saturating_sub(n.max(0) as u32),
        }
    }

    /// Maximum possible value (`3`).
    pub const MAX: i32 = 3;
}
