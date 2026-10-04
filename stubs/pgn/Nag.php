<?php
namespace shakmaty\pgn;

/**
 * A Numeric Annotation Glyph (NAG) — a `$1`..`$255` annotation marking move quality in PGN movetext.
 * Values outside 0..=255 are clamped at construction time, not rejected.
 *
 * @property-read int $value Returns the numeric NAG value (`0`..=`255`).
 * @method string __toString() Returns the NAG in its PGN text form, e.g. `"$4"` (PHP magic __toString).
 * @method string|null glyph() Symbolic glyph for well-known values 1-6 ("!", "?", "!!", "??", "!?", "?!"), or null otherwise (e.g. "$42").
 * @method static Nag|null fromAscii(string $text) Parses a NAG from symbolic form (`"??"`) or PGN text form (`"$24"`); returns null on unrecognized input instead of throwing.
 */
final class Nag {
    const GOOD_MOVE = 1;        // "!" — good move (positive annotation)
    const MISTAKE = 2;          // "?" — weaker than the best available line
    const BRILLIANT_MOVE = 3;   // "!!" — brilliant move
    const BLUNDER = 4;          // "??" — serious mistake losing material or the game
    const SPECULATIVE_MOVE = 5; // "!?" — risky but may have a point
    const DUBIOUS_MOVE = 6;     // "?!" — dubious/questionable choice

    /** @param int $value Numeric NAG code (clamped to 0..=255). */
    public function __construct(int $value) {}
}
