//! Symbols that parser works on
use std::fmt::Formatter;
use std::fmt::{Debug, Display};
use std::sync::OnceLock;

/// Type of tokens that parser considers invalid
pub const TOKEN_INVALID_TYPE: i32 = 0;
/// Type of tokens that DFA can use to advance to next state without consuming actual input token.
/// Should not be created by downstream implementations.
pub const TOKEN_EPSILON: i32 = -2;
/// Min token type that can be assigned to tokens created by downstream implementations.
pub const TOKEN_MIN_USER_TOKEN_TYPE: i32 = 1;
/// Type of EOF token
pub const TOKEN_EOF: i32 = crate::int_stream::EOF;
/// Default channel lexer emits tokens to
pub const TOKEN_DEFAULT_CHANNEL: i32 = 0;
/// Predefined additional channel for lexer to assign tokens to
pub const TOKEN_HIDDEN_CHANNEL: i32 = 1;
/// Shorthand for TOKEN_HIDDEN_CHANNEL
pub const HIDDEN: i32 = TOKEN_HIDDEN_CHANNEL;

/// Implemented by tokens that are produced by a `TokenFactory`
#[allow(missing_docs)]
pub trait Token: Debug + Display {
    /***
     * Begin `Token` interface
     */

    /// Get the text of the token
    fn get_text(&self) -> &str;

    /// Get the token type of the token
    fn get_token_type(&self) -> i32;

    /// The line number on which the first character of this token was
    /// matched. The first line in the input is line 1.
    fn get_line(&self) -> u32;

    /// The index of the first character of this token relative to the beginning
    /// of the line at which it occurs. The first character on a line has
    /// position 0.
    fn get_char_position_in_line(&self) -> i32;

    /// Returns the channel number this token was assigned to. Each token can
    /// arrive on a different channel, but the parser only "tunes" to a single
    /// channel. The parser ignores everything not on DEFAULT_CHANNEL.
    fn get_channel(&self) -> i32;

    /// An index from 0..n-1 of the token object in the token stream.
    /// This must be valid in order to print token streams and use
    /// token stream rewinding.
    ///
    /// Return -1 to indicate that the token was conjured up and does not
    /// have a valid index.
    fn get_token_index(&self) -> isize;

    /// The starting character index of the token
    ///
    /// This method is optional; return -1 if not implemented.
    fn get_start_index(&self) -> isize;

    /// The last character index of the token
    ///
    /// This method is optional; return -1 if not implemented.
    fn get_stop_index(&self) -> isize;

    #[doc(hidden)]
    #[inline]
    fn get_pos_offset(&self) -> Option<u32> {
        let start = self.get_start_index();
        if start >= 0 {
            Some(start as u32)
        } else {
            None
        }
    }

    /***
     * Begin `WritableToken` interface
     */

    fn set_text(&mut self, _text: String);

    fn set_type(&mut self, _ttype: i32);

    fn set_channel(&mut self, _channel: i32);

    fn set_token_index(&mut self, _v: isize);
}

const TOKEN_TYPE_BITS: u32 = 22;
const TOKEN_CHANNEL_BITS: u32 = 32 - TOKEN_TYPE_BITS;
const TOKEN_TYPE_MASK: i32 = (1 << TOKEN_TYPE_BITS) - 1;
const TOKEN_CHANNEL_MASK: i32 = !TOKEN_TYPE_MASK;
const TOKEN_TYPE_MAX: i32 = (TOKEN_TYPE_MASK as u32 >> 1) as i32;
const TOKEN_TYPE_MIN: i32 = -TOKEN_TYPE_MAX;
const TOKEN_CHANNEL_MAX: i32 = 1i32 << (TOKEN_CHANNEL_BITS - 1);
const TOKEN_CHANNEL_MIN: i32 = -TOKEN_CHANNEL_MAX;
#[inline]
pub(crate) fn line_from_starts(line_starts: &[u32], byte_offset: u32) -> u32 {
    line_starts.partition_point(|&start| start <= byte_offset) as u32
}

#[inline]
pub(crate) fn column_from_starts(
    line_starts: &[u32],
    is_ascii: bool,
    input: &str,
    byte_offset: u32,
) -> i32 {
    let line_idx = line_starts.partition_point(|&start| start <= byte_offset);
    let line_start = line_starts[line_idx - 1] as usize;
    let byte_offset = (byte_offset as usize).min(input.len());
    if is_ascii {
        (byte_offset - line_start) as i32
    } else {
        input[line_start..byte_offset].chars().count() as i32
    }
}

