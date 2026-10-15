//! How Lexer should produce tokens
use std::borrow::Cow;
use std::cell::Cell;
use std::fmt::Debug;

use crate::char_stream::CharStream;
use crate::token::{CommonToken, OwningToken, SourceFallback, SourceInfo, Token, TOKEN_EOF};
use crate::Arena;

#[inline]
pub fn invalid() -> &'static CommonToken<'static> {
    Default::default()
}

/// Trait for creating tokens.
pub trait TokenFactory<'input, 'arena>: Debug + Sized
where
    'input: 'arena,
{
    /// Type of tokens emitted by this factory.
    type Tok: Token + 'input;

    fn new(arena: &'arena Arena) -> Self;

    /// Creates token either from `source` or from pure data in `text`.
    /// Either `source` or `text` are not None.
    fn create<T>(
        &self,
        source: Option<&mut T>,
        ttype: i32,
        text: Option<String>,
        channel: i32,
        start: isize,
        stop: isize,
    ) -> &'arena mut Self::Tok
    where
        T: CharStream<'input> + ?Sized;

    /// Creates a conjured token (e.g. `<missing ...>` during parser error recovery)
    /// that reports `start = -1, stop = -1` and inherits its line/column from `reference_token`.
    fn create_conjured(
        &self,
        ttype: i32,
        text: String,
        channel: i32,
        reference_token: &dyn Token,
    ) -> &'arena mut Self::Tok;
}

/// Default token factory
#[derive(Debug)]
pub struct CommonTokenFactory<'input, 'arena> {
    arena: &'arena Arena,
    source_info: Cell<Option<&'input SourceInfo<'input>>>,
}

#[cold]
#[inline(never)]
fn advance_to_char_boundary(s: &str, mut index: usize) -> usize {
    let len = s.len();
    while index < len && !s.is_char_boundary(index) {
        index += 1;
    }
    index.min(len)
}

impl<'input, 'arena> CommonTokenFactory<'input, 'arena>
where
    'input: 'arena,
{
    /// Creates new `CommonTokenFactory`
    pub fn new(arena: &'arena Arena) -> Self {
        Self {
            arena,
            source_info: Cell::new(None),
        }
    }

    #[cold]
    #[inline(never)]
    fn init_source_info<T>(&self, x: &T) -> &'input SourceInfo<'input>
    where
        T: CharStream<'input> + ?Sized,
    {
        let raw_input: &'input str = match x.get_data_str() {
            Some(s) => s,
            None => {
                let sz = x.size();
                if sz <= 0 {
                    ""
                } else {
                    match x.get_text(0, sz) {
                        Cow::Borrowed(t) => t,
                        Cow::Owned(t) => unsafe {
                            std::mem::transmute::<&'arena str, &'input str>(
                                self.arena.alloc_string(t),
                            )
                        },
                    }
                }
            }
        };
        let info = self
            .arena
            .alloc_source_info(SourceInfo::new_stream(raw_input));
        let info_input: &'input SourceInfo<'input> = unsafe {
            std::mem::transmute::<&'arena SourceInfo<'input>, &'input SourceInfo<'input>>(info)
        };
        self.source_info.set(Some(info_input));
        info_input
    }

    #[cold]
    #[inline(never)]
    fn create_slow<T>(
        &self,
        source: Option<&mut T>,
        ttype: i32,
        text: Option<String>,
        channel: i32,
        start: isize,
        stop: isize,
    ) -> &'arena mut CommonToken<'input>
    where
        T: CharStream<'input> + ?Sized,
    {
        let stream_info = match (self.source_info.get(), source) {
            (Some(info), _) => Some(info),
            (None, Some(x)) => Some(self.init_source_info(x)),
            (None, None) => None,
        };
        let text_str: &'input str = match text {
            Some(t) => unsafe {
                std::mem::transmute::<&'arena str, &'input str>(self.arena.alloc_string(t))
            },
            None => "",
        };
        let override_info = self.arena.alloc_in_tokens(SourceInfo::new_override(
            text_str,
            stream_info.map(SourceFallback::Stream),
        ));
        let override_info: &'input SourceInfo<'input> = unsafe {
            std::mem::transmute::<&'arena SourceInfo<'input>, &'input SourceInfo<'input>>(
                override_info,
            )
        };
        let pos = if start >= 0 && stream_info.is_some() {
            start as u32
        } else {
            u32::MAX
        };
        self.arena.alloc_token(CommonToken::new_slice(
            override_info,
            ttype,
            channel,
            start as i32,
            stop as i32,
            -1,
            u32::MAX,
            pos,
        ))
    }
}

