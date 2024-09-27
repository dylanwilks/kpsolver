use std::ops::{Add, AddAssign, SubAssign};

//in the future allow partialord to work (closed paths)
pub trait CompatibleProblemType: Default + Clone + Copy + 
PartialOrd + Add<Output = Self> + AddAssign + SubAssign {
    // Required methods
    fn type_to_key(value: Self) -> u64;
    fn key_to_type(key: u64) -> Self; //somehow assert isomorphism without testing?

    // Provided methods
    fn scale(self, scale: usize) -> Self {
        let mut x = self;
        for _ in 0..scale {
            x += x;
        }

        x
    }
}

macro_rules! impl_CompatibleProblemType_for_unsigned {
    ( $( $type:ty ),* ) => {
        $(
            impl CompatibleProblemType for $type {
                fn type_to_key(value: Self) -> u64 {
                    value as u64
                }

                fn key_to_type(key: u64) -> Self {
                    key as Self
                }

                fn scale(self, scale: usize) -> Self {
                    self * scale as $type
                }
            }
        )*
    };
}

impl_CompatibleProblemType_for_unsigned!(usize, u64, u32, u16, u8);

macro_rules! impl_CompatibleProblemType_for_floats {
    ( $( $type:ty ),* ) => {
        $(
            impl CompatibleProblemType for $type {
                fn type_to_key(value: Self) -> u64 {
                    value.to_bits() as u64
                }

                fn key_to_type(key: u64) -> Self {
                    unsafe { std::mem::transmute::<u64, f64>(key) as Self }
                }

                fn scale(self, scale: usize) -> Self {
                    self * scale as $type
                }
            }
        )*
    }
}

impl_CompatibleProblemType_for_floats!(f64, f32);
