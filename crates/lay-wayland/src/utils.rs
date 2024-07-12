use std::{
    ffi::CString,
    fmt::{Debug, UpperHex},
    ops::Deref,
};

use crate::{Message, Payload, RawWord, WlFixed, WlString, Word};

impl WlFixed {
    pub fn from_f32(f: f32) -> Self {
        Self((f * 256.0) as i32)
    }
    pub const fn from_i32(i: i32) -> Self {
        Self(i * 256)
    }
    pub const fn to_i32(s: Self) -> i32 {
        s.0 / 256
    }
    pub fn to_f32(s: Self) -> f32 {
        (s.0 as f32) / 256.0
    }
}
impl From<f32> for WlFixed {
    fn from(f: f32) -> Self {
        WlFixed::from_f32(f)
    }
}
impl From<i32> for WlFixed {
    fn from(value: i32) -> Self {
        WlFixed::from_i32(value)
    }
}

impl From<WlFixed> for i32 {
    fn from(value: WlFixed) -> Self {
        WlFixed::to_i32(value)
    }
}
impl From<WlFixed> for f32 {
    fn from(value: WlFixed) -> Self {
        WlFixed::to_f32(value)
    }
}

impl Debug for WlFixed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("WlFixed")
            .field(&Self::to_f32(*self))
            .finish()
    }
}

pub trait FromWords<T> {
    #[allow(private_bounds)]
    type From<'a>;
    fn from_word<'a>(from: Self::From<'a>) -> Self;
}

impl FromWords<&[u8]> for Option<[u16; 2]> {
    type From<'a> = &'a [u8];

    fn from_word<'a>(from: Self::From<'a>) -> Self {
        if from.len() >= std::mem::size_of::<Self>() {
            return None;
        };

        #[cfg(target_endian = "little")]
        {
            return Some([
                u16::from_le_bytes(<[u8; 2]>::try_from(&from[2..4]).ok()?),
                u16::from_le_bytes(<[u8; 2]>::try_from(&from[0..2]).ok()?),
            ]);
        }

        // TODO: Test this on endian machine
        #[cfg(target_endian = "big")]
        {
            return Some([
                u16::from_le_bytes(<[u8; 2]>::try_from(&from[0..2]).ok()?),
                u16::from_le_bytes(<[u8; 2]>::try_from(&from[2..4]).ok()?),
            ]);
        }
    }
}
impl Word {
    pub const fn from_u16(upper: u16, lower: u16) -> RawWord {
        (upper as u32) << 16 | (lower as u32)
    }
    pub const SIZE: usize = std::mem::size_of::<RawWord>();
}

impl FromWords<&[u8]> for [u16; 2] {
    type From<'a> = &'a [u8];

    fn from_word<'a>(from: Self::From<'a>) -> Self {
        <Option<[u16; 2]> as FromWords<Self::From<'a>>>::from_word(&from).unwrap()
    }
}
impl FromWords<&[u8]> for Option<(u16, u16)> {
    type From<'a> = &'a [u8];

    fn from_word<'a>(from: Self::From<'a>) -> Self {
        <Option<[u16; 2]> as FromWords<Self::From<'a>>>::from_word(&from).map(|w| (w[0], w[1]))
    }
}
impl FromWords<&[u8]> for (u16, u16) {
    type From<'a> = &'a [u8];

    fn from_word<'a>(from: Self::From<'a>) -> Self {
        <Option<(u16, u16)> as FromWords<Self::From<'a>>>::from_word(&from).unwrap()
    }
}
impl FromWords<&[u8]> for Option<u32> {
    type From<'a> = &'a [u8];
    fn from_word<'a>(from: Self::From<'a>) -> Self {
        if from.len() >= std::mem::size_of::<Self>() {
            return None;
        }
        Some(u32::from_le_bytes(
            <[u8; std::mem::size_of::<Self>() / 2]>::try_from(from).ok()?,
        ))
    }
}
impl FromWords<&u8> for u32 {
    type From<'a> = &'a [u8];
    fn from_word<'a>(from: Self::From<'a>) -> Self {
        <Option<u32> as FromWords<Self::From<'a>>>::from_word(from).unwrap()
    }
}

impl WlString {
    pub fn from_buf(buf: &[u8]) -> Option<Self> {
        // TODO: Do Not assume that all string format are utf8
        // wl_string has a word size

        // wl_String + null terminator
        let len = RawWord::from_word(&buf[..Word::SIZE]) - 1;
        let text = String::from(String::from_utf8_lossy(
            &buf[Word::SIZE..Word::SIZE + len as usize],
        ));
        Some(Self { text })
    }

    pub fn new(s: &str) -> Self {
        let mut s = Self {
            text: s.to_string(),
        };
        s.text.push_str("\0");
        return s;
    }
}

#[allow(private_bounds)]
trait WordType {}

impl WordType for [u16; 2] {}
impl WordType for (u16, u16) {}
impl WordType for u32 {}
impl<T> WordType for Option<T> where T: WordType {}

impl Message {
    pub const PAYLOAD_START: usize = std::mem::size_of::<Self>();
    pub const fn payload_len(&self) -> usize {
        self.len as usize - Self::PAYLOAD_START
    }
}

impl<'a> Payload<'a> {
    pub fn from_buf(header: &Message, buf: &'a [u8]) -> Option<Self> {
        match header.payload_len() as usize > buf.len() {
            true => None,
            false => Some(Self(&buf[Message::PAYLOAD_START..header.len as usize])),
        }
    }
}

impl<'a> Deref for Payload<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl UpperHex for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "0x{:X} 0x{:X}",
            self.object_id,
            Word::from_u16(self.len, self.opcode)
        )
    }
}
