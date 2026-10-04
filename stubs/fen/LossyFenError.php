<?php
namespace shakmaty\fen;

/**
 * Thrown when a shakmaty\Setup cannot be represented losslessly as a FEN.
 * The exception code is a bitmask of the reason constants (retrieve via getCode()).
 */
class LossyFenError extends \Exception {
    /** Set of squares with promoted Crazyhouse pieces does not match the board. */
    public const PROMOTED = 1;
    /** More than two castling rights per side, or castling rights off the backrank. */
    public const CASTLING_RIGHTS = 2;
    /** More than 64 Crazyhouse pocket pieces. */
    public const POCKETS = 4;
}
