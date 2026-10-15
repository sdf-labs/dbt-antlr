//! `IntStream` extension for Lexer that allows subslicing of underlying data
use std::borrow::Cow;
use std::char::REPLACEMENT_CHARACTER;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::ops::{Index, Range, RangeFrom};

use crate::int_stream::IntStream;

/// Provides underlying data for Tokens.
pub trait CharStream<'input>: IntStream {
    /// Returns underlying data piece, either slice or owned copy.
    /// Panics if provided indexes are invalid
    /// Called by parser only on token intervals.
    /// This fact can be used by custom implementations  
    fn get_text(&self, a: isize, b: isize) -> Cow<'input, str>;

    /// Whether [`CharStream::item_at`] is usable on this stream. `false`
    /// (the default) means the stream only supports cursor-based
    /// `la`/`consume` access; the table-driven lexer walker then uses the
    /// (slower) cursor path. A method (not an associated const) to keep
    /// `CharStream` dyn-compatible; inlines to a constant in the
    /// monomorphized walker.
    #[doc(hidden)]
    #[inline]
    fn has_item_at(&self) -> bool {
        false
    }

    /// Random access to the underlying data: the code point starting at
    /// index `index`, or `None` at/past end of input. Only usable when
    /// [`CharStream::has_item_at`] is true; the default (never called then)
    /// returns `None`. Lets the table-driven lexer walker hold a local
    /// cursor and peek the next input item directly, instead of a `la(1)`
    /// decode plus a `consume()` walk per char.
    #[doc(hidden)]
    #[inline]
    fn item_at(&self, index: isize) -> Option<i32> {
        let _ = index;
        None
    }

    /// Random access to the raw input bytes from byte index `index` to end
    /// of input; `None` when the stream has no byte view (the default) or
    /// `index` is out of bounds. Only usable when [`CharStream::has_item_at`]
    /// is true. Lets the table-driven lexer walker run vectorized scans
    /// over the self-loop runs marked in the static lexer tables.
    #[doc(hidden)]
    #[inline]
    fn bytes_at(&self, index: isize) -> Option<&'input [u8]> {
        let _ = index;
        None
    }

    #[doc(hidden)]
    #[inline]
    fn get_data_str(&self) -> Option<&'input str> {
        None
    }

    #[doc(hidden)]
    #[inline]
    fn get_line(&self, byte_offset: isize) -> u32 {
        let end = byte_offset.max(0).min(self.size());
        let prefix = self.get_text(0, end);
        let bytes = prefix.as_bytes();
        let len = (byte_offset.max(0) as usize).min(bytes.len());
        1 + memchr::memchr_iter(b'\n', &bytes[..len]).count() as u32
    }

    #[doc(hidden)]
    #[inline]
    fn get_char_position_in_line(&self, byte_offset: isize) -> i32 {
        let end = byte_offset.max(0).min(self.size());
        let prefix = self.get_text(0, end);
        let bytes = prefix.as_bytes();
        let len = (byte_offset.max(0) as usize).min(bytes.len());
        let line_start = memchr::memrchr(b'\n', &bytes[..len]).map_or(0, |i| i + 1);
        prefix[line_start..len].chars().count() as i32
    }
}

#[allow(clippy::len_without_is_empty)]
/// Trait for input that can be accepted by `InputStream` to be able to provide lexer with data.
/// Public for implementation reasons.
pub trait InputData:
    Index<Range<usize>, Output = Self>
    + Index<RangeFrom<usize>, Output = Self>
    + ToOwned
    + Debug
    + 'static
{
    // fn to_indexed_vec(&self) -> Vec<(u32, u32)>;

    #[doc(hidden)]
    fn offset(&self, index: isize, item_offset: isize) -> Option<isize>;

    #[doc(hidden)]
    fn item(&self, index: isize) -> Option<i32>;

    #[doc(hidden)]
    fn len(&self) -> usize;

    #[doc(hidden)]
    fn from_text(text: &str) -> Self::Owned;

    #[doc(hidden)]
    fn to_display(&self) -> String;
}

impl<T: Into<u32> + From<u8> + TryFrom<u32> + Copy + Debug + 'static> InputData for [T]
where
    <T as TryFrom<u32>>::Error: Debug,
{
    // fn to_indexed_vec(&self) -> Vec<(u32, u32)> {
    //     self.into_iter()
    //         .enumerate()
    //         .map(|(x, &y)| (x as u32, y.into()))
    //         .collect()
    // }

    #[inline]
    fn offset(&self, index: isize, item_offset: isize) -> Option<isize> {
        let new_index = index + item_offset;
        if new_index < 0 {
            return None; // invalid; no char before first char
        }
        if new_index > self.len() as isize {
            return None;
        }

        Some(new_index)
    }

    #[inline]
    fn item(&self, index: isize) -> Option<i32> {
        self.get(index as usize).map(|&it| it.into() as i32)
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn from_text(text: &str) -> Self::Owned {
        text.chars()
            .map(|it| T::try_from(it as u32).unwrap())
            .collect()
    }

    #[inline]
    // default
    fn to_display(&self) -> String {
        self.iter()
            .map(|x| char::try_from((*x).into()).unwrap_or(REPLACEMENT_CHARACTER))
            .collect()
    }
}
//
// impl InputData for [u8] {
//     #[inline]
//     fn to_display(&self) -> String { String::from_utf8_lossy(self).into_owned() }
// }

// impl InputData for [u16] {
// }
//
// impl InputData for [u32] {
//     #[inline]
//     fn to_display(&self) -> String {
//         self.iter()
//             .map(|x| char::try_from(*x).unwrap_or(REPLACEMENT_CHARACTER))
//             .collect()
//     }
// }

impl InputData for str {
    // fn to_indexed_vec(&self) -> Vec<(u32, u32)> {
    //     self.char_indices()
    //         .map(|(i, ch)| (i as u32, ch as u32))
    //         .collect()
    // }

    #[inline]
    fn offset(&self, mut index: isize, mut item_offset: isize) -> Option<isize> {
        if item_offset == 0 {
            return Some(index);
        }
        let direction = item_offset.signum();

        while {
            index += direction;
            if index < 0 || index > self.len() as isize {
                return None;
            }
            if self.is_char_boundary(index as usize) {
                item_offset -= direction;
            }
            item_offset != 0
        } {}

        Some(index)
    }

    #[inline]
    fn item(&self, index: isize) -> Option<i32> {
        self.get(index as usize..)
            .and_then(|it| it.chars().next())
            .map(|it| it as i32)
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    fn from_text(text: &str) -> Self::Owned {
        text.to_owned()
    }

    // #[inline]
    // fn from_text(text: &str) -> Self::Owned { text.to_owned() }

    #[inline]
    fn to_display(&self) -> String {
        self.to_owned()
    }
}
