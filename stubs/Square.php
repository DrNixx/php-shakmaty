<?php
namespace shakmaty;

/**
 * Represents a square on a chessboard (0..63, A1..H8).
 *
 * @method int value() Returns the square index (0..63).
 * @method self fromCoords(File $file, Rank $rank) Creates square from file and rank.
 * @method File file() Returns the file of this square.
 * @method Rank rank() Returns the rank of this square.
 * @method self|null fromAscii(string $s) Creates from ASCII string (e.g., "e4").
 * @method string __toString() Returns algebraic notation (e.g., "e4") (PHP magic __toString).
 * @method bool isLight() Returns true if square is a light-colored square.
 * @method bool isDark() Returns true if square is a dark-colored square.
 * @method int distance(self $other) Returns Chebyshev distance to another square.
 * @method self flipHorizontal() Returns horizontally flipped square.
 * @method self flipVertical() Returns vertically flipped square.
 */
final class Square {
    const int A1 = 0; const int B1 = 1; const int C1 = 2; const int D1 = 3;
    const int E1 = 4; const int F1 = 5; const int G1 = 6; const int H1 = 7;
    const int A2 = 8; const int B2 = 9; const int C2 = 10; const int D2 = 11;
    const int E2 = 12; const int F2 = 13; const int G2 = 14; const int H2 = 15;
    const int A3 = 16; const int B3 = 17; const int C3 = 18; const int D3 = 19;
    const int E3 = 20; const int F3 = 21; const int G3 = 22; const int H3 = 23;
    const int A4 = 24; const int B4 = 25; const int C4 = 26; const int D4 = 27;
    const int E4 = 28; const int F4 = 29; const int G4 = 30; const int H4 = 31;
    const int A5 = 32; const int B5 = 33; const int C5 = 34; const int D5 = 35;
    const int E5 = 36; const int F5 = 37; const int G5 = 38; const int H5 = 39;
    const int A6 = 40; const int B6 = 41; const int C6 = 42; const int D6 = 43;
    const int E6 = 44; const int F6 = 45; const int G6 = 46; const int H6 = 47;
    const int A7 = 48; const int B7 = 49; const int C7 = 50; const int D7 = 51;
    const int E7 = 52; const int F7 = 53; const int G7 = 54; const int H7 = 55;
    const int A8 = 56; const int B8 = 57; const int C8 = 58; const int D8 = 59;
    const int E8 = 60; const int F8 = 61; const int G8 = 62; const int H8 = 63;
    const int NS = 64;
    public function __construct(int $value) {}
}
