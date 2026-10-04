<?php
namespace shakmaty\pgn;

/**
 * A single game parsed from a PGN document: its tag pairs, movetext tokens and outcome marker.
 * Instances are normally produced by Reader::readGame() / Reader::readAll(); the public constructor creates an empty (tag-less, move-less) game — mainly useful for testing.
 *
 * @method array tags() All tag pairs as an associative name => value map; if a name occurs more than once, the last occurrence wins in this map. Use tag($name) when you need first-occurrence semantics instead.
 * @method string|null tag(string $name) Returns the value of the FIRST tag with the given name (e.g. "White", "Result"), or null if no such tag exists; names are case-sensitive as written in the source document.
 * @method Token[] movetext() The movetext tokens in source order, including variation delimiters ("variation_begin" / "variation_end") and outcome markers — iterate with a loop over $token->kind() to dispatch on token type.
 * @method string outcome() Game termination marker: one of `"1-0"`, `"0-1"` or `"1/2-1/2"`; returns `*` when the movetext contains no explicit result (or an unknown-result marker). Independent of what the [Result ...] tag says — tags are not cross-checked against the movetext.
 * @method array mainline() SAN strings of the mainline moves only; any move inside a `( ... )` variation is excluded — handy for replaying or indexing games without walking the token stream.
 */
final class Game {

    /** Creates an empty game (no tags, no moves) — mainly useful for testing. */
    public function __construct() {}
}