/// Lazily constructed newline-offset table for $O(\log(\text{lines}))$ line and
/// column lookup over an input string.
#[derive(Debug)]
pub struct LineIndex {
    line_starts: Vec<u32>,
    is_ascii: bool,
}

impl LineIndex {
    pub fn new(input: &str) -> Self {
        let is_ascii = input.is_ascii();
        let mut line_starts = vec![0];
        line_starts.extend(memchr::memchr_iter(b'\n', input.as_bytes()).map(|i| (i + 1) as u32));
        Self {
            line_starts,
            is_ascii,
        }
    }

    #[inline]
    pub fn line(&self, byte_offset: u32) -> u32 {
        line_from_starts(&self.line_starts, byte_offset)
    }

    #[inline]
    pub fn char_position_in_line(&self, input: &str, byte_offset: u32) -> i32 {
        column_from_starts(&self.line_starts, self.is_ascii, input, byte_offset)
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum SourceFallback<'input> {
    Stream(&'input SourceInfo<'input>),
    Fixed { line: u32, column: i32 },
}

/// Shared input text and lazily built [`LineIndex`] referenced by [`CommonToken`].
#[derive(Debug)]
pub struct SourceInfo<'input> {
    pub(crate) input: &'input str,
    line_index: OnceLock<LineIndex>,
    fallback: Option<SourceFallback<'input>>,
}

impl<'input> SourceInfo<'input> {
    pub(crate) const fn new_static(input: &'input str) -> Self {
        Self {
            input,
            line_index: OnceLock::new(),
            fallback: None,
        }
    }

    pub(crate) const fn new_stream(input: &'input str) -> Self {
        Self {
            input,
            line_index: OnceLock::new(),
            fallback: None,
        }
    }

    pub(crate) const fn new_override(
        input: &'input str,
        fallback: Option<SourceFallback<'input>>,
    ) -> Self {
        Self {
            input,
            line_index: OnceLock::new(),
            fallback,
        }
    }

    #[inline]
    pub fn line_index(&self) -> &LineIndex {
        self.line_index.get_or_init(|| LineIndex::new(self.input))
    }

    #[inline]
    pub fn line(&self, byte_offset: u32) -> u32 {
        match self.fallback {
            None => self.line_index().line(byte_offset),
            Some(SourceFallback::Stream(s)) => s.line(byte_offset),
            Some(SourceFallback::Fixed { line, .. }) => line,
        }
    }

    #[inline]
    pub fn char_position_in_line(&self, byte_offset: u32) -> i32 {
        match self.fallback {
            None => self
                .line_index()
                .char_position_in_line(self.input, byte_offset),
            Some(SourceFallback::Stream(s)) => s.char_position_in_line(byte_offset),
            Some(SourceFallback::Fixed { column, .. }) => column,
        }
    }
}

/// Zero-copy token backed by a shared [`SourceInfo`] and lazy line/column index.
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub struct CommonToken<'input> {
    source: &'input SourceInfo<'input>,
    type_and_channel: i32,
    pub start: i32,
    pub stop: i32,
    pub token_index: i32,
    text_end: u32,
    pos: u32,
}

const _: () = assert!(size_of::<CommonToken<'static>>() == 32);
impl<'input> CommonToken<'input> {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new_slice(
        source: &'input SourceInfo<'input>,
        token_type: i32,
        channel: i32,
        start: i32,
        stop: i32,
        token_index: i32,
        text_end: u32,
        pos: u32,
    ) -> Self {
        Self {
            source,
            type_and_channel: (channel << TOKEN_TYPE_BITS) | (token_type & TOKEN_TYPE_MASK),
            start,
            stop,
            token_index,
            text_end,
            pos,
        }
    }
}

/// Token that owns its data
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub struct OwningToken {
    type_and_channel: i32,
    pub start: i32,
    pub stop: i32,
    pub token_index: i32,
    pub line: u32,
    pub column: i32,
    pub text: String,
}

impl OwningToken {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        token_type: i32,
        channel: i32,
        start: i32,
        stop: i32,
        token_index: i32,
        line: u32,
        column: i32,
        text: String,
    ) -> Self {
        Self {
            type_and_channel: (channel << TOKEN_TYPE_BITS) | (token_type & TOKEN_TYPE_MASK),
            start,
            stop,
            token_index,
            line,
            column,
            text,
        }
    }
}

