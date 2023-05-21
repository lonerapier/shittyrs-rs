/// Trait for finite fields
pub trait Fp: Sized {
    /// The order of the field
    const ORDER: usize;

    /// The prime polynomial of the field
    const PRIME_POLY: usize;

    /// Element of the field
    type Elem: Sized + Copy + Default + Clone + PartialEq + Eq + std::fmt::Debug;

    /// Returns the addititve identity of the field
    fn zero() -> Self::Elem;

    /// Returns the multiplicative identity of the field
    fn one() -> Self::Elem;

    /// Adds two elements of the field
    fn add(a: Self::Elem, b: Self::Elem) -> Self::Elem;

    /// Subtracts two elements of the field
    fn sub(a: Self::Elem, b: Self::Elem) -> Self::Elem;

    /// Multiplies two elements of the field
    fn mul(a: Self::Elem, b: Self::Elem) -> Self::Elem;

    /// Inverts an element of the field
    fn inverse(a: Self::Elem) -> Option<Self::Elem>;

    /// Divides two elements of the field
    fn div(a: Self::Elem, b: Self::Elem) -> Option<Self::Elem>;

    /// Exponentiates an element of the field
    fn exp(a: Self::Elem, b: usize) -> Self::Elem;

    /// Negates an element of the field
    fn neg(a: Self::Elem) -> Self::Elem;
}

// pub trait FpConfig<'a>: Sized {
//     const PRIME_POLY: u64;
//     const EXP: u64;
//     const exp_table: &'a [u64; Self::EXP];
//
//     fn add_assign(a: &mut GF2k<Self>, b: &GF2k<Self>);
//     fn sub_assign(a: &mut GF2k<Self>, b: &GF2k<Self>);
//     fn mul_assign(a: &mut GF2k<Self>, b: &GF2k<Self>);
//
//     fn pow(a: &GF2k<Self>, b: u64) -> GF2k<Self>;
//
//     // fn inverse(a: &GF2k<Self>) -> Option<GF2k<Self>>;
// }
//
// pub struct GF2k<'a, P: FpConfig>(pub u64, PhantomData<P>);
//
// impl<P: FpConfig> FpConfig for GF2k<P> {
//     const PRIME_POLY: u64 = 0x11b;
//
//     fn add_assign(a: &mut GF2k<Self>, b: &GF2k<Self>) {
//         a.0 ^= b.0;
//     }
//
//     fn sub_assign(a: &mut GF2k<Self>, b: &GF2k<Self>) {
//         a.0 ^= b.0;
//     }
//
//     fn mul_assign(a: &mut GF2k<Self>, b: &GF2k<Self>) {
//         let mut result = 0;
//         let mut b = b.0;
//         let mut a = a.0;
//         while b != 0 {
//             if b & 1 != 0 {
//                 result ^= a;
//             }
//             b >>= 1;
//             a <<= 1;
//             if a & 0x100 != 0 {
//                 a ^= Self::PRIME_POLY;
//             }
//         }
//         a = result;
//     }
//
//     fn pow(a: &GF2k<Self>, b: u64) -> GF2k<Self> {
//         let mut result = 1;
//         let a = a.0;
//         let mut b = b;
//         while b != 0 {
//             if b & 1 != 0 {
//                 result *= a;
//             }
//             b >>= 1;
//         }
//         GF2k(result, PhantomData)
//     }
// }
//
// impl<P: FpConfig> GF2k<P> {
//     pub fn new(value: u64) -> Self {
//         GF2k(value, PhantomData)
//     }
// }
//
// impl<P: FpConfig> std::ops::AddAssign for GF2k<P> {
//     fn add_assign(&mut self, rhs: Self) {
//         P::add_assign(self, &rhs);
//     }
// }
//
// impl<P: FpConfig> std::ops::SubAssign for GF2k<P> {
//     fn sub_assign(&mut self, rhs: Self) {
//         P::sub_assign(self, &rhs);
//     }
// }
//
// impl<P: FpConfig> std::ops::MulAssign for GF2k<P> {
//     fn mul_assign(&mut self, rhs: Self) {
//         P::mul_assign(self, &rhs);
//     }
// }
//
// impl<P: FpConfig> std::ops::Add for GF2k<P> {
//     type Output = Self;
//
//     fn add(mut self, rhs: Self) -> Self {
//         self += rhs;
//         self
//     }
// }
//
// impl<P: FpConfig> std::ops::Sub for GF2k<P> {
//     type Output = Self;
//
//     fn sub(mut self, rhs: Self) -> Self {
//         self -= rhs;
//         self
//     }
// }
//
// impl<P: FpConfig> std::ops::Mul for GF2k<P> {
//     type Output = Self;
//
//     fn mul(mut self, rhs: Self) -> Self {
//         self *= rhs;
//         self
//     }
// }
//
// impl<P: FpConfig> std::ops::Neg for GF2k<P> {
//     type Output = Self;
//
//     fn neg(self) -> Self {
//         self
//     }
// }
