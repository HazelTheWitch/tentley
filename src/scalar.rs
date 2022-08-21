pub trait Scalar: Copy + Sized {}

impl<T: Copy> Scalar for T {}

pub trait Zero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

pub trait One {
    fn one() -> Self;
    fn is_one(&self) -> bool;
}

macro_rules! impl_numeric {
    ($t: ty, $zero: expr, $one: expr) => {
        impl Zero for $t {
            fn zero() -> Self { $zero }
            fn is_zero(&self) -> bool { self == &($zero)}
        }

        impl One for $t {
            fn one() -> Self { $one }
            fn is_one(&self) -> bool { self == &($one)}
        }
    };
}

impl_numeric!(u8, 0u8, 1u8);
impl_numeric!(u16, 0u16, 1u16);
impl_numeric!(u32, 0u32, 1u32);
impl_numeric!(u64, 0u64, 1u64);
impl_numeric!(u128, 0u128, 1u128);

impl_numeric!(i8, 0i8, 1i8);
impl_numeric!(i16, 0i16, 1i16);
impl_numeric!(i32, 0i32, 1i32);
impl_numeric!(i64, 0i64, 1i64);
impl_numeric!(i128, 0i128, 1i128);

impl_numeric!(f32, 0.0f32, 1.0f32);
impl_numeric!(f64, 0.0f64, 1.0f64);
