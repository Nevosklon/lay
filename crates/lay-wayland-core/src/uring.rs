use std::{
    env,
    ffi::OsStr,
    io::ErrorKind,
    os::fd::{FromRawFd, IntoRawFd, OwnedFd},
    path::Path,
};

use crate::{
    Driver, FormatRequest, MetaData, Request, RequestInfo, SendRequest, SingleRequest,
    SingleRuntime,
};
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

    fn request<'a, R>(&self, request: R::Wire) -> Self::RequestResult
    where
        R: Request<'a>,
        R::Wire: FormatRequest<SingleRequest, [u8]>,
    {
        match <R as Request>::MULTIPLE {
            crate::RequestType::Multiple(_) => unreachable!(),
            crate::RequestType::Single(sender) => {
                // let x: &&[u8] = &mut request.format(sender);
                let buf = request.format(sender);
                return Ok(rustix::io::write(self.connection.as_fd(), &buf)?);
            }
        };
    }
}

#[test]
fn connection() {
    let conn = SingleRuntime::from_env().unwrap();
}
struct DummyRequest([u8; 4]);
impl RequestInfo<'_> for DummyRequest {
    const METADATA: MetaData = MetaData {
        fixed_size: false,
        size_hint: size_of::<Self>(),
    };
}
unsafe impl FormatRequest<SingleRequest, [u8; 4]> for DummyRequest {
    fn format<'a>(&'a self, _is_multiple_request: SingleRequest) -> &'a [u8; 4] {
        &self.0
    }
}
unsafe impl FormatRequest<SingleRequest, [u8]> for DummyRequest {
    fn format<'a>(&'a self, _is_multiple_request: SingleRequest) -> &'a [u8] {
        &self.0[..]
    }
}
impl<'a> Request<'a> for DummyRequest {
    type Wire = DummyRequest;
    const MULTIPLE: crate::RequestType = crate::RequestType::Single(SingleRequest);
}
impl<'a> Request<'a> for &'a DummyRequest {
    type Wire = &'a DummyRequest;
    const MULTIPLE: crate::RequestType = crate::RequestType::Single(SingleRequest);
}

impl<'a> RequestInfo<'a> for &DummyRequest {
    const METADATA: MetaData = DummyRequest::METADATA;
}

impl<'a> SendRequest<'a> for DummyRequest {}

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
    // runtime.request(request).unwrap();
    request.request(&runtime).unwrap();
    let SingleRuntime { connection } = runtime;
    let fs: std::fs::File = connection.into();
    let mut buf = [0u8; size_of::<u32>()];
    let readed = fs.read_at(&mut buf, 0).unwrap();
    assert_eq!(readed, 4);
    assert_eq!(buf, [b'A'; 4])
}
