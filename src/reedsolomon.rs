use std::vec;

use num_traits::{One, Pow, Zero};

use crate::errors::Errors;

use crate::ff::FieldElement as FE;
use crate::polynomial::PolynomialOps;
use crate::{ff::Fp, polynomial::Polynomial};

#[derive(Debug)]
pub struct ReedSolomon<F: Fp> {
    total_size: usize,
    parity_size: usize,
    generator: FE<F>,
    generator_poly: Polynomial<FE<F>>,
}

fn create_generator_poly<F: Fp>(n: usize, k: usize, gen: &FE<F>) -> Polynomial<FE<F>> {
    let mut p = Polynomial {
        coeffs: vec![FE::one(); 1],
    };

    for i in 1..=(n - k) {
        p = p.mul(&Polynomial {
            coeffs: vec![FE::one(), gen.pow(i)],
        })
    }

    p
}

impl<F: Fp> ReedSolomon<F> {
    pub fn new(n: usize, k: usize, gen: FE<F>) -> Result<ReedSolomon<F>, Errors> {
        if n == 0 {
            return Err(Errors::TooFewNumbers);
        }

        if k == 0 {
            return Err(Errors::TooFewParity);
        }

        if n > F::ORDER {
            return Err(Errors::TooManyNumbers);
        }

        Ok(ReedSolomon {
            total_size: n,
            parity_size: k,
            generator: gen.clone(),
            generator_poly: create_generator_poly(n, k, &gen),
        })
    }

    pub fn encode(&self, message: &str) -> Result<String, Errors> {
        if message.len() == 0 {
            return Err(Errors::InvalidMessage);
        }

        let mut coeffs: Vec<FE<F>> = message.as_bytes().iter().map(|x| FE::from(*x)).collect();

        coeffs.append(vec![FE::zero(); self.parity_size].as_mut());

        let message_poly = Polynomial {
            coeffs: coeffs.clone(),
        };

        let remainder = Polynomial::rem(&message_poly, &self.generator_poly);

        let encoded_poly = Polynomial::sub(&message_poly, &remainder);

        let encoded = encoded_poly
            .coeffs
            .iter()
            .map(|x| x.to_owned().into())
            .collect::<Vec<u8>>()
            .into_iter()
            .map(|x| x as char)
            .collect::<String>();

        Ok(encoded)
    }

    // TODO: implement fast check
    pub fn check(&self, message: &str) -> Result<bool, Errors> {
        if message.len() == 0 {
            return Err(Errors::InvalidMessage);
        }

        let mut coeffs: Vec<FE<F>> = message.as_bytes().iter().map(|x| FE::from(*x)).collect();

        coeffs.append(vec![FE::zero(); self.parity_size].as_mut());

        let message_poly = Polynomial {
            coeffs: coeffs.clone(),
        };

        let remainder = Polynomial::rem(&message_poly, &self.generator_poly);

        Ok(remainder.is_zero())
    }
}
