use ext_php_rs::prelude::*;

/// PHP class: `shakmaty\File`
///
/// Represents a chessboard file (column) from A (0) to H (7).
///
/// # Constants
///
/// - `File::A` = 0 through `File::H` = 7
///
/// # Usage
///
/// ```php
/// $file = \shakmaty\File::fromChar('e');
/// echo $file->value(); // 4
/// echo $file->toChar();    // "e"
/// echo $file->upperChar(); // "E"
/// ```
///
/// # See Also
///
/// Original Rust type: [`shakmaty::File`](https://docs.rs/shakmaty/0.30/shakmaty/struct.File.html)
#[php_class]
#[php(name = "shakmaty\\File")]
pub struct PhpFile {
    pub inner: u8, // 0..7 = A..H
}

#[php_impl]
impl PhpFile {
    /// Creates a new `File`.
    ///
    /// Accepts values from 0 (A) to 7 (H); out-of-range values fall back to file A.
    #[php(constructor)]
    pub fn new(value: i32) -> Self {
        let v = if (0..=7).contains(&value) { value as u8 } else { 0 };
        PhpFile { inner: v }
    }

    /// Returns the numeric file index, where A=0 .. H=7.
    pub fn value(&self) -> i32 { self.inner as i32 }

    /// Returns the lowercase letter for this file (`"a"` through `"h"`).
    pub fn to_char(&self) -> String {
        char::from(b'a' + self.inner).to_string()
    }

    /// Returns the uppercase letter for this file (`"A"` through `"H"`).
    pub fn upper_char(&self) -> String {
        char::from(b'A' + self.inner).to_string()
    }

    /// Returns the absolute distance (in files) between two files.
    ///
    /// ```php
    /// $a = new \shakmaty\File(\shakmaty\File::A); // 0
    /// $e = new \shakmaty\File(4);                  // e-file
    /// echo $a->distance($e); // 4
    /// ```
    pub fn distance(&self, other: &PhpFile) -> i32 {
        (self.inner as i32 - other.inner as i32).abs()
    }

    /// Returns the mirrored file across the vertical axis (`a`↔`h`, `b`↔`g`, ...).
    pub fn flip_horizontal(&self) -> Self {
        PhpFile { inner: 7 - self.inner }
    }

    /// Creates a `File` from its letter.
    ///
    /// Accepts both uppercase and lowercase letters `"a"` through `"h"`.
    /// Returns `None` for any other character.
    pub fn from_char(ch: String) -> Option<Self> {
        let c = ch.chars().next()?;
        let idx = match c {
            'a'|'A' => 0, 'b'|'B' => 1, 'c'|'C' => 2,
            'd'|'D' => 3, 'e'|'E' => 4, 'f'|'F' => 5,
            'g'|'G' => 6, 'h'|'H' => 7,
            _ => return None,
        };
        Some(PhpFile { inner: idx })
    }

    pub const A: i32 = 0;
    pub const B: i32 = 1;
    pub const C: i32 = 2;
    pub const D: i32 = 3;
    pub const E: i32 = 4;
    pub const F: i32 = 5;
    pub const G: i32 = 6;
    pub const H: i32 = 7;
}

/// PHP class: `shakmaty\Rank`
///
/// Represents a chessboard rank (row) from First (0) to Eighth (7).
///
/// # Constants
///
/// - `Rank::FIRST` = 0 through `Rank::EIGHTH` = 7
///
/// # Usage
///
/// ```php
/// $rank = \shakmaty\Rank::fromChar('4');
/// echo $rank->value(); // 3 (zero-based)
/// echo $rank->toChar(); // "4"
/// ```
///
/// # See Also
///
/// Original Rust type: [`shakmaty::Rank`](https://docs.rs/shakmaty/0.30/shakmaty/struct.Rank.html)
#[php_class]
#[php(name = "shakmaty\\Rank")]
pub struct PhpRank {
    pub inner: u8, // 0..7 = First..Eighth
}

#[php_impl]
impl PhpRank {
    /// Creates a new `Rank`.
    ///
    /// Accepts values from 0 (First) to 7 (Eighth); out-of-range falls back to the first rank.
    #[php(constructor)]
    pub fn new(value: i32) -> Self {
        let v = if (0..=7).contains(&value) { value as u8 } else { 0 };
        PhpRank { inner: v }
    }

    /// Returns the numeric rank index, where First=0 .. Eighth=7.
    pub fn value(&self) -> i32 { self.inner as i32 }

    /// Returns the digit character for this rank (`"1"` through `"8"`).
    pub fn to_char(&self) -> String {
        char::from(b'1' + self.inner).to_string()
    }

    /// Returns the absolute distance (in ranks) between two ranks.
    pub fn distance(&self, other: &PhpRank) -> i32 {
        (self.inner as i32 - other.inner as i32).abs()
    }

