use ext_php_rs::prelude::*;
use shakmaty::{ByColor, ByRole, Color, Role};
use crate::by_role::PhpByRole;

fn color_of(value: i32) -> Color {
    if value == 1 { Color::White } else { Color::Black }
}

/// PHP class: `shakmaty\Pockets`
///
/// Crazyhouse pockets: a per-color container of per-role piece counts. Mirrors
/// `shakmaty::ByColor<shakmaty::ByRole<u8>>`.
///
/// # Usage
///
/// ```php
/// $pockets = new \shakmaty\Pockets();
/// $pockets->setCount(\shakmaty\Color::WHITE, \shakmaty\Role::PAWN, 1);
/// echo $pockets->count(\shakmaty\Color::WHITE, \shakmaty\Role::PAWN); // 1
/// echo $pockets->total(); // 1
/// ```
///
/// # See Also
///
/// Original Rust type: [`shakmaty::ByColor`](https://docs.rs/shakmaty/0.30/shakmaty/struct.ByColor.html)
#[php_class]
#[php(name = "shakmaty\\Pockets")]
pub struct PhpPockets {
    pub inner: ByColor<ByRole<u8>>,
}

#[php_impl]
impl PhpPockets {
    /// Creates empty pockets (zero pieces for both colors).
    #[php(constructor)]
    pub fn new() -> Self {
        PhpPockets { inner: ByColor::default() }
    }

    /// Creates empty pockets (alias of the constructor).
    pub fn empty() -> Self {
        PhpPockets { inner: ByColor::default() }
    }

    /// Per-role counts for White.
    #[php(getter)]
    pub fn get_white(&self) -> PhpByRole {
        PhpByRole { inner: self.inner.white }
    }

    /// Per-role counts for Black.
    #[php(getter)]
    pub fn get_black(&self) -> PhpByRole {
        PhpByRole { inner: self.inner.black }
    }

    /// Returns the per-role counts for `color` (`1` = White, `0` = Black).
    pub fn get(&self, color: i32) -> PhpByRole {
        PhpByRole { inner: *self.inner.get(color_of(color)) }
    }

    /// Replaces the per-role counts for `color`.
    pub fn set(&mut self, color: i32, counts: &PhpByRole) {
        *self.inner.get_mut(color_of(color)) = counts.inner;
    }

    /// Returns the count of `role` held by `color`, or `0` for an invalid role.
    pub fn count(&self, color: i32, role: i32) -> i32 {
        match Role::try_from(role as u8) {
            Ok(r) => i32::from(*self.inner.get(color_of(color)).get(r)),
            Err(_) => 0,
        }
    }

    /// Sets the count of `role` held by `color` (clamped to `0..=255`). Invalid roles are ignored.
    pub fn set_count(&mut self, color: i32, role: i32, count: i32) {
        if let Ok(r) = Role::try_from(role as u8) {
            *self.inner.get_mut(color_of(color)).get_mut(r) = count.clamp(0, 255) as u8;
        }
    }

    /// Total number of pieces held in both pockets.
    pub fn total(&self) -> i32 {
        self.inner.black.iter().map(|c| i32::from(*c)).sum::<i32>()
            + self.inner.white.iter().map(|c| i32::from(*c)).sum::<i32>()
    }

    /// Whether both pockets are empty.
    pub fn is_empty(&self) -> bool {
        self.total() == 0
    }

    /// Returns an independent copy of these pockets.
    pub fn copy(&self) -> Self {
        PhpPockets { inner: self.inner }
    }
}
