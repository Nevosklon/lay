use core::slice;
use std::{
    array,
    io::{Read, Write},
    os::unix::net::UnixStream,
};

use rustix::{
    fd::{AsFd, AsRawFd, FromRawFd},
    io::Result,
};

use crate::{Connection, Message, MsgLen, MsgOpcode, ObjectID};

pub struct WlDisplay;
impl WlDisplay {}

trait Request {
    type Name;
    type Args;
}

pub struct GetRegistry {
    header: Message,
    // rest of the arguments ....
    registry: ObjectID,
}
// object ID 1 is explictly wl_display
const WL_DISIPLAY_ID: ObjectID = 1;

impl GetRegistry {
    const OPCODE: MsgOpcode = 1;
    fn send(registry: ObjectID) -> Self {
        let header = Message::new::<Self>(WL_DISIPLAY_ID, Self::OPCODE);
        Self { header, registry }
    }
    pub const fn into_array(&self) -> [u32; std::mem::size_of::<Self>() / 4] {
        assert!(self.header.len == 12);
        assert!(self.header.opcode == 1);
        [
            (self.header.object_id),
            unsafe { std::mem::transmute([self.header.opcode, self.header.len]) },
            (self.registry),
        ]
    }
}

impl Message {
    pub const fn new<T>(object_id: ObjectID, opcode: MsgOpcode) -> Self {
        const { assert!(std::mem::size_of::<Self>() == 8) };
        let msg_len = std::mem::size_of::<T>() as MsgLen;
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
            len: msg_len,
            opcode,
        }
    }
    pub fn from_bytes(buffer: &[u8]) -> Option<Self> {
        if buffer.len() < (std::mem::size_of::<u64>() / 4) {
            return None;
        };
        let object_id = unsafe { u32::from_le_bytes(*(buffer.as_ptr().cast::<[u8; 4]>())) };
        let [opcode, len] = unsafe {
            std::mem::transmute(u32::from_le_bytes(
                *(buffer[4..8].as_ptr().cast::<[u8; 4]>()),
            ))
        };
        Some(Self {
            object_id,
            len,
            opcode,
        })
    }
}

#[test]
fn send_get_registery() {
    let msg = GetRegistry::send(2);
    let result = msg.into_array();
    assert_eq!(msg.into_array(), [0x0000001, 0x001000C, 0x00000002]);
}

#[test]
fn get_registry() {
    env_logger::init();
    let conn = Connection::from_env().unwrap();
    let mut conn = unsafe { UnixStream::from_raw_fd(conn.0.as_raw_fd()) };
    let msg = GetRegistry::send(2);
    let send: [u8; std::mem::size_of::<u32>() * 3] =
        unsafe { std::mem::transmute(msg.into_array()) };

    let bytes = unsafe { conn.write(&send).unwrap() };

    assert_eq!(bytes, std::mem::size_of::<u32>() * 3);
    let mut buffer = [0; u8::MAX as _];
    let bytes = conn.read(&mut buffer).unwrap();
    dbg!(bytes);
    // assert!((bytes % 4) == 0);
    buffer[0..bytes]
        .chunks(4)
        .filter_map(|i| i.try_into().ok())
        .map(u32::from_le_bytes)
        .for_each(|i| eprintln!("0x{:08X}", i));
    dbg!(Message::from_bytes(&buffer[..bytes]));
    eprintln!("{:?}", String::from_utf8_lossy(&buffer[8..bytes]));
}
