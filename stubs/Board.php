<?php
namespace shakmaty;

/**
 * Represents piece positions on a chessboard.
 *
 * @method static self empty() Creates an empty board.
 * @method string|null pieceAt(int $sq) Returns piece character at square or null.
 * @method Bitboard occupied() Returns bitboard of all occupied squares.
 * @method Bitboard byColor(int $colorValue) Returns bitboard of pieces of given color.
 * @method Bitboard byRole(int $roleValue) Returns bitboard of pieces of given role.
 * @method int|null roleAt(int $sq) Returns role value at square or null.
 * @method int|null colorAt(int $sq) Returns color value at square or null.
 */
final class Board {
    public function __construct() {}
}
