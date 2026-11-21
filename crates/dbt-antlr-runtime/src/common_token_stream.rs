//! Channel based `TokenStream`

use crate::int_stream::{IntStream, IterWrapper, EOF};
use crate::token::{Token, TOKEN_DEFAULT_CHANNEL, TOKEN_INVALID_TYPE};
use crate::token_factory::TokenFactory;
use crate::token_source::TokenSource;
use crate::token_stream::{TokenStream, UnbufferedTokenStream};

/// Default token stream that skips token that not correspond to current channel.
#[derive(Debug)]
pub struct CommonTokenStream<'input, 'arena, TS, TF>
where
    TS: TokenSource<'input, 'arena, TF>,
    TF: TokenFactory<'input, 'arena> + 'arena,
{
    base: UnbufferedTokenStream<'input, 'arena, TS, TF>,
    channel: i32,
    cur_type: i32,
    cur_on_channel: bool,
    cur_tok: &'arena TF::Tok,
    prev_tok: Option<&'arena TF::Tok>,
    prev_p: isize,
}

impl<'input, 'arena, TS, TF> CommonTokenStream<'input, 'arena, TS, TF>
where
    TS: TokenSource<'input, 'arena, TF>,
    TF: TokenFactory<'input, 'arena> + 'arena,
{
    pub fn base(&self) -> &UnbufferedTokenStream<'input, 'arena, TS, TF> {
        &self.base
    }
}

impl<'input, 'arena, TS, TF> IntStream for CommonTokenStream<'input, 'arena, TS, TF>
where
    TS: TokenSource<'input, 'arena, TF>,
    TF: TokenFactory<'input, 'arena> + 'arena,
{
    #[inline]
    fn consume(&mut self) {
        if self.cur_type == EOF {
            assert!(!self.base.fetched_eof, "cannot consume EOF");
            self.base.fetched_eof = true;
            return;
        }
        let old_p = self.base.p;
        let old_tok = self.cur_tok;
        let old_on_channel = self.cur_on_channel;

        let (next, tok, ttype, on_channel) = self.advance_on_channel((old_p + 1) as usize);
        self.base.p = next;
        self.base.current_token_index = next;
        self.cur_tok = tok;
        self.cur_type = ttype;
        self.cur_on_channel = on_channel;
        if old_on_channel {
            self.prev_tok = Some(old_tok);
            self.prev_p = next;
        } else if self.prev_p == old_p {
            self.prev_p = next;
        }
    }

    #[inline(always)]
    fn la(&mut self, i: isize) -> i32 {
        if i == 1 {
            return self.cur_type;
        }
        self.lt(i)
            .map_or(TOKEN_INVALID_TYPE, |t| t.get_token_type())
    }

    #[inline(always)]
    fn mark(&mut self) -> isize {
        0
    }

    #[inline(always)]
    fn release(&mut self, _marker: isize) {}

    #[inline(always)]
    fn index(&self) -> isize {
        self.base.index()
    }

    #[inline]
    fn seek(&mut self, index: isize) {
        if self.base.p == index {
            return;
        }
        if index >= self.base.tokens.len() as isize {
            self.sync(index);
        }
        assert!(index >= 0 && (index as usize) < self.base.tokens.len());
        let tok = self.base.tokens[index as usize];
        self.base.p = index;
        self.base.current_token_index = index;
        self.cur_tok = tok;
        self.cur_type = tok.get_token_type();
        self.cur_on_channel = tok.get_channel() == self.channel;
    }

    #[inline(always)]
    fn size(&self) -> isize {
        self.base.size()
    }

    fn get_source_name(&self) -> String {
        self.base.get_source_name()
    }
}

impl<'input, 'arena, TS, TF> TokenStream<'input, 'arena, TF>
    for CommonTokenStream<'input, 'arena, TS, TF>
