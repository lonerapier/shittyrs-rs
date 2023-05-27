include!("./table.rs");

use crate::ff::Fp;

#[derive(Debug, Clone, Copy)]
pub struct GF2k;

impl Fp for GF2k {
    const ORDER: usize = 256;

    const PRIME_POLY: usize = 0x11b;

    type Elem = u8;

    #[inline]
    fn zero() -> Self::Elem {
        0
    }

    #[inline]
    fn one() -> Self::Elem {
        1
    }

    #[inline]
    fn add(a: Self::Elem, b: Self::Elem) -> Self::Elem {
        a ^ b
    }

    #[inline]
    fn sub(a: Self::Elem, b: Self::Elem) -> Self::Elem {
        a ^ b
    }

    #[inline]
    fn mul(a: Self::Elem, b: Self::Elem) -> Self::Elem {
        if a == 0 || b == 0 {
            return 0;
        }

        EXP_TABLE[LOG_TABLE[a as usize] as usize + LOG_TABLE[b as usize] as usize]
    }

    #[inline]
    fn exp(a: Self::Elem, power: usize) -> Self::Elem {
        if a == 0 {
            return 0;
        }
        if power == 0 {
            return 1;
        }

        let mut res_exp = LOG_TABLE[a as usize] * power as u8;
        while res_exp >= Self::ORDER as u8 {
            res_exp -= Self::ORDER as u8;
        }

        EXP_TABLE[res_exp as usize]
    }

    #[inline]
    fn inverse(a: Self::Elem) -> Option<Self::Elem> {
        if a == 0 {
            return None;
        }

        Some(EXP_TABLE[Self::ORDER - LOG_TABLE[a as usize] as usize])
    }

    #[inline]
    fn div(a: Self::Elem, b: Self::Elem) -> Option<Self::Elem> {
        if b == 0 {
            return None;
        }

        if a == 0 {
            return Some(0);
        }

        Some(Self::mul(a, Self::inverse(b).unwrap()))
    }

    #[inline]
    fn neg(a: Self::Elem) -> Self::Elem {
        a
    }

    #[inline]
    fn eq(a: &Self::Elem, b: &Self::Elem) -> bool {
        a == b
    }

    fn from_u8(a: u8) -> Self::Elem {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(GF2k::add(0, 0), 0);
        assert_eq!(GF2k::add(0, 1), 1);
        assert_eq!(GF2k::add(1, 0), 1);
        assert_eq!(GF2k::add(1, 1), 0);
    }

    #[test]
    fn test_sub() {
        assert_eq!(GF2k::sub(0, 0), 0);
        assert_eq!(GF2k::sub(0, 1), 1);
        assert_eq!(GF2k::sub(1, 0), 1);
        assert_eq!(GF2k::sub(1, 1), 0);
    }

    #[test]
    fn test_mul() {
        assert_eq!(GF2k::mul(0, 0), 0);
        assert_eq!(GF2k::mul(0, 1), 0);
        assert_eq!(GF2k::mul(1, 0), 0);
        assert_eq!(GF2k::mul(1, 1), 1);
    }

    #[test]
    fn test_exp() {
        assert_eq!(GF2k::exp(0, 0), 1);
        assert_eq!(GF2k::exp(0, 1), 0);
        assert_eq!(GF2k::exp(1, 0), 1);
        assert_eq!(GF2k::exp(1, 1), 1);
    }

    #[test]
    fn test_inverse() {
        assert_eq!(GF2k::inverse(0), None);
        assert_eq!(GF2k::inverse(1), Some(1));
    }

    #[test]
    fn test_div() {
        assert_eq!(GF2k::div(0, 0), None);
        assert_eq!(GF2k::div(0, 1), Some(0));
        assert_eq!(GF2k::div(1, 0), None);
        assert_eq!(GF2k::div(1, 1), Some(1));
    }

    #[test]
    fn test_neg() {
        assert_eq!(GF2k::neg(0), 0);
        assert_eq!(GF2k::neg(1), 1);
    }
}
