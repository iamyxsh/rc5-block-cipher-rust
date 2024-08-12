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

pub fn expand_key<W: Word>(key: &[u8], rounds: usize) -> Vec<W> {
    let t = 2 * (rounds + 1);
    let bytes = W::BYTES;
    let c = ((8 * key.len()) + (8 * bytes - 1)) / (8 * bytes);

    let mut l = vec![W::ZERO; c];
    for (i, &b) in key.iter().rev().enumerate() {
        let idx = i / bytes;
        l[idx] = (l[idx] << W::from_u8(8)).wrapping_add(&W::from_u8(b));
    }

    let mut s = Vec::with_capacity(t);
    s.push(W::P);
    for i in 1..t {
        s.push(s[i - 1].wrapping_add(&W::Q));
    }

    let mut a = W::ZERO;
    let mut b = W::ZERO;
    let mut i = 0;
    let mut j = 0;
    for _ in 0..(3 * std::cmp::max(t, c)) {
        a = rotl(s[i].wrapping_add(&a).wrapping_add(&b), W::from_u8(3));
        s[i] = a;
        b = rotl(l[j].wrapping_add(&a).wrapping_add(&b), a.wrapping_add(&b));
        l[j] = b;
        i = (i + 1) % t;
        j = (j + 1) % c;
    }

    s
}
