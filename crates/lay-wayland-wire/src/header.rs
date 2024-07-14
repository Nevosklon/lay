use crate::WlType;
use crate::{Header, HeaderLen, ObjectID, Opcode, Word};

impl Header {
    pub const fn new<T>(object_id: ObjectID, opcode: Opcode) -> Self {
        const { assert!(size_of::<Self>() == 8) };

        const {
            let size: isize = size_of::<T>() as isize - size_of::<Header>() as isize;

            assert!(size > 0, "Message length must be larger than 0");
            assert!(
                size < (u16::MAX as isize),
                "Message length must be less than 0xFFFF"
            );
        };
        let len: u16 = const { (size_of::<Self>() + size_of::<T>()) as HeaderLen };
        Self {
            object_id,
            opcode,
            len,
        }
    }
    pub fn from_bytes(buffer: &[u8]) -> Option<Self> {
        if buffer.len() < size_of::<Self>() {
            return None;
        };
        assert!(buffer.len() >= size_of::<Self>());

        Some(Self {
            object_id: u32::wl_type(&buffer[0..4]).unwrap(),
            opcode: u16::wl_type(&buffer[4..6]).unwrap(),
            len: u16::wl_type(&buffer[6..8]).unwrap(),
        })
    }

    pub const PAYLOAD_START: usize = std::mem::size_of::<Self>();
    pub const fn payload_len(&self) -> usize {
        self.len as usize - Self::PAYLOAD_START
    }
    pub const SIZE: usize = size_of::<Self>();
}

#[test]
fn deserialize() {
    #[cfg(target_endian = "little")]
    let buffer: [u8; Header::SIZE] = [
        0x1, 0x0, 0x0, 0x0, // Object ID
        0x2, 0x0, // opcode
        0x8, 0x0, // len
    ];

    #[cfg(target_endian = "big")]
    let buffer: [u8; Header::SIZE] = [
        0x0, 0x0, 0x0, 0x1, // Object ID
        0x0, 0x2, // opcode
        0x0, 0x8, // len
    ];

    let header = Header::from_bytes(&buffer).expect("Failed to serilise header from bytes");

    assert!(header.object_id == 1);
    assert!(header.opcode == 2);
    assert!(header.len == 8);
}
