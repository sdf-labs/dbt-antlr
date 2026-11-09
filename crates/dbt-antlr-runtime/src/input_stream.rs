//! Input to lexer
use std::borrow::Cow;

use crate::char_stream::{CharStream, InputData};
use crate::int_stream::IntStream;

/// Default rust target input stream.
///
/// Since Rust uses UTF-8 format which does not support indexing by char,
/// `InputStream<&str>` has slightly different index behavior in compare to java runtime when there are
/// non-ASCII unicode characters.
/// If you need it to generate exactly the same indexes as Java runtime, you have to use `CodePoint8/16/32BitCharStream`,
/// which does not use rusts native `str` type, so it would do additional conversions and allocations along the way.
#[derive(Debug)]
pub struct InputStream<'input> {
    data_raw: &'input str,
    index: isize,
}

impl<'input> CharStream<'input> for InputStream<'input> {
    #[inline]
    fn get_text(&self, start: isize, stop: isize) -> Cow<'input, str> {
        self.get_text_inner(start, stop).into()
    }

    /// Raw byte view of the whole input from `index` on, for the walker's
    /// vectorized run scans.
    #[inline]
    fn bytes_at(&self, index: isize) -> Option<&'input [u8]> {
        self.data_raw.as_bytes().get(index as usize..)
    }

    #[inline]
    fn get_data_str(&self) -> Option<&'input str> {
        Some(self.data_raw)
    }

    #[inline(always)]
    fn is_at_eof(&mut self) -> bool {
        (self.index as usize) >= self.data_raw.len()
    }

    #[inline]
    fn get_line(&self, byte_offset: isize) -> u32 {
        let bytes = self.data_raw.as_bytes();
        let len = (byte_offset.max(0) as usize).min(bytes.len());
        1 + memchr::memchr_iter(b'\n', &bytes[..len]).count() as u32
    }

    #[inline]
    fn get_char_position_in_line(&self, byte_offset: isize) -> i32 {
        let bytes = self.data_raw.as_bytes();
        let len = (byte_offset.max(0) as usize).min(bytes.len());
        let line_start = memchr::memrchr(b'\n', &bytes[..len]).map_or(0, |i| i + 1);
        bytes[line_start..len]
            .iter()
            .filter(|&&b| (b as i8) >= -0x40)
            .count() as i32
    }
}

impl<'input> InputStream<'input> {
    fn get_text_inner(&self, start: isize, stop: isize) -> &'input str {
        let start = start as usize;
        let stop = self.data_raw.offset(stop, 1).unwrap_or(stop) as usize;

        if stop < self.data_raw.len() {
            &self.data_raw[start..stop]
        } else {
            &self.data_raw[start..]
        }
    }

    /// Creates new `InputStream` over borrowed data
    #[inline]
    pub fn new(data_raw: &'input str) -> Self {
        Self { data_raw, index: 0 }
    }
    /// Resets input stream to start from the beginning of this slice
    #[inline]
    pub fn reset(&mut self) {
        self.index = 0
    }

    #[inline]
    fn item_at(&self, index: isize) -> Option<i32> {
        let &b = self.data_raw.as_bytes().get(index as usize)?;
        if b < 0x80 {
            Some(b as i32)
        } else {
            self.data_raw
                .get(index as usize..)
                .and_then(|it| it.chars().next())
                .map(|it| it as i32)
        }
    }
}

impl IntStream for InputStream<'_> {
    #[inline]
    fn consume(&mut self) {
        if let Some(index) = self.data_raw.offset(self.index, 1) {
            self.index = index;
            // self.current = self.data_raw.deref().item(index).unwrap_or(TOKEN_EOF);
            // Ok(())
        } else {
            panic!("cannot consume EOF");
        }
    }

    #[inline]
    fn la(&mut self, mut offset: isize) -> i32 {
        if offset == 1 {
            return self.item_at(self.index).unwrap_or(crate::int_stream::EOF);
        }
        assert!(offset != 0, "should not be called with offset 0");
        if offset < 0 {
            offset += 1; // e.g., translate LA(-1) to use offset i=0; then data[p+0-1]
        }

        self.data_raw
            .offset(self.index, offset - 1)
            .and_then(|index| self.data_raw.item(index))
            .unwrap_or(crate::int_stream::EOF)
    }

    #[inline]
    fn mark(&mut self) -> isize {
        -1
    }

    #[inline]
    fn release(&mut self, _marker: isize) {}

    #[inline]
    fn index(&self) -> isize {
        self.index
    }

    #[inline]
    fn seek(&mut self, index: isize) {
        self.index = index
    }

    #[inline]
    fn size(&self) -> isize {
        self.data_raw.len() as isize
    }

    fn get_source_name(&self) -> String {
        "<empty>".to_owned()
    }
}

#[cfg(test)]
mod test {
    use crate::{char_stream::CharStream, int_stream::EOF};

    use super::InputStream;

    #[test]
    fn test_str_input_stream() {
        let mut input = InputStream::new("V1は3");
        let input = &mut input as &mut dyn CharStream;
        assert_eq!(input.la(1), 'V' as i32);
        assert_eq!(input.index(), 0);
        input.consume();
        assert_eq!(input.la(1), '1' as i32);
        assert_eq!(input.la(-1), 'V' as i32);
        assert_eq!(input.index(), 1);
        input.consume();
        assert_eq!(input.la(1), 0x306F);
        assert_eq!(input.index(), 2);
        input.consume();
        assert_eq!(input.index(), 5);
        assert_eq!(input.la(-2), '1' as i32);
        assert_eq!(input.la(2), EOF);
        assert_eq!(input.get_text(1, 1), "1");
        assert_eq!(input.get_text(1, 2), "1は");
        assert_eq!(input.get_text(2, 2), "は");
        assert_eq!(input.get_text(2, 5), "は3");
        assert_eq!(input.get_text(5, 5), "3");
    }

    // #[test]
    // fn test_byte_input_stream() {
    //     let mut input = InputStream::new(&b"V\xaa\xbb"[..]);
    //     assert_eq!(input.la(1), 'V' as i32);
    //     input.seek(2);
    //     assert_eq!(input.la(1), 0xBB);
    //     assert_eq!(input.index(), 2);
    //     let mut input = InputStream::new("は".as_bytes());
    //     assert_eq!(input.la(1), 227);
    // }
}
