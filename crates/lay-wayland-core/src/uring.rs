use std::{
    borrow::BorrowMut,
    env,
    ffi::OsStr,
    io::{BufReader, ErrorKind, Write},
    os::{
        fd::{FromRawFd, IntoRawFd, OwnedFd},
        unix::net::UnixStream,
    },
    path::Path,
};

use crate::{Bytes, Driver, SingleRuntime};
use rustix::{
    self,
    fd::{AsFd, AsRawFd, RawFd},
    io::{Errno, FdFlags},
    net::{AddressFamily, SocketAddrUnix, SocketFlags, SocketType},
};

const WAYLAND_SOCKET: &'static str = "WAYLAND_SOCKET";

#[repr(u16)]
pub enum ConnError {
    InvalidFD,
    PermissionDenied,
    NotFound,
    IoError { errno: Errno },
}

impl SingleRuntime {
    pub fn from_env() -> Result<Self, ErrorKind> {
        #[cfg(debug_assertions)]
        if let Some(socket_fd) = env::var_os(WAYLAND_SOCKET) {
            log::warn!(target: "connection", "Attempting to connect with WAYLAND_SOCKET");

            let socket_fd: OwnedFd = match socket_fd.to_str() {
                Some(socket) => unsafe {
                    OwnedFd::from_raw_fd(socket.parse().map_err(|_| ErrorKind::Other)?)
                },
                None => Err(ErrorKind::Other)?,
            };

            let flags = rustix::io::fcntl_getfd(socket_fd.as_fd()).map_err(|err| err.kind())?;

            rustix::io::fcntl_setfd(socket_fd.as_fd(), flags | FdFlags::CLOEXEC)
                .map_err(|err| err.kind())?;

            return Ok(Self {
                connection: socket_fd,
            });
        };

        log::info!(target: "connection", "Attempt to connecting to wayland socket");

        if let Some(xdg_path) = env::var_os("XDG_RUNTIME_DIR") {
            //
            // XDG_RUNTIME_DIR + WAYLAND_DISPLAY
            if let Some(display) = env::var_os("WAYLAND_DISPLAY") {
                let path = Path::new(&xdg_path).join(display);
                log::info!(target:
                    "connection",
                    "Attempting to connect with XDG_RUNTIME + WAYLAND_DISPLAY: {path:?}",
                );

                if let Ok(connection) = Self::from_file(path) {
                    return Ok(connection);
                }
            }

            let path = Path::new(&xdg_path).join("wayland-0");
            log::info!(target: "connection", "Attempting to connect with XDG_RUNTIME_DIR + wayland-0: {path:?}");
            //
            // Assume that wayland socket is called wayland-0
            // XDG_RUNTIME_DIR + wayland-0
            if let Ok(connection) = Self::from_file(Path::new(&xdg_path)) {
                return Ok(connection);
            }
        };

        log::error!(target: "connection", "Unable to connect to wayland socket");
        Err(ErrorKind::NotFound)
    }

    pub fn from_file(path: impl AsRef<OsStr>) -> std::io::Result<Self> {
        let sockfd = rustix::net::socket_with(
            AddressFamily::UNIX,
            SocketType::STREAM,
            SocketFlags::NONBLOCK | SocketFlags::CLOEXEC,
            None,
        )?;

        rustix::net::connect_unix(sockfd.as_fd(), &(SocketAddrUnix::new(path.as_ref())?))?;

        return Ok(Self { connection: sockfd });
    }

    // SAFETY: The File MUST be open and a valid unix domain socket
    #[inline(always)]
    pub unsafe fn from_fd(fd: impl IntoRawFd) -> Self {
        Self {
            connection: OwnedFd::from_raw_fd(fd.into_raw_fd()),
        }
    }
}
struct Dummy;

impl Driver for SingleRuntime {
    type NotifyResult = Dummy;
    type RequestResult = rustix::io::Result<usize>;

    fn notify(&self, event: &impl crate::Interface) -> Self::NotifyResult {
        unimplemented!();
    }

    fn request<'a>(
        &self,
        request: &'a impl crate::Request<'a, Bytes = &'a [u8]>,
    ) -> Self::RequestResult {
        rustix::io::write(&self.connection, request.as_bytes())
    }

    fn notifing(&self, event: &impl crate::Interface) -> impl std::future::Future<Output = Dummy> {
        unimplemented!("Runtime doesn't support async");
    }

    fn requesting<'a>(
        &self,
        request: &impl crate::Request<'a>,
    ) -> impl std::future::Future<Output = Dummy> {
        unimplemented!("Runtime doesn't support async");
    }
}

#[test]
fn connection() {
    let conn = SingleRuntime::from_env().unwrap();
}