where
    TS: TokenSource<'input, 'arena, TF>,
    TF: TokenFactory<'input, 'arena> + 'arena,
{
    #[inline(always)]
    fn lt(&mut self, k: isize) -> Option<&'arena TF::Tok> {
        if k == 1 {
            return Some(self.cur_tok);
        }
        if k == -1 {
            if self.prev_p == self.base.p {
                return self.prev_tok;
            }
            return self.lb_slow(1);
        }
        if k == 0 {
            return None;
        }
        if k < 0 {
            return self.lb_slow(-k);
        }
        self.lt_ahead(k)
    }

    #[inline]
    fn get(&self, index: isize) -> &'arena TF::Tok {
        self.base.tokens[index as usize]
    }

    #[inline(always)]
    fn current_token(&self) -> &'arena TF::Tok {
        self.cur_tok
    }

    #[inline]
    fn next_lookahead(&mut self, cursor: isize) -> (isize, i32) {
        let idx = cursor as usize;
        if idx < self.base.tokens.len() && self.base.tokens[idx].get_token_type() == EOF {
            return (cursor, EOF);
        }
        let (next, _tok, ttype, _on_channel) = self.advance_on_channel(idx + 1);
        (next, ttype)
    }

    fn get_token_source(&self) -> &dyn TokenSource<'input, 'arena, TF> {
        self.base.get_token_source()
    }

    fn get_text_from_interval(&self, start: isize, stop: isize) -> String {
        self.base.get_text_from_interval(start, stop)
    }
}

