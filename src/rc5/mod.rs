pub trait Word:
    Clone
    + Copy
    + std::fmt::Debug
    + std::ops::BitXor<Output = Self>
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

#[inline]
pub fn encrypt<W: Word>(plaintext: [W; 2], key: &[u8], rounds: usize) -> [W; 2] {
    let sched = expand_key::<W>(key, rounds);
    let mut a = plaintext[0].wrapping_add(&sched[0]);
    let mut b = plaintext[1].wrapping_add(&sched[1]);

    for i in 1..=rounds {
        a = rotl(a ^ b, b).wrapping_add(&sched[2 * i]);
        b = rotl(b ^ a, a).wrapping_add(&sched[2 * i + 1]);
    }
    [a, b]
}

#[inline]
pub fn decrypt<W: Word>(ciphertext: [W; 2], key: &[u8], rounds: usize) -> [W; 2] {
    let sched = expand_key::<W>(key, rounds);
    let mut a = ciphertext[0];
    let mut b = ciphertext[1];

    for i in (1..=rounds).rev() {
        b = rotr(b.wrapping_sub(&sched[2 * i + 1]), a) ^ a;
        a = rotr(a.wrapping_sub(&sched[2 * i]), b) ^ b;
    }
    [a.wrapping_sub(&sched[0]), b.wrapping_sub(&sched[1])]
}

#[cfg(test)]
mod encrypt_decrypt_tests {
    use super::*;

    const ROUNDS: usize = 12;

    #[test]
    fn u8_round_trip_minimal() {
        let key = [0u8; 1];
        let plaintext = [0u8, 0u8];
        let cipher = encrypt(plaintext, &key, 1);
        let recovered = decrypt(cipher, &key, 1);
        assert_eq!(recovered, plaintext);
    }

    #[test]
    fn u32_zero_key_known_ciphertext() {
        let key = vec![0u8; 16];
        let pt = [0u32, 0u32];
        let ct = encrypt(pt, &key, ROUNDS);
        assert_eq!(ct, [0xEEDBA521, 0x6D8F4B15]);
        assert_eq!(decrypt(ct, &key, ROUNDS), pt);
    }

    #[test]
    fn u32_varied_key_round_trip() {
        let key = vec![0xAAu8; 16];
        let pt = [0x12345678_u32, 0x9ABCDEF0];
        let ct = encrypt(pt, &key, ROUNDS);
        let recovered = decrypt(ct, &key, ROUNDS);
        assert_eq!(recovered, pt, "round-trip failed for RAND key");
    }

    #[test]
    fn u32_fixed_key_specific_vector() {
        let key = vec![
            0x91, 0x5F, 0x46, 0x19, 0xBE, 0x41, 0xB2, 0x51, 0x63, 0x55, 0xA5, 0x01, 0x10, 0xA9,
            0xCE, 0x91,
        ];
        let pt: [u32; 2] = [0x12345678, 0x9ABCDEF0];
        let ct = encrypt(pt, &key, ROUNDS);
        assert_eq!(ct, [0xAC13C0F7, 0x52892B5B]);
        assert_eq!(decrypt(ct, &key, ROUNDS), pt);
    }
}
