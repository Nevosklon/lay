//!
//! lays wayland core
//! libary that contains the
//!

use std::{
    collections::VecDeque,
    future::Future,
    io::{IoSlice, IoSliceMut},
    os::fd::OwnedFd,
};

use lay_wayland_wire::Header;

// A Blocking event loop using io uring runtime
pub struct DefaultRuntime {
    connection: OwnedFd,
}

// The wayland will use actor model
pub trait Runtime {
    type NotifyResult;
    type RequestResult;

    const BLOCKING: bool = true;
    const ASYNC: bool = false;

    // notify has occured event
    fn notifing(&self, event: &impl Interface) -> impl Future<Output = Self::NotifyResult>;
    fn requesting(&self, request: &impl Request) -> impl Future<Output = Self::RequestResult>;
    fn notify(&self, event: &impl Interface) -> Self::NotifyResult;
    fn request(&self, request: &impl Request) -> Self::RequestResult;
}

pub mod uring;

pub trait Interface {
    type Event;
    type Error;
}

pub trait Bytes {
    const FIXED: bool = false;
    const N: usize = 0;
}

impl<const N: usize> Bytes for [u8; N] {
    const FIXED: bool = true;
    const N: usize = N;
}

impl Bytes for Vec<u8> {}
impl Bytes for VecDeque<u8> {}
impl Bytes for [u8] {}
impl<'a> Bytes for IoSlice<'a> {}
impl<'a> Bytes for IoSliceMut<'a> {}

impl Bytes for &Vec<u8> {}
impl Bytes for &VecDeque<u8> {}
impl Bytes for &[u8] {}
impl<'a> Bytes for &'a IoSlice<'a> {}
impl<'a> Bytes for &'a IoSliceMut<'a> {}

pub trait Request: Sized {
    type Interface: Interface;

    const N: usize = size_of::<Header>() + size_of::<Self>();
    const FIXED: bool = const { Self::N > 1 };
    type Bytes: Bytes;

    fn as_bytes(&self) -> &Self::Bytes;
    fn into_bytes(&self) -> Self::Bytes;
}

#[macro_export]
macro_rules! err {
    ($err:path, $($f:tt)*) => {
       match $($f)* {
           Ok(r) => Ok(r),
           Err(_) => Err($err),
       }
    };
}
