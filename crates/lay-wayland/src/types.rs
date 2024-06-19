use std::fmt::Debug;

use crate::WlFixed;

impl WlFixed {
    pub fn from_f32(f: f32) -> Self {
        Self((f * 256.0) as i32)
    }
    pub const fn from_i32(i: i32) -> Self {
        Self(i * 256)
    }
    pub const fn to_i32(s: Self) -> i32 {
        s.0 / 256
    }
    pub fn to_f32(s: Self) -> f32 {
        (s.0 as f32) / 256.0
    }
}
impl From<f32> for WlFixed {
    fn from(f: f32) -> Self {
        WlFixed::from_f32(f)
    }
}
impl From<i32> for WlFixed {
    fn from(value: i32) -> Self {
        WlFixed::from_i32(value)
    }
}

impl From<WlFixed> for i32 {
    fn from(value: WlFixed) -> Self {
        WlFixed::to_i32(value)
    }
}
impl From<WlFixed> for f32 {
    fn from(value: WlFixed) -> Self {
        WlFixed::to_f32(value)
    }
}

impl Debug for WlFixed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("WlFixed")
            .field(&Self::to_f32(*self))
            .finish()
    }
}
