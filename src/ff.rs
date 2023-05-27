/// Trait for finite fields
pub trait Fp: Sized + Clone {
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

    /// Returns a boolean indicating whether `a` and `b` are equal or not.
    fn eq(a: &Self::Elem, b: &Self::Elem) -> bool;

    fn from_u8(a: u8) -> Self::Elem;
}

#[derive(Debug, Clone, Copy)]
pub struct FieldElement<F: Fp> {
    value: F::Elem,
}

impl<F: Fp> From<u8> for FieldElement<F> {
    fn from(value: u8) -> Self {
        Self {
            value: F::from_u8(value),
        }
    }
}

impl<F: Fp> std::ops::Add for FieldElement<F> {
    type Output = Self;
    fn add(self, rhs: FieldElement<F>) -> Self {
        Self {
            value: F::add(self.value, rhs.value),
        }
    }
}

impl<F: Fp> std::ops::Add for &FieldElement<F> {
    type Output = FieldElement<F>;
    fn add(self, rhs: &FieldElement<F>) -> Self::Output {
        Self::Output {
            value: F::add(self.value.to_owned(), rhs.value),
        }
    }
}

impl<F: Fp> std::ops::AddAssign for FieldElement<F> {
    fn add_assign(&mut self, rhs: Self) {
        self.value = F::add(self.value, rhs.value);
    }
}

impl<F: Fp> std::ops::AddAssign<&FieldElement<F>> for FieldElement<F> {
    fn add_assign(&mut self, rhs: &Self) {
        self.value = F::add(self.value, rhs.value);
    }
}

impl<F: Fp> std::ops::Sub for FieldElement<F> {
    type Output = Self;
    fn sub(self, rhs: FieldElement<F>) -> Self {
        Self {
            value: F::sub(self.value, rhs.value),
        }
    }
}

impl<F: Fp> std::ops::SubAssign for FieldElement<F> {
    fn sub_assign(&mut self, rhs: Self) {
        self.value = F::sub(self.value, rhs.value);
    }
}

impl<F: Fp> std::ops::Mul for FieldElement<F> {
    type Output = Self;
    fn mul(self, rhs: FieldElement<F>) -> Self {
        Self {
            value: F::mul(self.value, rhs.value),
        }
    }
}

impl<F: Fp> std::ops::MulAssign for FieldElement<F> {
    fn mul_assign(&mut self, rhs: Self) {
        self.value = F::mul(self.value, rhs.value);
    }
}

impl<F: Fp> std::ops::Mul<&FieldElement<F>> for FieldElement<F> {
    type Output = Self;
    fn mul(self, rhs: &FieldElement<F>) -> Self {
        Self {
            value: F::mul(self.value, rhs.value),
        }
    }
}

impl<F: Fp> std::ops::Mul<&FieldElement<F>> for &FieldElement<F> {
    type Output = FieldElement<F>;
    fn mul(self, rhs: &FieldElement<F>) -> Self::Output {
        Self::Output {
            value: F::mul(self.value, rhs.value),
        }
    }
}

impl<F: Fp> std::ops::Div for FieldElement<F> {
    type Output = Self;
    fn div(self, rhs: FieldElement<F>) -> Self {
        Self {
            value: F::div(self.value, rhs.value).unwrap(),
        }
    }
}

impl<F: Fp> std::ops::DivAssign for FieldElement<F> {
    fn div_assign(&mut self, rhs: Self) {
        self.value = F::div(self.value, rhs.value).unwrap();
    }
}

impl<F: Fp> std::ops::Neg for FieldElement<F> {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            value: F::neg(self.value),
        }
    }
}

impl<F: Fp> std::ops::Neg for &FieldElement<F> {
    type Output = FieldElement<F>;

    fn neg(self) -> Self::Output {
        FieldElement {
            value: F::neg(self.value),
        }
    }
}

impl<F: Fp> std::cmp::PartialEq for FieldElement<F> {
    fn eq(&self, other: &Self) -> bool {
        F::eq(&self.value, &other.value)
    }
}

impl<F: Fp> FieldElement<F> {
    pub fn new(value: F::Elem) -> Self {
        Self { value }
    }

    pub fn zero() -> Self {
        Self { value: F::zero() }
    }

    pub fn one() -> Self {
        Self { value: F::one() }
    }
}
