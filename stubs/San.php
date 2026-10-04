<?php
namespace shakmaty;

/**
 * Wrapper for a Standard Algebraic Notation (SAN) string.
 *
 * @method string __toString() Returns the SAN string (PHP magic __toString).
 * @method bool isValid() Validates the SAN string.
 * @method static San fromMove(\shakmaty\Position $pos, Move $move) Builds SAN from a legal move in the given position, disambiguating the origin file/rank only as needed; throws if $pos is not a shakmaty\Position.
 * @method Move toMove(\shakmaty\Position $pos) Resolves this SAN to the unique legal move in $pos; throws "Invalid SAN", "Illegal SAN" or "Ambiguous SAN".
 * @method Move|null findMove(MoveList $moves) Returns the unique move in $moves matching this SAN, or null when none match, the match is ambiguous, or the string is not valid SAN.
 * @method bool matches(Move $move) Whether this SAN can match the given move in any position; false for an invalid stored string.
 */
final class San {
    public function __construct(string $san) {}
}
