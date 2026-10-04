<?php
namespace shakmaty\pgn;

/**
 * A single element of PGN movetext: a SAN move, NAG annotation, comment, variation delimiter or outcome marker.
 * No public constructor — tokens are created only while the Reader parses a game and exposed through Game::movetext().
 * Use kind() to discriminate a token; accessors that do not apply return null rather than throwing, so they can be called unconditionally if you prefer an early-return style over switching on kind().
 *
 * @method string kind() One of "san", "nag", "comment", "variation_begin", "variation_end" or "outcome".
 * @method \shakmaty\SanPlus|null san() The SAN move (with suffix) if this token is a move, otherwise null.
 * @method Nag|null nag() The NAG annotation glyph if this token carries one, otherwise null.
 * @method string|null comment() The `{ ... }` comment text for that position in the movetext; oversized comments are already merged into one token by the reader — or null when not applicable.
 * @method string|null outcome() One of `"1-0"`, `"0-1"` or `"1/2-1/2"` (or `*` for an unknown result) when the token is a game termination marker; otherwise null.
 * @method string __toString() A printable representation: the SAN move for moves, `$n` form for NAGs, raw text for comments, literal parentheses ("(" / ")") for variation delimiters and the outcome string itself — handy for logging or re-serializing a movetext stream without inspecting each token's type first (PHP magic __toString).
 */
final class Token {
}
