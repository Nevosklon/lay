//!
//! lays wayland core
//! libary that contains the
//!

use std::{future::Future, os::fd::OwnedFd};

// A Blocking event loop using io uring runtime
pub struct SingleRuntime {
    connection: OwnedFd,
}

// The wayland will use actor model
pub trait Driver {
    type NotifyResult;
    type RequestResult;

    const BLOCKING: bool = true;
    const ASYNC: bool = false;

    // notify has occured event
    // fn notifing(&self, event: &impl Interface) -> impl Future<Output = Self::NotifyResult>;
    // fn requesting<'a>(
    //     &self,
    //     request: &'a impl Request<'a>,
    // ) -> impl Future<Output = Self::RequestResult>;
    fn notify(&self, event: &impl Interface) -> Self::NotifyResult;
    fn request<'a, R>(&self, request: R) -> Self::RequestResult
    where
        R: Request<'a>;
}

pub trait AsyncDriver: Driver {}

pub trait Runtime {
    type Drivers: Driver;
    fn connect() -> Self::Drivers;
    fn connecting() -> impl Future<Output = Self::Drivers>;
}

pub mod uring;

pub trait Interface {
    type Event;
    type Error;
}

pub struct MetaData {
    pub is_fixed_size: bool,
    pub multiple_request: bool,
    pub size_hint: usize,
}
pub trait RequestMetaData {
    const METADATA: MetaData;
}
trait WireType {}
impl<const N: usize> WireType for [u8; N] {}
impl<const N: usize> WireType for &[u8; N] {}
impl WireType for &[u8] {}
pub trait Request<'a, T>: RequestMetaData
where
    T: 'a,
{
    const SIZEDHINT: usize = 0;
    const ISSIZED: bool = true;
    type Wire: AsRef<T> + 'a;
    fn wire(self) -> Self::Wire;
}
pub unsafe trait RequestTransmute<'a, Request, Output>
where
    Request: RequestMetaData,
    Output: 'a,
    Request: 'a,
{
    fn transmute(request: Request) -> Output;
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
