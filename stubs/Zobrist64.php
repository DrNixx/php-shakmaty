<?php
namespace shakmaty;

/**
 * A 64-bit Zobrist hash value. Mirrors `shakmaty::zobrist::Zobrist64`.
 *
 * PHP integers are signed 64-bit, so the read-only $value property returns the raw bit pattern as a
 * signed integer; use toHex()/__toString() for the exact unsigned value.
 *
 * @method static Zobrist64 fromHex(string $hex) Creates a Zobrist64 from a hex string (optional "0x"); throws on invalid.
 * @property-read int $value Raw 64-bit value as a signed PHP integer (may be negative).
 * @method string toHex() Unsigned 16-digit lowercase hexadecimal string.
 * @method string __toString() Alias of toHex() (PHP magic __toString).
 * @method bool isZero() Whether the value is zero.
 * @method Zobrist64 bitwiseXor(Zobrist64 $other) Bitwise XOR with another hash.
 * @method bool equals(Zobrist64 $other) Whether two hashes are equal.
 * @method int legacy24() Top 24 bits of the value ((value >>> 40) & 0xFFFFFF) — legacy compatibility with the old ZobristHash.php engine.
 */
class Zobrist64 {
    public function __construct(?int $value = null) {}
}
