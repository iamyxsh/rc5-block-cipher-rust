pub trait Word:
    Clone
    + Copy
    + std::fmt::Debug
    + PartialEq
    + num::traits::WrappingAdd
    + num::traits::WrappingSub
    + std::ops::Shl<Output = Self>
    + std::ops::Shr<Output = Self>
{
    const ZERO: Self;

    const P: Self;
    const Q: Self;

    const BYTES: usize;

    fn from_u8(val: u8) -> Self;
    fn from_usize(val: usize) -> Self;
}

impl Word for u8 {
    const ZERO: Self = 0;
    const P: Self = 0;
    const Q: Self = 0;
    const BYTES: usize = 1;

    #[inline]
    fn from_u8(val: u8) -> Self {
        val
    }

    #[inline]
    fn from_usize(val: usize) -> Self {
        val as u8
    }
}

impl Word for u32 {
    const ZERO: Self = 0;
    const P: Self = 0xB7E1_5163;
    const Q: Self = 0x9E37_79B9;
    const BYTES: usize = 4;

    #[inline]
    fn from_u8(val: u8) -> Self {
        val as u32
    }

    #[inline]
    fn from_usize(val: usize) -> Self {
        val as u32
    }
}
