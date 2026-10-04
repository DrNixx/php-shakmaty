<?php
namespace shakmaty;

/**
 * Dynamically dispatched chess variant position. Mirrors the Rust
 * `shakmaty::variant::VariantPosition` enum. Extends the abstract {@see Position}
 * and reports its concrete variant via {@see variant()}.
 *
 * @method static VariantPosition fromSetup(int $variant, Setup $setup, int|null $mode = null) Builds from a Setup; throws on invalid. mode: 0=Standard, 1=Chess960.
 * @method static VariantPosition fromFen(int $variant, string $fen, int|null $mode = null) Builds from FEN; throws on invalid.
 * @method Variant variant() The concrete variant of this position.
 * @method VariantPosition swapTurn() Swaps the side to move; throws if invalid.
 * @method string toFen() FEN of the current position.
 * @method int legalMovesCount() Number of legal moves.
 * @method bool playSan(string $san) Plays a SAN move (mutates); throws on invalid/illegal.
 * @method bool playUci(string $uci) Plays a UCI move (mutates); throws on invalid/illegal.
 */
class VariantPosition extends Position {
    public function __construct(?int $variant = null) {}
}
