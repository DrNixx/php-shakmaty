<?php
namespace shakmaty;

/**
 * Represents a colored chess piece (color + role).
 *
 * @property-read Color $color Piece color (White or Black).
 * @property-read Role $role Piece role (Pawn through King).
 * @method string toChar() Returns character (K/Q/R/B/N/P/k/q/r/b/n/p).
 * @method string toUnicode() Returns Unicode chess symbol (♔♕♖♗♘♙♚♛♜♝♞♟).
 * @method self|null fromChar(string $ch) Creates piece from character.
 * @method static int legacyCode(Color $color, Role $role) Converts a Color+Role pair to the legacy 12-code (White 1..6, Black 9..14).
 * @method static self|null fromLegacyCode(int $code) Decodes a legacy 12-code into a Piece (null for NOPIECE=7 or invalid code).
 */
final class Piece {
    public function __construct(int $color, int $role) {}
}
