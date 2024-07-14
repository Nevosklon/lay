use crate::WlFixed;
use core::fmt::Debug;
use std::ops::Deref;

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

/// Implement from trait
/// Example
/// from!(
///     MyType0; [f32; WlFixed::from_f32, i32; WlFixed::from_i32],
/// );
///
///
/// impl core::convert::From<f32>for WlFixed {
///     fn from(v:f32) -> Self {
///         WlFixed::from_f32(v)
///     }
/// }
/// impl core::convert::From<i32>for WlFixed {
///     fn from(v:i32) -> Self {
///         WlFixed::from_i32(v)
///     }
/// }
///    
macro_rules! from {
    ($($for: ty; [$($from: tt; $function: path),*]),*) => {
        $(
            $(
                impl core::convert::From<$from> for $for {
                    fn from(v: $from) -> Self {
                        $function(v)
                    }
                }

            )*
        )*
    };
}

from!(
    WlFixed; [f32; WlFixed::from_f32, i32; WlFixed::from_i32],
    i32; [WlFixed; WlFixed::to_i32],
    f32; [WlFixed; WlFixed::to_f32]
);

macro_rules! WlFixedMath {
    ($([$ops: tt, $trait: tt, $function_name: tt]),*) => {
        $(
            impl core::ops::$trait<f32> for WlFixed {
                type Output = f32;

                fn $function_name(self, rhs: f32) -> f32 {
                    WlFixed::to_f32(self) $ops rhs
                }
            }
        )*
    };
}

macro_rules! WlFixedMathAssign {
    ($([$ops: tt, $trait: tt, $function_name: tt]),*) => {
        $(
            impl core::ops::$trait<f32> for WlFixed {
                fn $function_name(&mut self, rhs: f32) {
                    *self = WlFixed::from_f32(WlFixed::to_f32(self.clone()) $ops rhs);
                }
            }
        )*
    };
}

WlFixedMath!([+, Add, add], [-, Sub, sub], [/, Div, div], [*, Mul, mul]);
WlFixedMathAssign!([+, AddAssign, add_assign], [-, SubAssign, sub_assign], [/, DivAssign, div_assign], [*, MulAssign, mul_assign]);

impl Debug for WlFixed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("WlFixed")
            .field(&Self::to_f32(*self))
            .finish()
    }
}
