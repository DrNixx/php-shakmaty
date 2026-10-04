<?php
namespace shakmaty;

/**
 * Crazyhouse pockets: a per-color container of per-role piece counts.
 * Mirrors `shakmaty::ByColor<shakmaty::ByRole<u8>>`.
 *
 * @method static Pockets empty() Empty pockets (alias of the constructor).
 * @property-read ByRole $white Per-role counts for White.
 * @property-read ByRole $black Per-role counts for Black.
 * @method ByRole get(int $color) Per-role counts for a color (1=White, 0=Black).
 * @method void set(int $color, ByRole $counts) Replaces the counts for a color.
 * @method int count(int $color, int $role) Count of a role for a color; 0 for an invalid role.
 * @method void setCount(int $color, int $role, int $count) Sets a count (clamped 0..255).
 * @method int total() Total pieces in both pockets.
 * @method bool isEmpty() Whether both pockets are empty.
 * @method Pockets copy() Independent copy.
 */
class Pockets {
    public function __construct() {}
}
