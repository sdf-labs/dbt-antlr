//! `IntStream` that produces tokens for Parser
use std::borrow::Borrow;
use std::cmp::min;
use std::marker::PhantomData;

use crate::char_stream::InputData;
use crate::int_stream::{IntStream, IterWrapper};
use crate::token::{Token, TOKEN_EOF, TOKEN_INVALID_TYPE};
use crate::token_factory::TokenFactory;
use crate::token_source::TokenSource;
use std::fmt::{Debug, Formatter};

/// An `IntSteam` of `Token`s
///
/// Used as an input for `Parser`s
/// If there is an existing source of tokens, you should implement
/// `TokenSource`, not `TokenStream`
pub trait TokenStream<'input, 'arena, TF>: IntStream
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
{
    /// Lookahead for tokens, same as `IntSteam::la` but return reference to full token
    fn lt(&mut self, k: isize) -> Option<&'arena TF::Tok>;

    /// Returns reference to token at `index`
    ///
    /// FIXME: panics if the underlying stream does not support retrieving the
    /// token at the specified index
    fn get(&self, index: isize) -> &'arena TF::Tok;

    /// Returns the token at the current stream position.
    #[inline]
    fn current_token(&self) -> &'arena TF::Tok {
        self.get(self.index())
    }

    /// Advances a lookahead cursor by one token and returns `(next_cursor, token_type)`.
    ///
    /// The stream position does not change.
    /// `cursor` must be `self.index()` or a value from a previous call while a `mark()` is active.
    #[inline]
    fn next_lookahead(&mut self, cursor: isize) -> (isize, i32) {
        let saved = self.index();
        if saved != cursor {
            self.seek(cursor);
        }
        if self.la(1) != TOKEN_EOF {
            self.consume();
        }
        let next_cursor = self.index();
        let next_type = self.la(1);
        self.seek(saved);
        (next_cursor, next_type)
    }

    /// Token source that produced data for tokens for this stream
    fn get_token_source(&self) -> &dyn TokenSource<'input, 'arena, TF>;

    //    fn set_token_source(&self,source: Box<TokenSource>);

    /// Get combined text of all tokens in this stream
    fn get_all_text(&self) -> String {
        self.get_text_from_interval(0, self.size() - 1)
    }

    /// Get combined text of tokens in start..=stop interval
    fn get_text_from_interval(&self, start: isize, stop: isize) -> String;

    //    fn get_text_from_rule_context(&self,context: RuleContext) -> String;

    /// Get combined text of tokens in between `a` and `b`
    fn get_text_from_tokens<T: Token + ?Sized>(&self, a: &T, b: &T) -> String
    where
        Self: Sized,
    {
        self.get_text_from_interval(a.get_token_index(), b.get_token_index())
    }
}

/// Iterator over tokens in `T`
#[derive(Debug)]
pub struct TokenIter<'a, 'input, 'arena, T, TF>(
    &'a mut T,
    bool,
    PhantomData<fn() -> &'input str>,
    PhantomData<fn() -> &'arena TF::Tok>,
)
where
    'input: 'a,
    T: TokenStream<'input, 'arena, TF>,
    TF: TokenFactory<'input, 'arena> + 'arena;

impl<'a, 'input, 'arena, T, TF> Iterator for TokenIter<'a, 'input, 'arena, T, TF>
where
    'input: 'a,
    T: TokenStream<'input, 'arena, TF>,
    TF: TokenFactory<'input, 'arena> + 'arena,
{
    type Item = &'arena TF::Tok;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 {
            return None;
        }
        let result = self.0.lt(1).unwrap();
        if result.get_token_type() == TOKEN_EOF {
            self.1 = true;
        } else {
            self.0.consume();
        }
        Some(result)
    }
}

/// Token stream that keeps all data in internal Vec
pub struct UnbufferedTokenStream<'input, 'arena, TS, TF>
where
    TS: TokenSource<'input, 'arena, TF>,
    TF: TokenFactory<'input, 'arena> + 'arena,
{
    token_source: TS,
    pub(crate) tokens: Vec<&'arena TF::Tok>,
    pub(crate) current_token_index: isize,
    markers_count: isize,
    pub(crate) p: isize,
    pub(crate) fetched_eof: bool,
    saw_eof: bool,
}

