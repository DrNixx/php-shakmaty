<?php
namespace shakmaty;

/**
 * Static-only utility class exposing attack and ray lookup tables.
 *
 * Squares are zero-based indices (A1 = 0 .. H8 = 63); out-of-range arguments
 * yield an empty Bitboard (or false) instead of throwing.
 *
 * @method static Bitboard pawnAttacks(int $color, int $sq) Attacks of a pawn of $color (0=Black, 1=White) on $sq.
 * @method static Bitboard knightAttacks(int $sq) Attacks of a knight on $sq.
 * @method static Bitboard kingAttacks(int $sq) Attacks of a king on $sq.
 * @method static Bitboard bishopAttacks(int $sq, Bitboard $occupied) Bishop attacks from $sq, blocked by $occupied.
 * @method static Bitboard rookAttacks(int $sq, Bitboard $occupied) Rook attacks from $sq, blocked by $occupied.
 * @method static Bitboard queenAttacks(int $sq, Bitboard $occupied) Queen attacks from $sq, blocked by $occupied.
 * @method static Bitboard attacks(int $sq, Piece $piece, Bitboard $occupied) Attacks of $piece on $sq, blocked by $occupied.
 * @method static Bitboard ray(int $a, int $b) The rank/file/diagonal containing both $a and $b.
 * @method static Bitboard between(int $a, int $b) Squares strictly between $a and $b.
 * @method static bool aligned(int $a, int $b, int $c) Whether all three squares lie on one rank/file/diagonal.
 */
final class Attacks {
    public function __construct() {}
}
