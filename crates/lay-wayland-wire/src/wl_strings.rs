use std::{
    borrow::{Borrow, Cow},
    io::{Read, Write},
};

use crate::WlType;
use crate::{word, WlString, Word};
use core::cmp::max;

const NULL_TERMINATOR: u32 = 1;
enum WlStringError {
    MalformedString = 0,
}

impl<'a> WlString<'a> {
    const PADDING: u32 = 4;
    const LEN: usize = 4;

    const fn padding(len: u32) -> usize {
        4 - (len % Self::PADDING) as usize
    }

    // the string length PADDED to 32 bits and INCLUDING the null terminator
    const fn str_capacity(len: u32) -> usize {
        (Self::padding(len + NULL_TERMINATOR) + len as usize + NULL_TERMINATOR as usize) as usize
    }

    // the string length EXCLUDING the null terminator and the PADDING
    pub fn str_len(&self) -> usize {
        self.len as usize
    }

    pub fn from_buf(buf: &'a [u8]) -> Option<Self> {
        // TODO: Do Not assume that all string format are utf8

        let len = max(u32::wl_type(&buf[0..4])?, 1) - NULL_TERMINATOR;

        // skips the string length and RETRIEVE the string content INCLUDE null terminator + padding
        let content = Cow::Borrowed(&buf[Word::SIZE..Self::str_capacity(len)]);

        // NOTE: the string length EXCLUDING the null terminator
        Some(Self { len, content })
    }

    pub fn new(s: &str) -> Self {
        let mut content = s.as_bytes().to_vec();
        content.resize(Self::str_capacity(s.len() as u32), b'\0');

        return Self {
            len: s.len() as _,
            content: Cow::Owned(content),
        };
    }

    pub fn buffer_hint(&self) -> usize {
        self.content.len() + Self::LEN
    }

    pub fn quene(&self, buf: &mut [u8]) -> Option<()> {
        // TODO: change ? option type to result
        if self.buffer_hint() > buf.len() {
            return None;
        }
        u32::write(self.len, &mut buf[0..4]);
        buf.copy_from_slice(&self.content);

        return Some(());
    }
}

impl core::cmp::PartialEq<str> for WlString<'_> {
    fn eq(&self, other: &str) -> bool {
        if other.len() as u32 != self.len {
            return false;
        };
        other.as_bytes().eq(&self.content[..(self.len as usize)])
    }
}
impl core::cmp::PartialEq<String> for WlString<'_> {
    fn eq(&self, other: &String) -> bool {
        if other.len() as u32 != self.len {
            return false;
        };
        other.as_bytes().eq(&self.content[0..(self.len as usize)])
    }
}
impl core::cmp::PartialEq<WlString<'_>> for str {
    fn eq(&self, wl_str: &WlString) -> bool {
        if self.len() as u32 != wl_str.len {
            return false;
        };
        self.as_bytes()
            .eq(&wl_str.content[0..(wl_str.len as usize)])
    }
}
impl core::cmp::PartialEq<WlString<'_>> for String {
    fn eq(&self, wl_str: &WlString) -> bool {
        if self.len() as u32 != wl_str.len {
            return false;
        };
        self.as_bytes()
            .eq(&wl_str.content[0..(wl_str.len as usize)])
    }
}
impl core::fmt::Debug for WlString<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(self.content.as_ref()))
    }
}
