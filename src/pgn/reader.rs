use std::io::{Cursor, Seek, SeekFrom};
use std::ops::ControlFlow;

use ext_php_rs::prelude::*;
use pgn_reader::{Nag, RawComment, RawTag, Reader, SanPlus, Skip, Visitor};
use shakmaty::Outcome;

use crate::pgn::game::{GameData, PhpGame};
use crate::pgn::token::TokenKind;

/// Internal visitor that collects an entire PGN game into [`GameData`].
///
/// The reader calls `partial_comment()` for oversized comments and `comment()`
/// for the final chunk; both are merged here so that each logical comment
/// becomes a single [`TokenKind::Comment`] token.
#[derive(Default)]
struct Collector {
    pending_comment: Vec<u8>,
}

impl Visitor for Collector {
    type Tags = Vec<(String, String)>;
    type Movetext = GameData;
    type Output = GameData;

    fn begin_tags(&mut self) -> ControlFlow<Self::Output, Self::Tags> {
        ControlFlow::Continue(Vec::new())
    }

    fn tag(
        &mut self,
        tags: &mut Self::Tags,
        name: &[u8],
        value: RawTag<'_>,
    ) -> ControlFlow<Self::Output> {
        let name = String::from_utf8_lossy(name).into_owned();
        let value = value.decode_utf8_lossy().into_owned();
        tags.push((name, value));
        ControlFlow::Continue(())
    }

    fn begin_movetext(&mut self, tags: Self::Tags) -> ControlFlow<Self::Output, Self::Movetext> {
        ControlFlow::Continue(GameData {
            tags,
            ..GameData::default()
        })
    }

    fn san(
        &mut self,
        movetext: &mut Self::Movetext,
        san_plus: SanPlus,
    ) -> ControlFlow<Self::Output> {
        if movetext.variation_depth == 0 {
            movetext.mainline.push(san_plus.to_string());
        }
        movetext.movetext.push(TokenKind::San(san_plus));
        ControlFlow::Continue(())
    }

    fn nag(&mut self, movetext: &mut Self::Movetext, nag: Nag) -> ControlFlow<Self::Output> {
        movetext.movetext.push(TokenKind::Nag(nag));
        ControlFlow::Continue(())
    }

    fn partial_comment(
        &mut self,
        _movetext: &mut Self::Movetext,
        comment: RawComment<'_>,
    ) -> ControlFlow<Self::Output> {
        self.pending_comment.extend_from_slice(comment.as_bytes());
        ControlFlow::Continue(())
    }

    fn comment(
        &mut self,
        movetext: &mut Self::Movetext,
        comment: RawComment<'_>,
    ) -> ControlFlow<Self::Output> {
        let mut bytes = std::mem::take(&mut self.pending_comment);
        bytes.extend_from_slice(comment.as_bytes());
        movetext
            .movetext
            .push(TokenKind::Comment(String::from_utf8_lossy(&bytes).into_owned()));
        ControlFlow::Continue(())
    }

    fn begin_variation(
        &mut self,
        movetext: &mut Self::Movetext,
    ) -> ControlFlow<Self::Output, Skip> {
        movetext.variation_depth += 1;
        movetext.movetext.push(TokenKind::VariationBegin);
        ControlFlow::Continue(Skip(false))
    }

    fn end_variation(&mut self, movetext: &mut Self::Movetext) -> ControlFlow<Self::Output> {
        movetext.variation_depth = movetext.variation_depth.saturating_sub(1);
        movetext.movetext.push(TokenKind::VariationEnd);
        ControlFlow::Continue(())
    }

    fn outcome(
        &mut self,
        movetext: &mut Self::Movetext,
        outcome: Outcome,
    ) -> ControlFlow<Self::Output> {
        movetext.outcome = Some(outcome);
        movetext.movetext.push(TokenKind::Outcome(outcome));
        ControlFlow::Continue(())
    }

    fn end_game(&mut self, movetext: Self::Movetext) -> Self::Output {
        movetext
    }
}

