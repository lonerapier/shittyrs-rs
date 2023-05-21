include!("./table.rs");

use crate::ff::Fp;

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
}

// impl std::ops::AddAssign for GF2k {
//     fn add_assign(&mut self, rhs: Self) {
//         let res = Self::add(*self, rhs);
//     }
// }
