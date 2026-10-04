<?php
namespace shakmaty;

/**
 * Parsed Forsyth-Edwards Notation (FEN/EPD) value object.
 *
 * Instances are created via the static factories; the public constructor has
 * been removed (it is private).
 *
 * @method static Fen empty() Returns the empty FEN (8/8/8/8/8/8/8/8 w - - 0 1). Useful as a neutral starting value without parsing a string.
 * @method static Fen parse(string $fen) Parses a FEN/EPD string. Throws a shakmaty\fen\ParseFenError subclass on invalid input.
 * @method static Fen fromPosition(Position $pos, ?int $mode = null) Builds a FEN from a position (Chess or VariantPosition). Mode: 0=Legal (default), 1=PseudoLegal, 2=Always. Throws when $pos is not a Position.
 * @method static Fen fromSetup(Setup $setup) Builds a FEN from a Setup. Throws shakmaty\fen\LossyFenError if the setup cannot be represented losslessly.
 * @method static bool isValid(string $fen) Validates whether $fen is a syntactically valid FEN/EPD string (never throws).
 * @method Setup getSetup() Returns the FEN contents as a Setup.
 * @method VariantPosition getPosition(?int $variant = null, ?int $mode = null) Converts to a VariantPosition. Variant: 0=Chess (default) ... 7=Horde. Mode: 0=Standard (default), 1=Chess960. Throws on invalid position.
 */
final class Fen {
    private function __construct() {}

    /** Returns the (normalized) FEN string (PHP magic __toString). */
    public function __toString(): string {}
}