    /// Returns the mirrored rank across the horizontal axis (`1`↔`8`, `2`↔`7`, ...).
    pub fn flip_vertical(&self) -> Self {
        PhpRank { inner: 7 - self.inner }
    }

    /// Creates a `Rank` from its digit.
    ///
    /// Accepts the characters `"1"` through `"8"`. Returns `None` for any other character.
    pub fn from_char(ch: String) -> Option<Self> {
        let c = ch.chars().next()?;
        let idx = match c {
            '1' => 0, '2' => 1, '3' => 2, '4' => 3,
            '5' => 4, '6' => 5, '7' => 6, '8' => 7,
            _ => return None,
        };
        Some(PhpRank { inner: idx })
    }

    pub const FIRST: i32 = 0;
    pub const SECOND: i32 = 1;
    pub const THIRD: i32 = 2;
    pub const FOURTH: i32 = 3;
    pub const FIFTH: i32 = 4;
    pub const SIXTH: i32 = 5;
    pub const SEVENTH: i32 = 6;
    pub const EIGHTH: i32 = 7;
}

/// PHP class: `shakmaty\Square`
///
/// Represents a square on a chessboard, with index 0 (A1) through 63 (H8).
///
/// # Constants
///
/// All 64 squares and the null square NS are available as class constants:
/// `Square::A1` (0) through `Square::H8` (63).
///
/// # Usage
///
/// ```php
/// $sq = \shakmaty\Square::fromAscii('e4');
/// echo $sq->__toString(); // "e4"
/// echo $sq->file()->toChar(); // "e"
/// echo $sq->rank()->toChar(); // "4"
/// var_dump($sq->isLight()); // true
/// ```
///
/// # See Also
///
/// Original Rust type: [`shakmaty::Square`](https://docs.rs/shakmaty/0.30/shakmaty/struct.Square.html)
#[php_class]
#[php(name = "shakmaty\\Square")]
pub struct PhpSquare {
    pub inner: u8, // 0..63
}

#[php_impl]
impl PhpSquare {
    /// Creates a new `Square`.
    ///
    /// Accepts values from 0 (A1) to 63 (H8); out-of-range falls back to A1.
    #[php(constructor)]
    pub fn new(value: i32) -> Self {
        let v = if (0..=63).contains(&value) { value as u8 } else { 0 };
        PhpSquare { inner: v }
    }

    /// Returns the zero-based square index, where A1=0 .. H8=63.
    pub fn value(&self) -> i32 { self.inner as i32 }

    /// Creates a `Square` from its file and rank components.
    pub fn from_coords(file: &PhpFile, rank: &PhpRank) -> Self {
        PhpSquare { inner: file.inner | (rank.inner << 3) }
    }

    /// Returns the File (column) of this square.
    pub fn file(&self) -> PhpFile {
        PhpFile { inner: self.inner & 7 }
    }

    /// Returns the Rank (row) of this square.
    pub fn rank(&self) -> PhpRank {
        PhpRank { inner: self.inner >> 3 }
    }

    /// Creates a `Square` from its two-character coordinate, e.g. `"e4"`.
    ///
    /// Returns `None` if the string is not exactly two characters or does not name a valid square.
    pub fn from_ascii(s: String) -> Option<Self> {
        let bytes = s.as_bytes();
        if bytes.len() != 2 { return None; }
        let f = bytes[0] as char;
        let r = bytes[1] as char;
        let file = PhpFile::from_char(f.to_string())?;
        let rank = PhpRank::from_char(r.to_string())?;
        Some(Self::from_coords(&file, &rank))
    }

    /// Returns the FEN coordinate of this square (e.g. `"a1"` .. `"h8"`).
    pub fn __to_string(&self) -> String {
        let file_char = char::from(b'a' + (self.inner & 7));
        let rank_char = char::from(b'1' + (self.inner >> 3));
        format!("{}{}", file_char, rank_char)
    }

    /// Returns `true` if this is a light-colored square.
    pub fn is_light(&self) -> bool {
        let f = (self.inner & 7) as u32;
        let r = (self.inner >> 3) as u32;
        (f + r) % 2 == 1
    }

    /// Returns `true` if this is a dark-colored square.
    pub fn is_dark(&self) -> bool {
        !self.is_light()
    }

    /// Returns the Chebyshev distance between two squares — the maximum of their file and rank deltas.
    pub fn distance(&self, other: &PhpSquare) -> i32 {
        let fd = ((self.inner & 7) as i32 - (other.inner & 7) as i32).abs();
        let rd = ((self.inner >> 3) as i32 - (other.inner >> 3) as i32).abs();
        fd.max(rd)
    }

