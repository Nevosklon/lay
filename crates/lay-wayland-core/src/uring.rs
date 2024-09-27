use std::{
    env,
    ffi::OsStr,
    io::ErrorKind,
    os::fd::{FromRawFd, IntoRawFd, OwnedFd},
    path::Path,
};

use crate::{Driver, Request, SingleRuntime};
// use lay_wayland_wire::Runtime;
#[allow(unused_imports)]
use rustix::{
    self,
    fd::AsFd,
    fs::FileExt,
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

impl Driver for SingleRuntime {
    type NotifyResult = ();
    type RequestResult = rustix::io::Result<usize>;

    fn notify(&self, _event: &impl crate::Interface) -> Self::NotifyResult {
        unimplemented!();
    }

    fn request<'a, R>(&self, request: R) -> Self::RequestResult
    where
        R: Request<'a>,
    {
        let method = request.wire();
        rustix::io::write(self.connection.as_fd(), &method.as_ref()[..])
    }
}

#[test]
fn connection() {
    let conn = SingleRuntime::from_env().unwrap();
}
struct DummyRequest([u8; 4]);

impl<'a> Request<'a> for DummyRequest {
    const SIZEDHINT: usize = size_of::<Self>();
    type Wire = [u8; 4];

    fn wire(self) -> Self::Wire {
        self.0
    }
}
impl<'a> Request<'a> for &'a DummyRequest {
    const SIZEDHINT: usize = size_of::<Self>();
    type Wire = &'a [u8; 4];

    fn wire(self) -> Self::Wire {
        &self.0
    }
}
#[test]
fn write_request() {
    let request = DummyRequest(*b"AAAA");
    let fs = std::fs::File::options()
        .create(true)
        .append(true)
        .read(true)
        .open("DummyRequest.txt")
        .unwrap();

    let runtime = SingleRuntime {
        connection: fs.into(),
    };
    runtime.request(&request).unwrap();
    let SingleRuntime { connection } = runtime;
    let fs: std::fs::File = connection.into();
    let mut buf = [0u8; size_of::<u32>()];
    let readed = fs.read_at(&mut buf, 0).unwrap();
    assert_eq!(readed, 4);
    assert_eq!(buf, [b'A'; 4])
}
