use crate::WlType;
use crate::{Header, HeaderLen, ObjectID, Opcode, Word};

impl Header {
    pub const fn new<T>(object_id: ObjectID, opcode: Opcode) -> Self {
        const { assert!(std::mem::size_of::<Self>() == 8) };

        const {
            let size: isize =
                std::mem::size_of::<T>() as isize - std::mem::size_of::<Header>() as isize;

            assert!(size > 0, "Message length must be larger than 0");
            assert!(
                size < (u16::MAX as isize),
                "Message length must be less than 0xFFFF"
            );
        };
        let len: u16 =
            const { (std::mem::size_of::<Self>() + std::mem::size_of::<T>()) as HeaderLen };
        Self {
            object_id,
            len,
            opcode,
        }
    }
    pub fn from_bytes(buffer: &[u8]) -> Option<Self> {
        if buffer.len() < std::mem::size_of::<Self>() {
            return None;
        };
        assert!(buffer.len() > std::mem::size_of::<Self>());

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
}