impl<'input, 'arena, TS, TF> Debug for UnbufferedTokenStream<'input, 'arena, TS, TF>
where
    TS: TokenSource<'input, 'arena, TF>,
    TF: TokenFactory<'input, 'arena> + 'arena,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnbufferedTokenStream")
            .field("tokens", &self.tokens)
            .field("current_token_index", &self.current_token_index)
            .field("markers_count", &self.markers_count)
            .field("p(buffer index)", &self.p)
            .finish()
    }
}

impl<'input, 'arena, TS, TF> UnbufferedTokenStream<'input, 'arena, TS, TF>
where
    TS: TokenSource<'input, 'arena, TF>,
    TF: TokenFactory<'input, 'arena> + 'arena,
{
    /// Creates iterator over this token stream
    pub fn iter(&mut self) -> IterWrapper<'_, Self> {
        IterWrapper(self, false)
    }

    /// Creates iterator over tokens in this token stream
    pub fn token_iter(&mut self) -> TokenIter<'_, 'input, 'arena, Self, TF> {
        TokenIter(self, false, PhantomData, PhantomData)
    }

    const DEFAULT_BUFFERED_CAPACITY: usize = 64;

    /// Creates token stream that keeps all tokens inside
    pub fn new_buffered(source: TS) -> Self {
        Self::new_buffered_with_capacity(source, Self::DEFAULT_BUFFERED_CAPACITY)
    }

    /// Creates a buffered token stream with an initial token buffer capacity.
    pub fn new_buffered_with_capacity(source: TS, capacity: usize) -> Self {
        let mut a = UnbufferedTokenStream {
            token_source: source,
            tokens: Vec::with_capacity(capacity),
            current_token_index: 0,
            markers_count: 0,
            p: 0,
            fetched_eof: false,
            saw_eof: false,
        };
        a.mark();
        a
    }

    /// Creates token stream that keeps only tokens required by `mark`
    pub fn new_unbuffered(source: TS) -> Self {
        UnbufferedTokenStream {
            token_source: source,
            tokens: Vec::new(),
            current_token_index: 0,
            markers_count: 0,
            p: 0,
            fetched_eof: false,
            saw_eof: false,
        }
    }

    pub fn get_dfa_string(&self) -> String {
        self.token_source.get_dfa_string()
    }

    fn sync(&mut self, want: isize) {
        let need = (self.p + want - 1) - self.tokens.len() as isize + 1;
        if need > 0 {
            self.fill(need);
        }
    }

    fn get_buffer_start_index(&self) -> isize {
        self.current_token_index - self.p
    }

    pub(crate) fn fill(&mut self, need: isize) -> isize {
        for i in 0..need {
            if self.saw_eof {
                return i;
            }
            let token = self.token_source.next_token();
            token.set_token_index(self.get_buffer_start_index() + self.tokens.len() as isize);
            if token.get_token_type() == TOKEN_EOF {
                self.saw_eof = true;
            }
            self.tokens.push(token);
        }

        need
    }

    #[inline]
    pub(crate) fn fetch_buffered_one(&mut self) -> Option<&'arena TF::Tok> {
        if self.saw_eof {
            return None;
        }
        let token = self.token_source.next_token();
        let idx = self.tokens.len() as isize;
        token.set_token_index(idx);
        let tok: &'arena TF::Tok = &*token;
        if tok.get_token_type() == TOKEN_EOF {
            self.saw_eof = true;
        }
        self.tokens.push(tok);
        Some(tok)
    }
}

impl<'input, 'arena, TS, TF> TokenStream<'input, 'arena, TF>
    for UnbufferedTokenStream<'input, 'arena, TS, TF>
