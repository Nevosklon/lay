use crate::{WlFixed, Wl_I32, Wl_U32, Word};
macro_rules! wl_types {
    ($($t: ty),*) => {
        $(
        impl crate::WlType for $t {
            type WlType<'a> = $t;
            fn wl_type(buffer: &[u8]) -> Option<$t> {
                fn internal_wl_type(buffer: &[u8]) -> Option<$t> {
                    if buffer.len() < std::mem::size_of::<$t>() {
                        return None;
                    };
                    assert!(buffer.len() >= std::mem::size_of::<$t>());

                    #[cfg(target_endian = "little")]
                    return {
                        Some(<$t>::from_le_bytes(
                            <[u8; std::mem::size_of::<$t>()]>::try_from(buffer).unwrap(),
                        ))
                    };

                    #[cfg(target_endian = "big")]
                    return {
                        Some(<$t>::from_le_bytes(
                            <[u8; std::mem::size_of::<$t>()]>::try_from(buffer).unwrap(),
                        ))
                    };
                }
                internal_wl_type(buffer)
            }

        fn write(value: Self::WlType<'_>, buffer: &mut [u8]) -> Option<()>{
            const fn wl_bytes(i: $t) -> [u8; std::mem::size_of::<$t>()] {
                #[cfg(target_endian = "little")]
                return i.to_le_bytes();

                #[cfg(target_endian = "big")]
                return i.to_be_bytes();
            }
            if buffer.len() < std::mem::size_of::<$t>() {return None};

            buffer.copy_from_slice(&wl_bytes(value));

            return Some(());

        }
    }
        )*
    };
}

wl_types!(u8, i8, u16, i16, u32, i32, u64, i64);
impl Word {
    pub const SIZE: usize = 4;
}
