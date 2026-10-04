use ext_php_rs::prelude::*;
use shakmaty::{ByRole, Role};

/// PHP class: `shakmaty\ByRole`
///
/// Container holding a `u8` count for each piece role (Pawn..King). Mirrors
/// `shakmaty::ByRole<u8>`. Used as the per-color half of Crazyhouse pockets.
///
/// # Usage
///
/// ```php
/// $counts = new \shakmaty\ByRole();
/// $counts->set(\shakmaty\Role::KNIGHT, 2);
/// echo $counts->knight;   // 2
/// echo $counts->total();    // 2
/// ```
///
/// # See Also
///
/// Original Rust type: [`shakmaty::ByRole`](https://docs.rs/shakmaty/0.30/shakmaty/struct.ByRole.html)
#[php_class]
#[php(name = "shakmaty\\ByRole")]
pub struct PhpByRole {
    pub inner: ByRole<u8>,
}

fn role_of(value: i32) -> Option<Role> {
    Role::try_from(value as u8).ok()
}

#[php_impl]
impl PhpByRole {
    /// Creates a container with a zero count for every role.
    #[php(constructor)]
    pub fn new() -> Self {
        PhpByRole { inner: ByRole::default() }
    }

    /// Returns the count for `role` (`1` = Pawn … `6` = King), or `0` for an invalid role.
    pub fn get(&self, role: i32) -> i32 {
        match role_of(role) {
            Some(r) => i32::from(*self.inner.get(r)),
            None => 0,
        }
    }

    /// Sets the count for `role` (clamped to `0..=255`). Invalid roles are ignored.
    pub fn set(&mut self, role: i32, count: i32) {
        if let Some(r) = role_of(role) {
            *self.inner.get_mut(r) = count.clamp(0, 255) as u8;
        }
    }

    /// Count of pawns.
    #[php(getter)]
    pub fn get_pawn(&self) -> i32 {
        i32::from(self.inner.pawn)
    }

    /// Count of knights.
    #[php(getter)]
    pub fn get_knight(&self) -> i32 {
        i32::from(self.inner.knight)
    }

    /// Count of bishops.
    #[php(getter)]
    pub fn get_bishop(&self) -> i32 {
        i32::from(self.inner.bishop)
    }

    /// Count of rooks.
    #[php(getter)]
    pub fn get_rook(&self) -> i32 {
        i32::from(self.inner.rook)
    }

    /// Count of queens.
    #[php(getter)]
    pub fn get_queen(&self) -> i32 {
        i32::from(self.inner.queen)
    }

    /// Count of kings.
    #[php(getter)]
    pub fn get_king(&self) -> i32 {
        i32::from(self.inner.king)
    }

    /// Sum of all counts.
    pub fn total(&self) -> i32 {
        Role::ALL.iter().map(|r| i32::from(*self.inner.get(*r))).sum()
    }

    /// Whether every count is zero.
    pub fn is_empty(&self) -> bool {
        self.total() == 0
    }

    /// Returns the six counts as a list `[pawn, knight, bishop, rook, queen, king]`.
    pub fn to_array(&self) -> Vec<i32> {
        Role::ALL.iter().map(|r| i32::from(*self.inner.get(*r))).collect()
    }

    /// Returns an independent copy of this container.
    pub fn copy(&self) -> Self {
        PhpByRole { inner: self.inner }
    }
}