/// PHP class: `shakmaty\pgn\Reader`
///
/// A streaming reader for PGN documents, wrapping
/// [`pgn_reader::Reader`](https://docs.rs/pgn-reader/0.29/pgn_reader/struct.Reader.html).
///
/// The document is supplied as a string to the constructor. Games are read one
/// at a time with [`Reader::read_game()`](#method.read_game) or all at once
/// with [`Reader::read_all()`](#method.read_all).
///
/// # Usage
///
/// ```php
/// $reader = new \shakmaty\pgn\Reader($pgn);
///
/// while ($reader->has_more()) {
///     $game = $reader->read_game();
///     echo $game->tag('White'), ' ', $game->outcome(), "\n";
/// }
/// ```
#[php_class]
#[php(name = "shakmaty\\pgn\\Reader")]
pub struct PhpReader {
    pub data: Vec<u8>,
    pub offset: u64,
    pub tag_line_bytes: usize,
    pub comment_bytes: usize,
}

impl PhpReader {
    /// Runs `parse` against a fresh reader positioned at the current offset and
    /// returns the produced value together with the new logical offset.
    fn with_reader<'a, T>(
        &'a self,
        parse: impl FnOnce(&mut Reader<Cursor<&'a [u8]>>, &mut Collector) -> std::io::Result<T>,
    ) -> Result<(T, u64), String> {
        let mut reader = Reader::build(Cursor::new(self.data.as_slice()))
            .set_supported_tag_line_length(self.tag_line_bytes)
            .set_supported_comment_length(self.comment_bytes)
            .finish();
        reader
            .seek(SeekFrom::Start(self.offset))
            .map_err(|err| err.to_string())?;
        let mut collector = Collector::default();
        let value = parse(&mut reader, &mut collector).map_err(|err| err.to_string())?;
        let position = reader.stream_position().map_err(|err| err.to_string())?;
        Ok((value, position))
    }
}

#[php_impl]
impl PhpReader {
    /// Creates a reader over the given PGN document.
    #[php(constructor)]
    pub fn new(pgn: String) -> Self {
        PhpReader {
            data: pgn.into_bytes(),
            offset: 0,
            tag_line_bytes: 255,
            comment_bytes: 255,
        }
    }

    /// Configures the buffer to support *at least* the given tag line length.
    ///
    /// Values below `255` are clamped to `255`. Defaults to `255`.
    pub fn set_supported_tag_line_length(&mut self, bytes: i32) {
        self.tag_line_bytes = bytes.max(255) as usize;
    }

    /// Configures the buffer to support *at least* the given comment length.
    ///
    /// Longer comments are still reported as a single comment token. Values
    /// below `255` are clamped to `255`. Defaults to `255`.
    pub fn set_supported_comment_length(&mut self, bytes: i32) {
        self.comment_bytes = bytes.max(255) as usize;
    }

    /// Reads the next game, or returns `null` when the document is exhausted.
    ///
    /// # Errors
    ///
    /// Throws a PHP exception on irrecoverable parser errors.
    pub fn read_game(&mut self) -> Result<Option<PhpGame>, String> {
        let (game, position) =
            self.with_reader(|reader, collector| reader.read_game(collector))?;
        self.offset = position;
        Ok(game.map(|inner| PhpGame { inner }))
    }

    /// Returns whether another game is available, without parsing it.
    ///
    /// # Errors
    ///
    /// Throws a PHP exception on irrecoverable parser errors.
    pub fn has_more(&mut self) -> Result<bool, String> {
        let (has_more, position) = self.with_reader(|reader, _collector| reader.has_more())?;
        self.offset = position;
        Ok(has_more)
    }

    /// Skips the next game, returning `true` if a game was skipped.
    ///
    /// # Errors
    ///
    /// Throws a PHP exception on irrecoverable parser errors.
    pub fn skip_game(&mut self) -> Result<bool, String> {
        let (skipped, position) = self.with_reader(|reader, _collector| reader.skip_game())?;
        self.offset = position;
        Ok(skipped)
    }

    /// Reads all remaining games into an array of `shakmaty\pgn\Game`.
    ///
    /// # Errors
    ///
    /// Throws a PHP exception on irrecoverable parser errors.
    pub fn read_all(&mut self) -> Result<Vec<PhpGame>, String> {
        let mut games = Vec::new();
        while let Some(game) = self.read_game()? {
            games.push(game);
        }
        Ok(games)
    }

    /// Returns the current byte offset of the reader within the document.
    pub fn offset(&self) -> i64 {
        self.offset as i64
    }

    /// Resets the reader back to the start of the document.
    pub fn reset(&mut self) {
        self.offset = 0;
    }
}