fn fmt_token<T: Token>(tok: &T, f: &mut Formatter<'_>) -> std::fmt::Result {
    let txt = tok.get_text();
    let txt = txt.replace("\n", "\\n");
    let txt = txt.replace("\r", "\\r");
    let txt = txt.replace("\t", "\\t");
    f.write_fmt(format_args!(
        "[@{},{}:{}='{}',<{}>{},{}:{}]",
        tok.get_token_index(),
        tok.get_start_index(),
        tok.get_stop_index(),
        txt,
        tok.get_token_type(),
        if tok.get_channel() > 0 {
            ",channel=".to_owned() + tok.get_channel().to_string().as_str()
        } else {
            String::new()
        },
        tok.get_line(),
        tok.get_char_position_in_line()
    ))
}

impl Display for CommonToken<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt_token(self, f)
    }
}

impl Display for OwningToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt_token(self, f)
    }
}

impl Token for CommonToken<'_> {
    #[inline(always)]
    fn get_token_type(&self) -> i32 {
        (self.type_and_channel << TOKEN_CHANNEL_BITS) >> TOKEN_CHANNEL_BITS
    }

    #[inline(always)]
    fn get_channel(&self) -> i32 {
        self.type_and_channel >> TOKEN_TYPE_BITS
    }

    #[inline(always)]
    fn get_start_index(&self) -> isize {
        self.start as isize
    }

    #[inline(always)]
    fn get_stop_index(&self) -> isize {
        self.stop as isize
    }

    #[inline]
    fn get_line(&self) -> u32 {
        if self.pos == u32::MAX {
            0
        } else {
            self.source.line(self.pos)
        }
    }

    #[inline]
    fn get_char_position_in_line(&self) -> i32 {
        if self.pos == u32::MAX {
            0
        } else {
            self.source.char_position_in_line(self.pos)
        }
    }

    #[inline(always)]
    fn get_pos_offset(&self) -> Option<u32> {
        if self.pos == u32::MAX {
            None
        } else {
            Some(self.pos)
        }
    }

    #[inline(always)]
    fn get_text(&self) -> &str {
        if self.get_token_type() == TOKEN_EOF || self.text_end == u32::MAX - 1 {
            "<EOF>"
        } else if self.text_end != u32::MAX {
            &self.source.input[self.start as usize..self.text_end as usize]
        } else {
            self.source.input
        }
    }

    #[inline(always)]
    fn get_token_index(&self) -> isize {
        self.token_index as isize
    }

    fn set_token_index(&mut self, v: isize) {
        self.token_index = v as i32;
    }

    fn set_text(&mut self, _text: String) {
        panic!("Cannot set text of &str, it is immutable");
    }

    #[inline]
    fn set_type(&mut self, ttype: i32) {
        debug_assert!(
            (TOKEN_TYPE_MIN..=TOKEN_TYPE_MAX).contains(&ttype),
            "Token type {} is out of range ({}..={})",
            ttype,
            TOKEN_TYPE_MIN,
            TOKEN_TYPE_MAX
        );

        self.type_and_channel =
            (self.type_and_channel & !(TOKEN_TYPE_MASK)) | (ttype & TOKEN_TYPE_MASK);
    }

    #[inline]
    fn set_channel(&mut self, channel: i32) {
        debug_assert!(
            (TOKEN_CHANNEL_MIN..=TOKEN_CHANNEL_MAX).contains(&channel),
            "Token channel {} is out of range ({}..={})",
            channel,
            TOKEN_CHANNEL_MIN,
            TOKEN_CHANNEL_MAX
        );

        self.type_and_channel =
            (self.type_and_channel & !(TOKEN_CHANNEL_MASK)) | (channel << TOKEN_TYPE_BITS);
    }
}

impl Token for OwningToken {
    #[inline(always)]
    fn get_token_type(&self) -> i32 {
        (self.type_and_channel << TOKEN_CHANNEL_BITS) >> TOKEN_CHANNEL_BITS
    }

    #[inline(always)]
    fn get_channel(&self) -> i32 {
        self.type_and_channel >> TOKEN_TYPE_BITS
    }

    #[inline(always)]
    fn get_start_index(&self) -> isize {
        self.start as isize
    }

    #[inline(always)]
    fn get_stop_index(&self) -> isize {
        self.stop as isize
    }

    #[inline(always)]
    fn get_line(&self) -> u32 {
        self.line
    }

    #[inline(always)]
    fn get_char_position_in_line(&self) -> i32 {
        self.column
    }

