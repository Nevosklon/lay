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
    fn request<R>(&self, request: R) -> Self::RequestResult
    where
        R: Request;
}

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

pub trait Request {
    const SIZEDHINT: usize = 0;
    type Wire<'a>: AsRef<[u8]>;
    fn wire<'a>(self) -> Self::Wire<'a>;
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