where
    TS: TokenSource<'input, 'arena, TF>,
    TF: TokenFactory<'input, 'arena> + 'arena,
{
    #[inline]
    fn lt(&mut self, i: isize) -> Option<&'arena TF::Tok> {
        if i == -1 {
            let idx = self.p - 1;
            if idx < 0 {
                return None;
            }
            return self.tokens.get(idx as usize).copied();
        }

        self.sync(i);

        let idx = self.p + i - 1;
        if idx < 0 {
            return None;
        }
        self.tokens.get(idx as usize).copied()
    }

    #[inline]
    fn get(&self, index: isize) -> &'arena TF::Tok {
        self.tokens[(index - self.get_buffer_start_index()) as usize]
    }

    #[inline]
    fn next_lookahead(&mut self, cursor: isize) -> (isize, i32) {
        let buf_start = self.get_buffer_start_index();
        let cur_buf = (cursor - buf_start) as usize;
        if cur_buf < self.tokens.len() && self.tokens[cur_buf].get_token_type() == TOKEN_EOF {
            return (cursor, TOKEN_EOF);
        }
        let next_cursor = cursor + 1;
        let next_buf = cur_buf + 1;
        if next_buf >= self.tokens.len() {
            let need = (next_buf - self.tokens.len() + 1) as isize;
            self.fill(need);
        }
        match self.tokens.get(next_buf) {
            Some(tok) => (next_cursor, tok.get_token_type()),
            None => {
                let last = self.tokens.len().saturating_sub(1);
                let last_type = self
                    .tokens
                    .get(last)
                    .map_or(TOKEN_EOF, |t| t.get_token_type());
                (buf_start + last as isize, last_type)
            }
        }
    }

    fn get_token_source(&self) -> &dyn TokenSource<'input, 'arena, TF> {
        &self.token_source
    }

    fn get_text_from_interval(&self, start: isize, stop: isize) -> String {
        //        println!("get_text_from_interval {}..{}",start,stop);
        //        println!("all tokens {:?}",self.tokens.iter().map(|x|x.as_ref().to_owned()).collect::<Vec<OwningToken>>());

        let buffer_start_index = self.get_buffer_start_index();
        let buffer_stop_index = buffer_start_index + self.tokens.len() as isize - 1;
        assert!(
            !(start < buffer_start_index || stop > buffer_stop_index),
            "interval {}..={} not in token buffer window: {}..{}",
            start,
            stop,
            buffer_start_index,
            buffer_stop_index
        );

        let a = start - buffer_start_index;
        let b = stop - buffer_start_index;

        let mut buf = String::new();
        for i in a..(b + 1) {
            let t = self.tokens[i as usize].borrow();
            if t.get_token_type() == TOKEN_EOF {
                break;
            }
            buf.push_str(&t.get_text().to_display());
        }

        buf
    }
}

impl<'input, 'arena, TS, TF> IntStream for UnbufferedTokenStream<'input, 'arena, TS, TF>
where
    TS: TokenSource<'input, 'arena, TF>,
    TF: TokenFactory<'input, 'arena> + 'arena,
{
    #[inline]
    fn consume(&mut self) {
        assert!(!self.fetched_eof, "cannot consume EOF");
        if self.la(1) == TOKEN_EOF {
            self.fetched_eof = true;
        }

        if self.p == self.tokens.len() as isize && self.markers_count == 0 {
            self.tokens.clear();
            self.p = -1;
        }

        self.p += 1;
        self.current_token_index += 1;

        self.sync(1);
        // Ok(())
    }

    #[inline]
    fn la(&mut self, i: isize) -> i32 {
        self.lt(i)
            .map_or(TOKEN_INVALID_TYPE, |t| t.borrow().get_token_type())
    }

    #[inline]
    fn mark(&mut self) -> isize {
        self.markers_count += 1;
        -self.markers_count
    }

    #[inline]
    fn release(&mut self, marker: isize) {
        assert_eq!(marker, -self.markers_count);

        self.markers_count -= 1;
        if self.markers_count == 0 && self.p > 0 {
            self.tokens.copy_within(self.p as usize.., 0);
            self.tokens.truncate(self.tokens.len() - self.p as usize);

            self.p = 0;
        }
    }

    #[inline(always)]
    fn index(&self) -> isize {
        self.current_token_index
    }

    #[inline]
    fn seek(&mut self, mut index: isize) {
        if self.current_token_index == index {
            return;
        }
        if index > self.current_token_index {
            self.sync(index - self.current_token_index);
            index = min(index, self.get_buffer_start_index() + self.size() + 1);
        }
        let i = index - self.get_buffer_start_index();
        assert!(!(i < 0 || i >= self.tokens.len() as isize),);

        self.p = i;
        self.current_token_index = index;
    }

    #[inline(always)]
    fn size(&self) -> isize {
        self.tokens.len() as isize
    }

    fn get_source_name(&self) -> String {
        self.token_source.get_source_name()
    }
}
