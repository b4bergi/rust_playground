use crate::fft::Fft;

#[derive(Debug, Clone)]
pub struct PolynomialReborn {
    pub coefficients: Vec<i64>,
}

impl PolynomialReborn {
    pub fn new(coefficients: Vec<i64>) -> PolynomialReborn {
        PolynomialReborn { coefficients }
    }
    pub fn multiply(self, rhs: PolynomialReborn) -> PolynomialReborn {
        let coefficients = Fft::multiply(
            self.coefficients.iter().map(|&x| x as f64).collect(),
            rhs.coefficients.iter().map(|&x| x as f64).collect(),
        );

        let coefficients: Vec<i64> = coefficients
            .iter()
            .map(|&x| x as i64)
            .take_while(|&x| x != 0)
            .collect();
        PolynomialReborn::new(coefficients)
    }
    pub fn add(self, rhs: PolynomialReborn) -> PolynomialReborn {
        let max_len = std::cmp::max(self.coefficients.len(), rhs.coefficients.len());
        let mut result = Vec::with_capacity(max_len);

        for i in 0..max_len {
            result.push(
                self.coefficients.get(i).unwrap_or(&0) + rhs.coefficients.get(i).unwrap_or(&0),
            );
        }

        PolynomialReborn::new(result)
    }

    pub fn remove_leading_zeroes(self) -> PolynomialReborn {
        let result: Vec<i64> = self.coefficients.into_iter().take_while(|&x| x != 0).collect();

        if result.len() == 0 {
            PolynomialReborn::new(vec![0])
        } else {
            PolynomialReborn::new(result)
        }
    }

    pub fn coefficient_modulus(self, modulus: i64) -> PolynomialReborn {
        let reduced_coefficients = self
            .coefficients
            .iter()
            .map(|x| x.rem_euclid(modulus))
            .collect();
        PolynomialReborn::new(reduced_coefficients)
    }

    pub fn polynomial_modulus(self, modulus: PolynomialReborn) -> PolynomialReborn {
        // TODO test this
        let mut new_coefficients = self.coefficients;
        for i in (modulus.coefficients.len()..new_coefficients.len()).rev() {
            for j in 0..modulus.coefficients.len() {
                new_coefficients[i - modulus.coefficients.len() + j] = new_coefficients
                    [i - modulus.coefficients.len() + j]
                    - new_coefficients[i] * modulus.coefficients.get(j).unwrap_or(&0);
            }
        }

        PolynomialReborn::new(new_coefficients)
    }

    pub fn scale(self, factor: i64) -> PolynomialReborn {
        let new_coefficients = self.coefficients.iter().map(|&x| factor * x).collect();
        PolynomialReborn::new(new_coefficients)
    }
}
