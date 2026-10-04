<?php
namespace shakmaty;

/**
 * Represents a chessboard file (column) from A (0) to H (7).
 *
 * @method int value() Returns the file index (0..7).
 * @method string toChar() Returns lowercase letter (a..h).
 * @method string upperChar() Returns uppercase letter (A..H).
 * @method int distance(self $other) Returns absolute file distance.
 * @method self flipHorizontal() Returns the horizontally mirrored file.
 * @method self|null fromChar(string $ch) Creates from character (a-h or A-H).
 */
final class File {
    const int A = 0; const int B = 1; const int C = 2; const int D = 3;
    const int E = 4; const int F = 5; const int G = 6; const int H = 7;
    public function __construct(int $value) {}
}