impl<'input, 'arena> TokenFactory<'input, 'arena> for CommonTokenFactory<'input, 'arena>
where
    'input: 'arena,
{
    type Tok = CommonToken<'input>;

    fn new(arena: &'arena Arena) -> Self {
        Self::new(arena)
    }

    #[inline]
    fn create<T>(
        &self,
        source: Option<&mut T>,
        ttype: i32,
        text: Option<String>,
        channel: i32,
        start: isize,
        stop: isize,
    ) -> &'arena mut Self::Tok
    where
        T: CharStream<'input> + ?Sized,
    {
        match (text, source) {
            (None, Some(x)) => {
                let source_info = match self.source_info.get() {
                    Some(info) => info,
                    None => self.init_source_info(x),
                };
                let input_len = source_info.input.len();
                let text_end = if ttype == TOKEN_EOF
                    || stop >= input_len as isize
                    || start >= input_len as isize
                {
                    u32::MAX - 1
                } else if stop < start || start < 0 {
                    start.max(0) as u32
                } else {
                    let end = (stop + 1) as usize;
                    if end < input_len {
                        if source_info.input.is_char_boundary(end) {
                            end as u32
                        } else {
                            advance_to_char_boundary(source_info.input, end) as u32
                        }
                    } else {
                        input_len as u32
                    }
                };
                self.arena.alloc_token(CommonToken::new_slice(
                    source_info,
                    ttype,
                    channel,
                    start as i32,
                    stop as i32,
                    -1,
                    text_end,
                    start.max(0) as u32,
                ))
            }
            (text, source) => self.create_slow(source, ttype, text, channel, start, stop),
        }
    }

    #[cold]
    fn create_conjured(
        &self,
        ttype: i32,
        text: String,
        channel: i32,
        reference_token: &dyn Token,
    ) -> &'arena mut Self::Tok {
        let text_str: &'input str = unsafe {
            std::mem::transmute::<&'arena str, &'input str>(self.arena.alloc_string(text))
        };
        let (fallback, pos) = match (self.source_info.get(), reference_token.get_pos_offset()) {
            (Some(stream_info), Some(offset)) => {
                (Some(SourceFallback::Stream(stream_info)), offset)
            }
            _ => (
                Some(SourceFallback::Fixed {
                    line: reference_token.get_line(),
                    column: reference_token.get_char_position_in_line(),
                }),
                0,
            ),
        };
        let override_info = self
            .arena
            .alloc_in_tokens(SourceInfo::new_override(text_str, fallback));
        let override_info: &'input SourceInfo<'input> = unsafe {
            std::mem::transmute::<&'arena SourceInfo<'input>, &'input SourceInfo<'input>>(
                override_info,
            )
        };
        self.arena.alloc_token(CommonToken::new_slice(
            override_info,
            ttype,
            channel,
            -1,
            -1,
            -1,
            u32::MAX,
            pos,
        ))
    }
}

/// Token factory that creates `OwningToken`s
#[derive(Debug)]
pub struct OwningTokenFactory<'arena>(CommonTokenFactory<'static, 'arena>);

impl<'input, 'arena> TokenFactory<'input, 'arena> for OwningTokenFactory<'arena>
where
    'input: 'arena,
{
    type Tok = OwningToken;

    fn new(arena: &'arena Arena) -> Self {
        Self(CommonTokenFactory::new(arena))
    }

    #[inline]
    fn create<T>(
        &self,
        source: Option<&mut T>,
        ttype: i32,
        text: Option<String>,
        channel: i32,
        start: isize,
        stop: isize,
    ) -> &'arena mut Self::Tok
    where
        T: CharStream<'input> + ?Sized,
    {
        let (line, column) = match source {
            Some(ref x) => {
                let offset = start.max(0);
                (x.get_line(offset), x.get_char_position_in_line(offset))
            }
            None => (1, 0),
        };
        let text = match (text, source) {
            (Some(t), _) => t,
            (None, Some(x)) => {
                if stop >= x.size() || start >= x.size() {
                    "<EOF>".to_owned()
                } else {
                    x.get_text(start, stop).to_string()
                }
            }
            _ => "".to_owned(),
        };
        self.0.arena.alloc_token(OwningToken::new(
            ttype,
            channel,
            start as i32,
            stop as i32,
            -1,
            line,
            column,
            text,
        ))
    }

    #[cold]
    fn create_conjured(
        &self,
        ttype: i32,
        text: String,
        channel: i32,
        reference_token: &dyn Token,
    ) -> &'arena mut Self::Tok {
        self.0.arena.alloc_token(OwningToken::new(
            ttype,
            channel,
            -1,
            -1,
            -1,
            reference_token.get_line(),
            reference_token.get_char_position_in_line(),
            text,
        ))
    }
}
