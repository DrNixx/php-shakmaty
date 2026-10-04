<?php
namespace shakmaty\pgn;

/**
 * A streaming reader over a PGN document containing one or more games.
 * The full text is supplied to the constructor and parsing is lazy — nothing happens until you call a reading method. Games can be read one at a time (hasMore() / readGame()) or all at once (readAll()). Irrecoverable parse errors throw PHP exceptions; end of input yields null/false instead.
 *
 * @method void setSupportedTagLineLength(int $bytes) Configures the buffer to support tag lines AT LEAST as long as `$bytes`; values below 255 are clamped up to 255 (the default). Raise this when parsing documents with very long [Tag "value"] pairs.
 * @method void setSupportedCommentLength(int $bytes) Same idea for `{ ... }` comments; oversized chunks from the underlying reader are merged by this binding into a single comment token, so callers always see one logical comment per block. Values below 255 are clamped up to 255 (the default).
 * @method Game|null readGame() Reads and returns the next game in document order; null when the document is exhausted. Advances offset(). Throws on irrecoverable parse errors — wrap untrusted input accordingly.
 * @method bool hasMore() Whether another game follows in the document, without parsing it fully (cheap lookahead). false at end of input.
 * @method bool skipGame() Skips past the next game so you can jump ahead cheaply; true if a game was skipped, false at end of input — useful when only later games matter and materializing Game objects is wasteful.
 * @method array readAll() Reads all remaining games into an array (equivalent to looping hasMore()/readGame()); returns [] once the document is exhausted.
 * @method int offset() Current byte position within the source text after the last completed operation — useful for resuming or reporting progress on large files; 0 right after construction and reset().
 * @method void reset() Rewinds back to the beginning of the document (offset becomes 0) so it can be read again from scratch.
 */
final class Reader {

    /** @param string $pgn The complete PGN text to parse — may contain multiple games separated by blank lines; parsing is deferred until a reading method is called. */
    public function __construct(string $pgn) {}
}
