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

#[repr(C, packed)]
pub struct Message {
    object_id: ObjectID,
    msg_len: u16,
    opcode: u16,
}

pub struct Connection(OwnedFd);

mod connection;
mod events;
mod request;
mod types;
