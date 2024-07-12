use core::{fmt, slice};
use std::{
    array,
    fmt::{LowerHex, UpperHex},
    io::{Read, Write},
    os::unix::net::UnixStream,
};

use rustix::{
    fd::{AsFd, AsRawFd, FromRawFd},
    io::Result,
};

use crate::{Connection, FromWords, Message, MsgLen, MsgOpcode, ObjectID, RawWord, WlString, Word};

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
        [
            (self.header.object_id),
            Word::from_u16(self.header.len, self.header.opcode),
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
        if buffer.len() < std::mem::size_of::<[u32; 2]>() {
            return None;
        };

        let object_id = ObjectID::from_word(&buffer[0..4]);
        let (len, opcode) = <(MsgLen, MsgOpcode)>::from_word(&buffer[4..8]);

        Some(Self {
            object_id,
            len,
            opcode,
        })
    }
}

#[test]
fn word_get_registry() {
    let msg = GetRegistry::send(2);
    assert_eq!(msg.into_array(), [0x1, 0xC0001, 0x2]);
}

#[test]
fn get_registry() {
    let conn = Connection::from_env().unwrap();
    let mut conn = unsafe { UnixStream::from_raw_fd(conn.0.as_raw_fd()) };
    unsafe {
        let msg: [u8; std::mem::size_of::<GetRegistry>()] =
            std::mem::transmute(GetRegistry::send(2).into_array());
        conn.write(&msg).expect("Failed to write")
    };
    let mut buffer = [0; 128 * 2];
    let bytes = conn.read(&mut buffer).unwrap();
    assert_eq!(buffer.len(), bytes);
    let msg = dbg!(Message::from_bytes(&buffer[..]).unwrap());

    dbg!(RawWord::from_word(
        &buffer[Message::PAYLOAD_START..Message::PAYLOAD_START + Word::SIZE]
    ));
    dbg!(WlString::from_buf(
        &buffer[Message::PAYLOAD_START + Word::SIZE..]
    ));

    assert_eq!(msg.len, 28);
    assert_eq!(msg.opcode, 0);
    assert_eq!(msg.object_id, 2);
}
