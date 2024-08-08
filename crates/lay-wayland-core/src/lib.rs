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
    type Error;
    type Buffer;
    type Handle<'a>: RuntimeHandle;

    const BLOCKING: bool = true;
    const ASYNC: bool = false;

    /// Blocking constructor of the reactor
    fn connect_env() -> Option<Self::Buffer>;

    fn read(buffer: &Self::Buffer) -> usize;

    /// async constructor of the reactor
    fn connecting() -> impl Future<Output = Option<Self::Buffer>> + Send;

    fn reading(buffer: &Self::Buffer) -> impl Future<Output = Option<Self::Error>> + Send;

    fn handle<'a>(&self) -> Self::Handle<'a>;
}

pub trait RuntimeHandle {
    type Runtime: Runtime;

    /// Blocking variant of channels sending
    fn request(&mut self, request: impl Request);

    /// Blocking variant of channels recv
    fn event(&mut self, event: impl Interface);

    /// async variant of channels sending
    fn requesting(&self, request: impl Request) -> impl Future<Output = ()>;

    /// async variant of channels recv
    fn eventing(&self, event: impl Interface) -> impl Future<Output = ()>;
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

pub trait Request: Sized {
    type Interface: Interface;

    const N: usize = size_of::<Header>() + size_of::<Self>();
    const FIXED: bool = const { Self::N > 1 };
    type Bytes: Bytes;

    fn as_bytes(&self) -> &Self::Bytes;
    fn into_bytes(&self) -> Self::Bytes;
    fn request(&self, runtime: &impl RuntimeHandle);
    fn requesting(&self, runtime: &impl RuntimeHandle) -> impl Future<Output = ()>;
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