    /// Returns the square mirrored across the vertical axis (`a`↔`h`, `b`↔`g`, ...), keeping its rank.
    pub fn flip_horizontal(&self) -> Self {
        PhpSquare { inner: self.inner ^ 7 }
    }

    /// Returns the square mirrored across the horizontal axis (rank `1`↔`8`, `2`↔`7`, ...).
    pub fn flip_vertical(&self) -> Self {
        PhpSquare { inner: self.inner ^ 56 }
    }
    
    #[php(name = "A1")] pub const A1: i32 = 0;
    #[php(name = "B1")] pub const B1: i32 = 1;
    #[php(name = "C1")] pub const C1: i32 = 2;
    #[php(name = "D1")] pub const D1: i32 = 3;
    #[php(name = "E1")] pub const E1: i32 = 4;
    #[php(name = "F1")] pub const F1: i32 = 5;
    #[php(name = "G1")] pub const G1: i32 = 6;
    #[php(name = "H1")] pub const H1: i32 = 7;

    #[php(name = "A2")] pub const A2: i32 = 8;
    #[php(name = "B2")] pub const B2: i32 = 9;
    #[php(name = "C2")] pub const C2: i32 = 10;
    #[php(name = "D2")] pub const D2: i32 = 11;
    #[php(name = "E2")] pub const E2: i32 = 12;
    #[php(name = "F2")] pub const F2: i32 = 13;
    #[php(name = "G2")] pub const G2: i32 = 14;
    #[php(name = "H2")] pub const H2: i32 = 15;

    #[php(name = "A3")] pub const A3: i32 = 16;
    #[php(name = "B3")] pub const B3: i32 = 17;
    #[php(name = "C3")] pub const C3: i32 = 18;
    #[php(name = "D3")] pub const D3: i32 = 19;
    #[php(name = "E3")] pub const E3: i32 = 20;
    #[php(name = "F3")] pub const F3: i32 = 21;
    #[php(name = "G3")] pub const G3: i32 = 22;
    #[php(name = "H3")] pub const H3: i32 = 23;

    #[php(name = "A4")] pub const A4: i32 = 24;
    #[php(name = "B4")] pub const B4: i32 = 25;
    #[php(name = "C4")] pub const C4: i32 = 26;
    #[php(name = "D4")] pub const D4: i32 = 27;
    #[php(name = "E4")] pub const E4: i32 = 28;
    #[php(name = "F4")] pub const F4: i32 = 29;
    #[php(name = "G4")] pub const G4: i32 = 30;
    #[php(name = "H4")] pub const H4: i32 = 31;

    #[php(name = "A5")] pub const A5: i32 = 32;
    #[php(name = "B5")] pub const B5: i32 = 33;
    #[php(name = "C5")] pub const C5: i32 = 34;
    #[php(name = "D5")] pub const D5: i32 = 35;
    #[php(name = "E5")] pub const E5: i32 = 36;
    #[php(name = "F5")] pub const F5: i32 = 37;
    #[php(name = "G5")] pub const G5: i32 = 38;
    #[php(name = "H5")] pub const H5: i32 = 39;

    #[php(name = "A6")] pub const A6: i32 = 40;
    #[php(name = "B6")] pub const B6: i32 = 41;
    #[php(name = "C6")] pub const C6: i32 = 42;
    #[php(name = "D6")] pub const D6: i32 = 43;
    #[php(name = "E6")] pub const E6: i32 = 44;
    #[php(name = "F6")] pub const F6: i32 = 45;
    #[php(name = "G6")] pub const G6: i32 = 46;
    #[php(name = "H6")] pub const H6: i32 = 47;

    #[php(name = "A7")] pub const A7: i32 = 48;
    #[php(name = "B7")] pub const B7: i32 = 49;
    #[php(name = "C7")] pub const C7: i32 = 50;
    #[php(name = "D7")] pub const D7: i32 = 51;
    #[php(name = "E7")] pub const E7: i32 = 52;
    #[php(name = "F7")] pub const F7: i32 = 53;
    #[php(name = "G7")] pub const G7: i32 = 54;
    #[php(name = "H7")] pub const H7: i32 = 55;

    #[php(name = "A8")] pub const A8: i32 = 56;
    #[php(name = "B8")] pub const B8: i32 = 57;
    #[php(name = "C8")] pub const C8: i32 = 58;
    #[php(name = "D8")] pub const D8: i32 = 59;
    #[php(name = "E8")] pub const E8: i32 = 60;
    #[php(name = "F8")] pub const F8: i32 = 61;
    #[php(name = "G8")] pub const G8: i32 = 62;
    #[php(name = "H8")] pub const H8: i32 = 63;

    /// Null square — represents an undefined/off-board square (value 64).
    #[php(name = "NS")]
    pub const NS: i32 = 64;
}
