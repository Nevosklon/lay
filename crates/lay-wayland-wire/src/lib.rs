//! The wire type and implementation of [wayland](https://wayland.freedesktop.org/docs/html/ch04.html)
//!

use std::borrow::Cow;

#[repr(transparent)]
#[derive(Clone, Copy)]
/// wl_fixed type from wayland
pub struct WlFixed(i32);

pub trait WlType {
    type WlType<'a>;
    fn wl_type<'a>(buffer: &'a [u8]) -> Option<Self::WlType<'a>>;
    fn write<'a>(v: Self::WlType<'a>, buffer: &mut [u8]) -> Option<()>;
}

pub struct WlStr<'a> {
    /// the str length excluding the null terminator
    len: u32,
    content: Cow<'a, [u8]>,
}

#[repr(transparent)]
pub struct WlArray<T> {
    value: Vec<T>,
}

#[allow(non_camel_case_types)]
pub type Wl_I32 = i32;
#[allow(non_camel_case_types)]
pub type Wl_U32 = u32;
pub type ObjectID = u32;
pub type NewId = u32;
pub type HeaderLen = u16;
pub type Opcode = u16;
pub type RawWord = u32;

#[derive(Debug)]
#[repr(transparent)]
pub(crate) struct Payload<'a>(&'a [u8]);

#[derive(Debug)]
pub struct Header {
    object_id: ObjectID,
    opcode: Opcode,
    len: HeaderLen,
}

pub struct Word;

mod word;

mod header;
mod wl_fixed;
mod wl_strings;

use std::future::Future;
use std::process::Output;

// The wayland will use actor model
pub trait Runtime {
    type Error;
    type Buffer;

    /// Blocking constructor of the reactor
    fn connect() -> Option<Self::Buffer>;

    /// Blocking variant of channels sending
    fn send() -> Option<Self::Error>;
    /// Blocking variant of channels recv
    fn recv() -> Option<Self::Error>;

    fn read(buffer: &Self::Buffer) -> usize;

    /// async constructor of the reactor
    fn connecting() -> impl Future<Output = Option<Self::Buffer>> + Send;

    /// async variant of channels sending
    fn sending() -> impl Future<Output = Option<Self::Error>> + Send;
    /// async variant of channels recv
    fn recving() -> impl Future<Output = Option<Self::Error>> + Send;

    fn reading(buffer: &Self::Buffer) -> impl Future<Output = Option<Self::Error>> + Send;
}
