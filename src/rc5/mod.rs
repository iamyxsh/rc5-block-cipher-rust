pub trait Word:
    Clone
    + Copy
    + std::fmt::Debug
    + PartialEq
    + num::traits::WrappingAdd
    + num::traits::WrappingSub
    + std::ops::Shl<Output = Self>
    + std::ops::Shr<Output = Self>
    + std::ops::BitOr<Output = Self>
    + std::ops::BitAnd<Output = Self>
{
    const ZERO: Self;

    const P: Self;
    const Q: Self;

    const BYTES: usize;

    fn from_u8(val: u8) -> Self;
    fn from_usize(val: usize) -> Self;

    fn to_usize(self) -> usize;
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
    #[inline]
    fn to_usize(self) -> usize {
        self as usize
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
    #[inline]
    fn to_usize(self) -> usize {
        self as usize
    }
}

#[inline]
pub fn rotl<W: Word>(val: W, shift: W) -> W {
    let bits = W::BYTES * 8;
    let s = shift.to_usize() % bits;
    if s == 0 {
        val
    } else {
        (val << W::from_usize(s)) | (val >> W::from_usize(bits - s))
    }
}

#[inline]
pub fn rotr<W: Word>(val: W, shift: W) -> W {
    let bits = W::BYTES * 8;
    let s = shift.to_usize() % bits;
    if s == 0 {
        val
    } else {
        (val >> W::from_usize(s)) | (val << W::from_usize(bits - s))
    }
}
