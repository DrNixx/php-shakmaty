<?php
namespace shakmaty;

/**
 * Chess variant discriminant. Mirrors the Rust `shakmaty::variant::Variant` enum.
 *
 * @method static Variant|null fromUci(string $name) Selects a variant by its exact UCI name; null if unknown.
 * @method static Variant|null fromAscii(string $name) Selects a variant by name or alias; null if unknown.
 * @method static Variant[] all() Returns all eight variants.
 * @method int value() Numeric discriminant (0 = Chess … 7 = Horde).
 * @method string uci() Canonical UCI name (e.g. "atomic", "3check").
 * @method string __toString() Alias of uci() (PHP magic __toString).
 * @method bool distinguishesPromoted() Whether the variant tracks promoted pieces (only Crazyhouse).
 */
class Variant {
    public const CHESS = 0;
    public const ATOMIC = 1;
    public const ANTICHESS = 2;
    public const KING_OF_THE_HILL = 3;
    public const THREE_CHECK = 4;
    public const CRAZYHOUSE = 5;
    public const RACING_KINGS = 6;
    public const HORDE = 7;

    public function __construct(int $value) {}
}
