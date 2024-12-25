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

    pub fn subtract(self, rhs: PolynomialReborn) -> PolynomialReborn {
        self.add(rhs.scale(-1))
    }

    pub fn remove_leading_zeroes(self) -> PolynomialReborn {
        let result: Vec<i64> =
            self.coefficients.into_iter()
                .rev().skip_while(|&x| x == 0)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect();

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
        self.divide(modulus).1
    }

    pub fn divide(self, rhs: PolynomialReborn) -> (PolynomialReborn, PolynomialReborn) {
        let lhs_deg = self.degree();
        let rhs_deg = rhs.degree();

        if rhs_deg == 0 {
            //TODO cannot divide by 0
        }

        let mut quot = PolynomialReborn::new(Vec::with_capacity(1 + lhs_deg.saturating_sub(rhs_deg)));
        for _i in 0..quot.coefficients.capacity() {
            quot.coefficients.push(0);
        }

        let mut rem = self.clone();

        let mut rhs = rhs.clone();
        for _i in 0..(lhs_deg - rhs_deg) {
            rhs.coefficients.insert(0, 0);
        }

        for i in (0..(1 + lhs_deg - rhs_deg)).rev() {
            quot.coefficients[i] = rem.coefficients[rhs_deg + i] / rhs.coefficients[rhs_deg + i];

            rem = rem.subtract(rhs.clone().scale(quot.coefficients[i]).clone());

            rhs.coefficients.remove(0);
        }

        let quot_deg = quot.degree();
        let rem_deg = rem.degree();
        quot.coefficients.truncate(quot_deg + 1);
        rem.coefficients.truncate(rem_deg + 1);
        (quot, rem)
    }

    pub fn degree(&self) -> usize {
        self.coefficients.iter()
            .enumerate()
            .rev()
            .find(|&(_i, &n)| n != 0)
            .map(|(i, _n)| i)
            .unwrap_or(0) // should this be - infinity
    }


    pub fn scale(self, factor: i64) -> PolynomialReborn {
        let new_coefficients = self.coefficients.iter().map(|&x| factor * x).collect();
        PolynomialReborn::new(new_coefficients)
    }
}
