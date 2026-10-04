<?php
namespace shakmaty;

/**
 * A SAN move together with its optional check/checkmate suffix (`+`, `#`), as produced by the PGN reader.
 * Lives in the root namespace (moved out of shakmaty\pgn). No public constructor — create instances via SanPlus::fromAscii() or read them from \shakmaty\pgn\Token::san().
 *
 * @method static SanPlus fromAscii(string $text) Parses a SAN move with an optional `+`/`#` suffix (e.g. "Qxf7#", "Nf3+"); throws on invalid SAN instead of returning null.
 * @method string __toString() Returns the full notation including any suffix, e.g. `"Qxf7#"` (PHP magic __toString).
 * @method string san() Returns only the SAN part without the check/checkmate suffix (e.g. `"Qxf7"`).
 * @method string|null suffix() The trailing marker: `+` for a checking move, `#` for checkmate, or null when absent.
 * @method bool isCheck() Whether this token denotes a checking move (`suffix()` === "+"). Always false for checkmates — use isCheckmate() separately if you want to treat both as "gives check".
 * @method bool isCheckmate() Whether this token ends the game with checkmate (`suffix()` === "#"); mutually exclusive with isCheck().
 * @method bool isNull() Whether this token is a null move ("--" / "Z0") — an explicit no-move placeholder; both boolean helpers above are false in that case.
 * @method \shakmaty\San toSan() Returns the SAN part wrapped in a shakmaty\San instance (suffix dropped) for use with other base-namespace APIs such as Chess::playSan().
 */
final class SanPlus {
}