    #[inline(always)]
    fn get_text(&self) -> &str {
        if self.get_token_type() == TOKEN_EOF {
            "<EOF>"
        } else {
            self.text.as_str()
        }
    }

    #[inline(always)]
    fn get_token_index(&self) -> isize {
        self.token_index as isize
    }

    fn set_token_index(&mut self, v: isize) {
        self.token_index = v as i32;
    }

    fn set_text(&mut self, text: String) {
        self.text = text;
    }

    #[inline]
    fn set_type(&mut self, ttype: i32) {
        debug_assert!(
            (TOKEN_TYPE_MIN..=TOKEN_TYPE_MAX).contains(&ttype),
            "Token type {} is out of range ({}..={})",
            ttype,
            TOKEN_TYPE_MIN,
            TOKEN_TYPE_MAX
        );

        self.type_and_channel =
            (self.type_and_channel & !(TOKEN_TYPE_MASK)) | (ttype & TOKEN_TYPE_MASK);
    }

    #[inline]
    fn set_channel(&mut self, channel: i32) {
        debug_assert!(
            (TOKEN_CHANNEL_MIN..=TOKEN_CHANNEL_MAX).contains(&channel),
            "Token channel {} is out of range ({}..={})",
            channel,
            TOKEN_CHANNEL_MIN,
            TOKEN_CHANNEL_MAX
        );

        self.type_and_channel =
            (self.type_and_channel & !(TOKEN_CHANNEL_MASK)) | (channel << TOKEN_TYPE_BITS);
    }
}

impl Default for &'_ CommonToken<'_> {
    fn default() -> Self {
        &INVALID_COMMON
    }
}

impl From<&dyn Token> for OwningToken {
    fn from(value: &dyn Token) -> Self {
        OwningToken::new(
            value.get_token_type(),
            value.get_channel(),
            value.get_start_index() as i32,
            value.get_stop_index() as i32,
            value.get_token_index() as i32,
            value.get_line(),
            value.get_char_position_in_line(),
            value.get_text().to_owned(),
        )
    }
}

static INVALID_SOURCE_INFO: SourceInfo<'static> = SourceInfo::new_static("<invalid>");

pub(crate) static INVALID_COMMON: CommonToken<'static> = CommonToken::new_slice(
    &INVALID_SOURCE_INFO,
    TOKEN_INVALID_TYPE,
    0,
    -1,
    -1,
    -1,
    u32::MAX,
    u32::MAX,
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_stream::InputStream;
    use crate::token_factory::{CommonTokenFactory, TokenFactory};
    use crate::Arena;

    #[test]
    fn test_common_token_size_and_lazy_line_index() {
        assert_eq!(size_of::<CommonToken<'static>>(), 32);

        let input_text = "alpha\nbeta は!\ngamma";
        Arena::with(|arena| {
            let mut stream = InputStream::new(input_text);
            let factory = CommonTokenFactory::new(arena);

            let t0 = factory.create(Some(&mut stream), 1, None, 0, 0, 4);
            t0.set_token_index(0);
            let t1 = factory.create(Some(&mut stream), 2, None, 0, 6, 9);
            t1.set_token_index(1);
            let t2 = factory.create(Some(&mut stream), 3, None, 0, 14, 14);
            t2.set_token_index(2);

            assert!(t0.source.line_index.get().is_none());
            assert_eq!(t0.get_text(), "alpha");
            assert_eq!(t1.get_text(), "beta");
            assert_eq!(t2.get_text(), "!");
            assert!(t0.source.line_index.get().is_none());

            assert_eq!((t0.get_line(), t0.get_char_position_in_line()), (1, 0));
            assert!(t0.source.line_index.get().is_some());
            assert_eq!((t1.get_line(), t1.get_char_position_in_line()), (2, 0));
            assert_eq!((t2.get_line(), t2.get_char_position_in_line()), (2, 6));

            let conjured = factory.create_conjured(4, "<missing ';'>".to_owned(), 0, t2);
            assert_eq!(conjured.get_start_index(), -1);
            assert_eq!(conjured.get_stop_index(), -1);
            assert_eq!(conjured.get_text(), "<missing ';'>");
            assert_eq!(
                (conjured.get_line(), conjured.get_char_position_in_line()),
                (2, 6)
            );
            assert_eq!(conjured.to_string(), "[@-1,-1:-1='<missing ';'>',<4>,2:6]");
        });
    }
}
