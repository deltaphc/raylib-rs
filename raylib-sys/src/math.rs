use crate::{Matrix, Quaternion, Vector2, Vector3, Vector4};
use mint;
use std::mem;

macro_rules! mint_transmutable {
    ($ffit:ty, $mt:ty) => {
        impl From<$mt> for $ffit {
            fn from(value: $mt) -> Self {
                unsafe { mem::transmute(value) }
            }
        }

        impl Into<$mt> for $ffit {
            fn into(self) -> $mt {
                unsafe { mem::transmute(self) }
            }
        }
    };
}

mint_transmutable!(Vector2, mint::Vector2<f32>);
mint_transmutable!(Vector3, mint::Vector3<f32>);
mint_transmutable!(Vector4, mint::Vector4<f32>);

mint_transmutable!(Matrix, mint::ColumnMatrix4<f32>);
mint_transmutable!(Quaternion, mint::Quaternion<f32>);

#[test]
fn math_test() {
  // Do some transmutation tests.
}