use crate::ff::{FieldElement, Fp};

/// Trait for polynomial operations
pub trait PolynomialOps: Sized {
    /// Returns the degree of the polynomial
    fn degree(&self) -> usize;

    /// Returns the number of coefficients in the polynomial
    fn len(&self) -> usize;

    /// Negates a polynomial
    fn neg(&self) -> Self;

    /// adds two polynomials
    fn add(&self, other: &Self) -> Self;

    /// subtracts two polynomials
    fn sub(&self, other: &Self) -> Self;

    /// multiplies two polynomials
    fn mul(&self, other: &Self) -> Self;

    /// Divides two polynomials and returns quotient and remainder
    fn div(&self, divisor: &Self) -> (Self, Self);

    /// Divides two polynomials and returns remainder
    fn rem(&self, divisor: &Self) -> Self;
}

/// Polynomial over a field
#[derive(Debug, Clone)]
pub struct Polynomial<E> {
    pub coeffs: Vec<E>,
}

impl<F: Fp> Polynomial<FieldElement<F>> {
    pub fn new(coeffs: &[FieldElement<F>]) -> Self {
        // TODO: done some sure shit here which is not rust worthy.
        let from = match coeffs.iter().position(|x| *x != FieldElement::zero()) {
            Some(i) => i,
            None => coeffs.len() + 1,
        };

        if from > coeffs.len() {
            panic!("Empty polynomial");
        }

        Polynomial {
            coeffs: coeffs[from..].to_vec(),
        }
    }

    // value of a polynomial at a point using Horner's method.
    pub fn evaluate(&self, at: &FieldElement<F>) -> FieldElement<F> {
        let mut result = FieldElement::zero();

        for x in self.coeffs.iter() {
            result = x + &(result * at);
        }

        result
    }
}

impl<F: Fp> PolynomialOps for Polynomial<FieldElement<F>> {
    fn degree(&self) -> usize {
        self.coeffs.len() - 1
    }

    fn len(&self) -> usize {
        self.coeffs.len()
    }

    fn add(&self, other: &Self) -> Self {
        // TODO: what if carry addition?
        let mut coefficients: Vec<FieldElement<F>> =
            vec![FieldElement::zero(); std::cmp::max(self.len(), other.len())];

        for i in 0..self.len() - 1 {
            coefficients[i] += &self.coeffs[i];
        }

        for i in 0..other.len() - 1 {
            coefficients[i] += &other.coeffs[i];
        }

        Polynomial {
            coeffs: coefficients,
        }
    }

    // TODO: return reference?
    fn neg(&self) -> Self {
        return self.clone();
    }

    fn sub(&self, other: &Self) -> Self {
        // TODO: self + -other?
        return self.add(&other.neg());
    }

    /// Implements naive multiplication algorithm for numbers.
    ///
    /// **Note:** carryless addition as described in the Field.
    fn mul(&self, other: &Self) -> Self {
        let mut coefficients = vec![FieldElement::zero(); self.len() + other.len()];

        for i in (0..self.len()).rev() {
            if self.coeffs[i] == FieldElement::zero() {
                continue;
            }
            for j in (0..other.len()).rev() {
                if other.coeffs[j] == FieldElement::zero() {
                    continue;
                }

                coefficients[i + j + 1] += &self.coeffs[i] * &other.coeffs[j];
            }
        }

        Polynomial {
            coeffs: coefficients,
        }
    }

    /// Implements Synthetic division method along with some optimisations mentioned [here](https://research.swtch.com/field).
    fn div(&self, divisor: &Self) -> (Self, Self) {
        let mut coefficients = self.coeffs.clone();

        for i in 0..(self.len() - divisor.len() + 1) {
            if self.coeffs[i] == FieldElement::zero() {
                continue;
            }

            for j in 1..(divisor.len()) {
                coefficients[i + j] += &divisor.coeffs[j] * &self.coeffs[i];
            }
        }

        let (quotient, remainder) = coefficients.split_at(self.len() - (divisor.len() + 1));

        return (
            Polynomial {
                coeffs: quotient.to_vec(),
            },
            Polynomial {
                coeffs: remainder.to_vec(),
            },
        );
    }

    fn rem(&self, divisor: &Self) -> Self {
        self.div(divisor).1
    }
}

#[cfg(test)]
mod tests {

    use super::{Polynomial, PolynomialOps};
    use crate::backend::gf2_8::GF2k;
    use crate::ff::{FieldElement, Fp};
    type Elem = FieldElement<GF2k>;

    #[test]
    fn new_polynomial() {
        let coeffs = [Elem::one(); 3];
        let poly = Polynomial::new(&coeffs);

        assert_eq!(poly.len(), coeffs.len());
    }

    #[test]
    fn evaluate() {
        let poly = Polynomial::new(&[Elem::one(); 3]);

        assert_eq!(poly.evaluate(&Elem::new(10)), 79.into());
    }

    #[test]
    fn add() {
        let poly1 = Polynomial::new(&[Elem::one(), Elem::new(10)]);
        let poly2 = Polynomial::new(&[Elem::one(), Elem::new(10)]);

        let poly3 = poly1.add(&poly2);

        assert_eq!(poly3.coeffs, vec![Elem::new(0), Elem::new(0)]);
    }

    #[test]
    fn sub() {
        let poly1 = Polynomial::new(&[Elem::one(), Elem::new(10)]);
        let poly2 = Polynomial::new(&[Elem::one(), Elem::new(10)]);

        let poly3 = poly1.sub(&poly2);

        assert_eq!(poly3.coeffs, vec![Elem::new(0), Elem::new(0)]);
    }

    #[test]
    fn mul() {
        let poly1 = Polynomial::new(&[Elem::one(), Elem::new(1)]);
        let poly2 = Polynomial::new(&[Elem::one(), Elem::new(2)]);

        let poly3 = poly1.mul(&poly2);

        assert_eq!(
            poly3.coeffs,
            vec![Elem::one(), Elem::new(20), Elem::new(100)]
        );
    }

    #[test]
    fn div() {
        let poly1 = Polynomial::new(&[Elem::one(), Elem::new(10)]);
        let poly2 = Polynomial::new(&[Elem::one(), Elem::new(10)]);

        let (poly3, poly4) = poly1.div(&poly2);

        assert_eq!(poly3.coeffs, vec![Elem::one()]);
        assert_eq!(poly4.coeffs, vec![Elem::zero()]);
    }

    #[test]
    fn rem() {
        let poly1 = Polynomial::new(&[Elem::one(), Elem::new(10)]);
        let poly2 = Polynomial::new(&[Elem::one(), Elem::new(10)]);

        let poly3 = poly1.rem(&poly2);

        assert_eq!(poly3.coeffs, vec![Elem::zero()]);
    }

    #[test]
    fn degree() {
        let poly1 = Polynomial::new(&[Elem::one(), Elem::new(10)]);

        assert_eq!(poly1.degree(), 1);
    }

    #[test]
    fn len() {
        let poly1 = Polynomial::new(&[Elem::one(), Elem::new(10)]);

        assert_eq!(poly1.len(), 2);
    }
}
