use crate::fft::Fft;

pub struct Polynomial {
    pub coefficients: Vec<i32>,
}

impl Polynomial {
    pub fn new(coefficients: Vec<i32>) -> Polynomial {
        Polynomial { coefficients }
    }

    pub fn add(self, other: Polynomial) -> Polynomial {
        let max_len = std::cmp::max(self.coefficients.len(), other.coefficients.len());
        let mut result = Vec::with_capacity(max_len);

        for i in 0..max_len {
            result.push(
                self.coefficients.get(i).unwrap_or(&0) + other.coefficients.get(i).unwrap_or(&0),
            );
        }
        let result: Vec<i32> = result.into_iter().take_while(|&x| x != 0).collect();

        if result.len() == 0 // trim leading zeroes
        {
            Polynomial::new(vec![0])
        } else {
            Polynomial::new(result)
        }
    }

    pub fn multiply(self, other: Polynomial) -> Polynomial {
        let coefficients = Fft::multiply(
            self.coefficients.iter().map(|&x| x as f64).collect(),
            other.coefficients.iter().map(|&x| x as f64).collect(),
        );

        let coefficients: Vec<i32> = coefficients.iter().map(|&x| x as i32).take_while(|&x| x != 0).collect();

        if coefficients.len() == 0 // trim leading zeroes
        {
            Polynomial::new(vec![0])
        } else {
            Polynomial::new(coefficients)
        }

    }
}
