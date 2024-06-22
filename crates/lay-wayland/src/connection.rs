use log;
use rustix::fd::{AsRawFd, FromRawFd, RawFd};
use rustix::path::Arg;
use rustix::{
    fd::{IntoRawFd, OwnedFd},
    net::{AddressFamily, SocketType},
};
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::string::ParseError;
use std::{borrow::BorrowMut, ffi::OsStr};

use crate::Connection;
use libc::statfs as statfs_t;
use libc::statfs as statfs_f;
macro_rules! err {
    ($err:path, $($f:tt)*) => {
       match $($f)* {
           Ok(r) => Ok(r),
           Err(_) => Err($err),
       }
    };
}

#[derive(Debug)]
pub enum ConnError {
    InvalidEnv,
    NotFound,
    InvlidFD,
}

impl Connection {
    pub fn from_env() -> Result<Self, ConnError> {
        #[cfg(debug_assertions)]
        if let Some(socket) = env::var_os("WAYLAND_SOCKET") {
            log::warn!(target: "connection", "Attempting to connect with WAYLAND_SOCKET");

            let socket = err!(
                ConnError::InvalidEnv,
                socket
                    .as_str()
                    .map(|socket| err!(ConnError::InvalidEnv, socket.parse::<i32>()))
            )??;

            // SAFTEY: fcntl SHOULD be in most linux distrubution
            let flags: i32 = unsafe { libc::fcntl(socket, libc::F_GETFD) };
            if flags == -1 {
                log::error!(target: "connection", "Unable to access socket");
                return Err(ConnError::InvlidFD);
            };

            unsafe {
                if libc::fcntl(socket, libc::F_SETFD, flags | libc::FD_CLOEXEC) == -1 {
                    log::error!("Failed to set FD_CLOEXEC")
                };
            }

            return Ok(unsafe { Self::from_fd(FromRawFd::from_raw_fd(socket)) });
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
        Err(ConnError::NotFound)
    }

    pub fn from_file(path: impl AsRef<OsStr>) -> std::io::Result<Self> {
        let connection = UnixStream::connect(path.as_ref())?;
        connection.set_nonblocking(true)?;

        // SAFETY: The File is open and valid unix domain socket
        return unsafe { Ok(Self::from_fd(connection.into_raw_fd())) };
    }

    // SAFETY: The File MUST be open and a valid unix domain socket
    #[inline(always)]
    pub unsafe fn from_fd(fd: RawFd) -> Self {
        Self(OwnedFd::from_raw_fd(FromRawFd::from_raw_fd(fd)))
    }
}

#[test]
fn connection() {
    let conn = Connection::from_env().unwrap();
}
