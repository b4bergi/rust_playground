use crate::fft::Fft;
use std::ops;
//TODO clean up this file
#[derive(Clone)]
pub struct Polynomial {
    pub coefficients: Vec<i32>,
    pub modulus: i32,
}

impl ops::Add<Polynomial> for Polynomial {
    type Output = Polynomial;
    fn add(self, rhs: Polynomial) -> Polynomial {
        let max_len = std::cmp::max(self.coefficients.len(), rhs.coefficients.len());
        let mut result = Vec::with_capacity(max_len);

        for i in 0..max_len {
            result.push(
                self.coefficients.get(i).unwrap_or(&0) + rhs.coefficients.get(i).unwrap_or(&0),
            );
        }
        let result: Vec<i32> = result.into_iter().take_while(|&x| x != 0).collect();

        if result.len() == 0
        // trim leading zeroes
        {
            Polynomial::new(vec![0], self.modulus)
        } else {
            Polynomial::new(result, self.modulus)
        }
    }
}

impl ops::Mul<Polynomial> for Polynomial {
    type Output = Polynomial;
    fn mul(self, rhs: Polynomial) -> Polynomial {
        let coefficients = Fft::multiply(
            self.coefficients.iter().map(|&x| x as f64).collect(),
            rhs.coefficients.iter().map(|&x| x as f64).collect(),
        );

        let coefficients: Vec<i32> = coefficients
            .iter()
            .map(|&x| x as i32)
            .take_while(|&x| x != 0)
            .collect();

        if coefficients.len() == 0 // trim leading zeroes
        {
            Polynomial::new(vec![0], self.modulus)
        } else {
            Polynomial::new(coefficients, self.modulus)
        }
    }
}

impl Polynomial {
    pub fn new(coefficients: Vec<i32>, modulus: i32) -> Polynomial {
        Polynomial {
            coefficients: coefficients.iter().map(|x| x.rem_euclid(modulus)).collect(),
            modulus,
        }
    }
    pub fn multiply_with_modulus(lhs: Polynomial, rhs: Polynomial, modulus: i32) -> Polynomial {
        let coefficients = Fft::multiply(
            lhs.coefficients.iter().map(|&x| x as f64).collect(),
            rhs.coefficients.iter().map(|&x| x as f64).collect(),
        );

        let coefficients: Vec<i32> = coefficients
            .iter()
            .map(|&x| x as i32)
            .take_while(|&x| x != 0)
            .collect();

        if coefficients.len() == 0
        // trim leading zeroes
        {
            Polynomial::new(vec![0], modulus)
        } else {
            Polynomial::new(coefficients, modulus)
        }
    }
    pub fn add_with_modulus(lhs: Polynomial, rhs: Polynomial, modulus: i32) -> Polynomial {
        let result = add_coefficients(lhs.coefficients, rhs.coefficients);

        return if result.len() == 0
        // trim leading zeroes // TODO make trim zeroes a function
        {
            let asdf = Polynomial::new(vec![0], modulus);
            asdf
        } else {
            let asdf = Polynomial::new(result, modulus);
            asdf
        };

        fn add_coefficients(lhs: Vec<i32>, rhs: Vec<i32>) -> Vec<i32> {
            let max_len = std::cmp::max(lhs.len(), rhs.len());
            let mut result = Vec::with_capacity(max_len);

            for i in 0..max_len {
                result.push(lhs.get(i).unwrap_or(&0) + rhs.get(i).unwrap_or(&0));
            }
            result.into_iter().take_while(|&x| x != 0).collect()
        }
    }
}