impl<'input, 'arena, TS, TF> CommonTokenStream<'input, 'arena, TS, TF>
where
    TS: TokenSource<'input, 'arena, TF>,
    TF: TokenFactory<'input, 'arena> + 'arena,
{
    /// Creates CommonTokenStream that produces tokens from `TOKEN_DEFAULT_CHANNEL`
    pub fn new(lexer: TS) -> CommonTokenStream<'input, 'arena, TS, TF> {
        Self::with_channel(lexer, TOKEN_DEFAULT_CHANNEL)
    }

    /// Creates CommonTokenStream that produces tokens from `channel`
    pub fn with_channel(lexer: TS, channel: i32) -> CommonTokenStream<'input, 'arena, TS, TF> {
        let mut base = UnbufferedTokenStream::new_buffered(lexer);
        let (p, cur_tok, cur_type, cur_on_channel) = loop {
            let tok = base
                .fetch_buffered_one()
                .expect("token source must yield at least EOF");
            let on_channel = tok.get_channel() == channel;
            let ttype = tok.get_token_type();
            if on_channel || ttype == EOF {
                break ((base.tokens.len() - 1) as isize, tok, ttype, on_channel);
            }
        };
        base.p = p;
        base.current_token_index = p;
        CommonTokenStream {
            base,
            channel,
            cur_type,
            cur_on_channel,
            cur_tok,
            prev_tok: None,
            prev_p: p,
        }
    }

    pub fn get_dfa_string(&self) -> String {
        self.base.get_dfa_string()
    }

    #[inline]
    fn advance_on_channel(&mut self, mut i: usize) -> (isize, &'arena TF::Tok, i32, bool) {
        let tokens = &self.base.tokens;
        let len = tokens.len();
        while i < len {
            let tok = tokens[i];
            let on_channel = tok.get_channel() == self.channel;
            let ttype = tok.get_token_type();
            if on_channel || ttype == EOF {
                return (i as isize, tok, ttype, on_channel);
            }
            i += 1;
        }
        self.advance_on_channel_fetch()
    }

    #[inline]
    fn advance_on_channel_fetch(&mut self) -> (isize, &'arena TF::Tok, i32, bool) {
        while let Some(tok) = self.base.fetch_buffered_one() {
            let on_channel = tok.get_channel() == self.channel;
            let ttype = tok.get_token_type();
            if on_channel || ttype == EOF {
                return (
                    (self.base.tokens.len() - 1) as isize,
                    tok,
                    ttype,
                    on_channel,
                );
            }
        }
        let last = self.base.tokens.len() - 1;
        let tok = self.base.tokens[last];
        (
            last as isize,
            tok,
            tok.get_token_type(),
            tok.get_channel() == self.channel,
        )
    }

    fn lt_ahead(&mut self, k: isize) -> Option<&'arena TF::Tok> {
        let mut i = self.base.p;
        let mut tok = self.cur_tok;
        if self.cur_type == EOF {
            return Some(tok);
        }
        let mut n = 1;
        while n < k {
            let (next, next_tok, ttype, _) = self.advance_on_channel((i + 1) as usize);
            i = next;
            tok = next_tok;
            if ttype == EOF {
                break;
            }
            n += 1;
        }
        Some(tok)
    }

    /// Restarts this token stream
    pub fn reset(&mut self) {
        self.base.fetched_eof = false;
        let (p, tok, ttype, on_channel) = self.advance_on_channel(0);
        self.base.p = p;
        self.base.current_token_index = p;
        self.cur_tok = tok;
        self.cur_type = ttype;
        self.cur_on_channel = on_channel;
        self.prev_tok = None;
        self.prev_p = p;
    }

    /// Creates iterator over this token stream
    pub fn iter(&mut self) -> IterWrapper<'_, Self> {
        IterWrapper(self, false)
    }

    fn sync(&mut self, i: isize) -> bool {
        let need = i - self.size() + 1;
        if need > 0 {
            let fetched = self.base.fill(need);
            return fetched >= need;
        }

        true
    }

    #[cold]
    fn lb_slow(&mut self, k: isize) -> Option<&'arena TF::Tok> {
        let p = self.base.p;
        if p < k {
            if k == 1 {
                self.prev_p = p;
                self.prev_tok = None;
            }
            return None;
        }
        let tokens = &self.base.tokens;
        let mut i = p;
        let mut n = 0;
        while n < k {
            i -= 1;
            loop {
                if i < 0 {
                    if k == 1 {
                        self.prev_p = p;
                        self.prev_tok = None;
                    }
                    return None;
                }
                if tokens[i as usize].get_channel() == self.channel {
                    break;
                }
                i -= 1;
            }
            n += 1;
        }
        let tok = tokens.get(i as usize).copied();
        if k == 1 {
            self.prev_p = p;
            self.prev_tok = tok;
        }
        tok
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{TOKEN_DEFAULT_CHANNEL, TOKEN_HIDDEN_CHANNEL};
    use crate::token_factory::OwningTokenFactory;
    use crate::Arena;

    struct SliceTokenSource<'arena> {
        specs: Vec<(i32, i32, &'static str)>,
        pos: usize,
        factory: OwningTokenFactory<'arena>,
    }

    impl<'arena> SliceTokenSource<'arena> {
        fn new(arena: &'arena Arena, specs: Vec<(i32, i32, &'static str)>) -> Self {
            Self {
                specs,
                pos: 0,
                factory: OwningTokenFactory::new(arena),
            }
        }
    }

    impl<'input, 'arena> TokenSource<'input, 'arena, OwningTokenFactory<'arena>>
        for SliceTokenSource<'arena>
    where
        'input: 'arena,
    {
        fn next_token(&mut self) -> &'arena mut crate::token::OwningToken {
            let (ttype, channel, text) = if self.pos < self.specs.len() {
                let spec = self.specs[self.pos];
                self.pos += 1;
                spec
            } else {
                (EOF, TOKEN_DEFAULT_CHANNEL, "<EOF>")
            };
            self.factory.create::<crate::InputStream<'input>>(
                None,
                ttype,
                Some(text.to_owned()),
                channel,
                -1,
                -1,
            )
        }

        fn get_input_stream(&mut self) -> Option<&mut dyn IntStream> {
            None
        }

        fn get_source_name(&self) -> String {
            "test".to_owned()
        }

        fn get_token_factory(&self) -> &OwningTokenFactory<'arena> {
            &self.factory
        }

        fn get_dfa_string(&self) -> String {
            String::new()
        }
    }

    #[test]
    fn leading_hidden_tokens_and_lookahead() {
        Arena::with(|arena| {
            let src = SliceTokenSource::new(
                arena,
                vec![
                    (10, TOKEN_HIDDEN_CHANNEL, " "),
                    (11, TOKEN_HIDDEN_CHANNEL, "/*c*/"),
                    (1, TOKEN_DEFAULT_CHANNEL, "SELECT"),
                    (10, TOKEN_HIDDEN_CHANNEL, " "),
                    (2, TOKEN_DEFAULT_CHANNEL, "a"),
                    (10, TOKEN_HIDDEN_CHANNEL, " "),
                    (3, TOKEN_DEFAULT_CHANNEL, ";"),
                ],
            );
            let mut stream = CommonTokenStream::new(src);

            assert_eq!(stream.index(), 2);
            assert_eq!(stream.la(1), 1);
            assert_eq!(stream.current_token().get_text(), "SELECT");
            assert_eq!(stream.lt(1).unwrap().get_text(), "SELECT");
            assert!(stream.lt(0).is_none());
            assert!(stream.lt(-1).is_none());
            assert!(stream.lt(-1).is_none());
            assert!(stream.lt(-2).is_none());
            assert_eq!(stream.la(2), 2);
            assert_eq!(stream.la(3), 3);
            assert_eq!(stream.la(4), EOF);
            assert_eq!(stream.la(5), EOF);

            stream.consume();
            assert_eq!(stream.index(), 4);
            assert_eq!(stream.la(1), 2);
            assert_eq!(stream.lt(-1).unwrap().get_text(), "SELECT");
            assert_eq!(stream.lt(-1).unwrap().get_text(), "SELECT");
            assert!(stream.lt(-2).is_none());

            stream.consume();
            assert_eq!(stream.index(), 6);
            assert_eq!(stream.la(1), 3);
            assert_eq!(stream.lt(-1).unwrap().get_text(), "a");
            assert_eq!(stream.lt(-2).unwrap().get_text(), "SELECT");
            assert!(stream.lt(-3).is_none());

            stream.consume();
            assert_eq!(stream.index(), 7);
            assert_eq!(stream.la(1), EOF);
            assert_eq!(stream.lt(-1).unwrap().get_text(), ";");
            assert_eq!(stream.lt(-2).unwrap().get_text(), "a");

            stream.consume();
            assert_eq!(stream.index(), 7);
            assert_eq!(stream.la(1), EOF);
            assert_eq!(stream.lt(-1).unwrap().get_text(), ";");
        });
    }

    #[test]
    #[should_panic(expected = "cannot consume EOF")]
    fn double_consume_eof_panics() {
        Arena::with(|arena| {
            let src = SliceTokenSource::new(arena, vec![]);
            let mut stream = CommonTokenStream::new(src);
            assert_eq!(stream.la(1), EOF);
            stream.consume();
            stream.consume();
        });
    }

    #[test]
    fn seek_and_off_channel_consume_keep_prev_exact() {
        Arena::with(|arena| {
            let src = SliceTokenSource::new(
                arena,
                vec![
                    (1, TOKEN_DEFAULT_CHANNEL, "SELECT"),
                    (10, TOKEN_HIDDEN_CHANNEL, " "),
                    (2, TOKEN_DEFAULT_CHANNEL, "a"),
                    (10, TOKEN_HIDDEN_CHANNEL, " "),
                    (3, TOKEN_DEFAULT_CHANNEL, "b"),
                ],
            );
            let mut stream = CommonTokenStream::new(src);
            stream.consume();
            stream.consume();
            assert_eq!(stream.index(), 4);
            assert_eq!(stream.lt(-1).unwrap().get_text(), "a");

            stream.seek(2);
            assert_eq!(stream.la(1), 2);
            assert_eq!(stream.current_token().get_text(), "a");
            assert_eq!(stream.lt(-1).unwrap().get_text(), "SELECT");
            assert_eq!(stream.lt(-1).unwrap().get_text(), "SELECT");

            stream.seek(2);
            assert_eq!(stream.lt(-1).unwrap().get_text(), "SELECT");

            stream.seek(1);
            assert_eq!(stream.la(1), 10);
            assert_eq!(stream.lt(-1).unwrap().get_text(), "SELECT");
            stream.consume();
            assert_eq!(stream.index(), 2);
            assert_eq!(stream.lt(-1).unwrap().get_text(), "SELECT");

            stream.seek(4);
            stream.seek(1);
            stream.consume();
            assert_eq!(stream.index(), 2);
            assert_eq!(stream.lt(-1).unwrap().get_text(), "SELECT");

            stream.seek(5);
            assert_eq!(stream.la(1), EOF);
            assert_eq!(stream.lt(-1).unwrap().get_text(), "b");
        });
    }

    #[test]
    fn empty_and_all_hidden_and_custom_channel() {
        Arena::with(|arena| {
            let empty = CommonTokenStream::new(SliceTokenSource::new(arena, vec![]));
            assert_eq!(empty.index(), 0);
            assert_eq!(empty.current_token().get_token_type(), EOF);

            let mut all_hidden = CommonTokenStream::new(SliceTokenSource::new(
                arena,
                vec![
                    (10, TOKEN_HIDDEN_CHANNEL, " "),
                    (11, TOKEN_HIDDEN_CHANNEL, "\n"),
                ],
            ));
            assert_eq!(all_hidden.index(), 2);
            assert_eq!(all_hidden.la(1), EOF);
            assert!(all_hidden.lt(-1).is_none());
            assert!(all_hidden.lt(-2).is_none());

            let mut hidden = CommonTokenStream::with_channel(
                SliceTokenSource::new(
                    arena,
                    vec![
                        (1, TOKEN_DEFAULT_CHANNEL, "a"),
                        (10, TOKEN_HIDDEN_CHANNEL, " "),
                        (2, TOKEN_DEFAULT_CHANNEL, "b"),
                        (11, TOKEN_HIDDEN_CHANNEL, "/*c*/"),
                        (3, TOKEN_DEFAULT_CHANNEL, "c"),
                    ],
                ),
                TOKEN_HIDDEN_CHANNEL,
            );
            assert_eq!(hidden.index(), 1);
            assert_eq!(hidden.la(1), 10);
            assert!(hidden.lt(-1).is_none());
            assert_eq!(hidden.la(2), 11);
            assert_eq!(hidden.la(3), EOF);
            hidden.consume();
            assert_eq!(hidden.la(1), 11);
            assert_eq!(hidden.lt(-1).unwrap().get_token_type(), 10);
            hidden.consume();
            assert_eq!(hidden.la(1), EOF);
            assert_eq!(hidden.lt(-1).unwrap().get_token_type(), 11);
            assert_eq!(hidden.lt(-2).unwrap().get_token_type(), 10);

            let mut no_hidden = CommonTokenStream::with_channel(
                SliceTokenSource::new(arena, vec![(1, TOKEN_DEFAULT_CHANNEL, "a")]),
                TOKEN_HIDDEN_CHANNEL,
            );
            assert_eq!(no_hidden.la(1), EOF);
            assert!(no_hidden.lt(-1).is_none());
        });
    }

    #[test]
    fn reset_after_eof_does_not_duplicate_eof() {
        Arena::with(|arena| {
            let src = SliceTokenSource::new(
                arena,
                vec![
                    (10, TOKEN_HIDDEN_CHANNEL, " "),
                    (1, TOKEN_DEFAULT_CHANNEL, "a"),
                    (10, TOKEN_HIDDEN_CHANNEL, " "),
                    (2, TOKEN_DEFAULT_CHANNEL, "b"),
                ],
            );
            let mut stream = CommonTokenStream::new(src);
            assert_eq!(stream.index(), 1);
            stream.consume();
            stream.consume();
            assert_eq!(stream.la(1), EOF);
            let total_size = stream.size();
            stream.consume();

            stream.reset();
            assert_eq!(stream.index(), 1);
            assert_eq!(stream.la(1), 1);
            assert!(stream.lt(-1).is_none());
            stream.consume();
            assert_eq!(stream.la(1), 2);
            stream.consume();
            assert_eq!(stream.la(1), EOF);
            assert_eq!(stream.size(), total_size);
            stream.consume();
            assert_eq!(stream.size(), total_size);
        });
    }

    #[test]
    fn next_lookahead_preserves_position_and_prev_cache() {
        Arena::with(|arena| {
            let src = SliceTokenSource::new(
                arena,
                vec![
                    (1, TOKEN_DEFAULT_CHANNEL, "a"),
                    (10, TOKEN_HIDDEN_CHANNEL, " "),
                    (2, TOKEN_DEFAULT_CHANNEL, "b"),
                    (10, TOKEN_HIDDEN_CHANNEL, " "),
                    (3, TOKEN_DEFAULT_CHANNEL, "c"),
                ],
            );
            let mut stream = CommonTokenStream::new(src);
            stream.consume();
            assert_eq!(stream.index(), 2);
            assert_eq!(stream.lt(-1).unwrap().get_text(), "a");

            let (c1, t1) = stream.next_lookahead(stream.index());
            assert_eq!((c1, t1), (4, 3));
            let (c2, t2) = stream.next_lookahead(c1);
            assert_eq!((c2, t2), (5, EOF));
            let (c3, t3) = stream.next_lookahead(c2);
            assert_eq!((c3, t3), (5, EOF));

            assert_eq!(stream.index(), 2);
            assert_eq!(stream.la(1), 2);
            assert_eq!(stream.current_token().get_text(), "b");
            assert_eq!(stream.lt(-1).unwrap().get_text(), "a");

            let (b1, bt1) = stream.next_lookahead(stream.index());
            assert_eq!((b1, bt1), (4, 3));
            assert_eq!(stream.index(), 2);
        });
    }
}
