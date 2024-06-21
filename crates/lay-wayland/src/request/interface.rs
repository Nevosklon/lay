use std::{
    array,
    io::{Read, Write},
    os::unix::net::UnixStream,
};

use rustix::{
    fd::{AsFd, AsRawFd, FromRawFd},
    io::Result,
};

use crate::{Connection, Message, ObjectID};

pub struct WlDisplay;
impl WlDisplay {}

trait Request {
    type Name;
    type Args;
}

#[repr(C, packed)]
pub struct GetRegistry {
    header: Message,
    // rest of the arguments ....
    registry: ObjectID,
}
// object ID 1 is explictly wl_display
const WL_DISIPLAY_ID: ObjectID = 1;

impl GetRegistry {
    const OPCODE: u16 = 2;
    fn send(registry: ObjectID) -> Self {
        let header = Message::new::<Self>(WL_DISIPLAY_ID, Self::OPCODE);
        Self { header, registry }
    }
}

impl Message {
    const fn new<T>(object_id: u32, opcode: u16) -> Self {
        let msg_len = std::mem::size_of::<T>() as u16;
        const {
            let size: usize = std::mem::size_of::<T>() - std::mem::size_of::<Message>();
            assert!(size > 0, "Message length must be larger than 0");
            assert!(
                size < (u16::MAX as usize),
                "Message length must be less than 0xFFFF"
            );
        };
        Self {
            object_id,
            msg_len,
            opcode,
        }
    }
}

#[test]
fn get_registry() {
    env_logger::init();
    let conn = Connection::from_env().unwrap();
    let mut conn = unsafe { UnixStream::from_raw_fd(conn.0.as_raw_fd()) };
    let msg = GetRegistry::send(2);
    unsafe {
        conn.write(&std::mem::transmute::<
            GetRegistry,
            [u8; std::mem::size_of::<GetRegistry>()],
        >(msg))
            .expect("Failed to write")
    };
    let mut buffer = [0; u8::MAX as _];
    let bytes = conn.read(&mut buffer).unwrap();
    assert!((bytes % 4) == 0);
    buffer[0..bytes]
        .chunks(4)
        .filter_map(|i| i.try_into().ok())
        .map(u32::from_le_bytes)
        .for_each(|i| eprintln!("{:010x}", i));
}
