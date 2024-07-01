use std::{ffi::CString, ptr::NonNull};

use rustix::fd::OwnedFd;

#[repr(transparent)]
#[derive(Clone, Copy)]
/// wl_fixed type from wayland
pub struct WlFixed(i32);

#[repr(C)]
pub struct WlString {
    len: u32,
    text: CString,
    _padding: u32,
}
#[repr(C)]
pub struct WlArray {
    len: u32,
    value: Vec<u8>,
    _padding: u32,
}
#[derive(Debug, Clone, Copy)]
pub struct OOB(usize);

pub type Wli32 = i32;
pub type Wlu32 = u32;
pub type ObjectID = u32;
pub type NewId = u32;
pub type MsgLen = u16;
pub type MsgOpcode = u16;
pub type RawWord = u32;

#[derive(Debug)]
#[repr(transparent)]
pub(crate) struct Word(RawWord);

#[derive(Debug)]
pub struct Message {
    object_id: ObjectID,
    len: MsgLen,
    opcode: MsgOpcode,
}

pub struct Connection(OwnedFd);

mod connection;
mod events;
mod request;
mod utils;

pub use crate::utils::FromWords;
