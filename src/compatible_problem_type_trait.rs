use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

pub trait CompatibleProblemType:
    Default
    + Clone
    + Copy
    + Into<f64>
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
{
    // Required methods
    fn type_to_key(value: Self) -> u64;
    fn key_to_type(key: u64) -> Self;
    fn identity() -> Self;

    // Provided methods
    fn null() -> Option<Self>
    where
        Self: Sized,
    {
        Some(Self::default())
    }
}

pub trait UnboundedCompatibility: Default {
    // Provided methods
    fn null() -> Option<Self>
    where
        Self: Sized,
    {
        Some(Self::default())
    }

    fn is_unbounded() -> bool {
        false
    }
}

impl<T> UnboundedCompatibility for T
where
    T: CompatibleProblemType,
{
    fn null() -> Option<Self>
    where
        Self: Sized,
    {
        T::null()
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

                fn identity() -> Self {
                    1
                }
            }
        )*
    };
}

impl_CompatibleProblemType_for_unsigned!(u32, u16, u8);

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

                fn identity() -> Self {
                    1.0
                }
            }
        )*
    }
}

impl_CompatibleProblemType_for_floats!(f64, f32);
