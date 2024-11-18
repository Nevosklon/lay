//!
//! lays wayland core
//! libary that contains the
//!

use std::{borrow::Cow, future::Future, os::fd::OwnedFd};

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
    fn request<R>(&self, request: R::Wire) -> Self::RequestResult
    where
        R: Request,
        R::Wire: IntoWire;
}

trait SendRequest
where
    Self: Sized,
    Self: Request<Wire = Self>,
    Self::Wire: IntoWire,
{
    fn request<R>(self, runtime: &R) -> R::RequestResult
    where
        R: Driver,
    {
        runtime.request::<Self>(self)
    }
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
/// If it is safe to transmute
pub struct MetaData {
    pub fixed_size: bool,
    pub size_hint: usize,
}
pub trait RequestInfo {
    const METADATA: MetaData;
}
pub enum RequestType {
    Single(SingleRequest),
    Multiple(MultipleRequest),
}
#[derive(Clone, Copy)]
pub struct SingleRequest;
#[derive(Clone, Copy)]
pub struct MultipleRequest;
pub trait Request {
    const MULTIPLE: RequestType;
    type Wire: RequestInfo;
}
#[repr(C, packed)]
struct Request1<T, U>(T, U);
impl<T: RequestInfo, U: RequestInfo> Request1<T, U>
where
    T: RequestInfo,
    U: RequestInfo,
{
    const METADATA: MetaData = MetaData {
        fixed_size: true,
        size_hint: T::METADATA.size_hint + U::METADATA.size_hint,
    };
}

impl<T, U> RequestInfo for (T, U)
where
    T: RequestInfo,
    U: RequestInfo,
{
    const METADATA: MetaData = MetaData {
        fixed_size: true,
        size_hint: T::METADATA.size_hint + U::METADATA.size_hint,
    };
}
pub trait IntoWire {
    fn as_bytes<'a>(&'a self) -> Cow<'a, [u8]>;
    fn into_vec(&self) -> Vec<u8>;
}
impl<T: RequestInfo + IntoWire, U: RequestInfo + IntoWire> IntoWire for Request1<T, U> {
    fn as_bytes<'a>(&'a self) -> Cow<'a, [u8]> {
        match Self::METADATA {
            MetaData {
                fixed_size: true,
                size_hint,
            } => Cow::Borrowed(unsafe {
                std::slice::from_raw_parts(self as *const Self as *const u8, size_hint)
            }),
            _ => {
                let b1 = self.0.as_bytes();
                Cow::Owned(vec![0u8, 0u8, 0u8])
            }
        }
    }
    fn into_vec(&self) -> Vec<u8> {
        let mut ret = (&raw self.0).into_vec();
        ret.extend(self.1.into_vec().iter());
        ret
    }
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

// struct Request1<T, U>(T, U);
