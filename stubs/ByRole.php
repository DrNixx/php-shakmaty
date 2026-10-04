<?php
namespace shakmaty;

/**
 * Container holding an integer count for each piece role. Mirrors
 * `shakmaty::ByRole<u8>`. Used as the per-color half of Crazyhouse pockets.
 *
 * @method int get(int $role) Count for a role (1=Pawn..6=King); 0 for an invalid role.
 * @method void set(int $role, int $count) Sets the count for a role (clamped 0..255).
 * @property-read int $pawn Count of pawns.
 * @property-read int $knight Count of knights.
 * @property-read int $bishop Count of bishops.
 * @property-read int $rook Count of rooks.
 * @property-read int $queen Count of queens.
 * @property-read int $king Count of kings.
 * @method int total() Sum of all counts.
 * @method bool isEmpty() Whether every count is zero.
 * @method int[] toArray() Counts as [pawn, knight, bishop, rook, queen, king].
 * @method ByRole copy() Independent copy.
 */
class ByRole {
    public function __construct() {}
}
