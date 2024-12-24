use crate::polynomial_reborn::PolynomialReborn;
use rand::{thread_rng, Rng};
use rand_distr::{Bernoulli, Normal, Uniform};
pub struct Key {
    pub public_key: (PolynomialReborn, PolynomialReborn),
    pub private_key: PolynomialReborn,
}

impl Key {
    fn get_binary_polynomial(size: usize) -> PolynomialReborn {
        let bernoulli = Bernoulli::new(0.5).unwrap();
        let coefficients: Vec<i64> = thread_rng()
            .sample_iter(&bernoulli)
            .take(size)
            .map(|x| x as i64)
            .collect();
        PolynomialReborn::new(coefficients).coefficient_modulus(2)
    }

    fn get_uniform_polynomial(size: usize, modulus: i64) -> PolynomialReborn {
        let between = Uniform::from(0..modulus);
        let coefficients: Vec<i64> = thread_rng().sample_iter(&between).take(size).collect();
        PolynomialReborn::new(coefficients).coefficient_modulus(modulus)
    }

    fn get_normal_polynomial(size: usize) -> PolynomialReborn {
        let normal = Normal::new(0.0, 2.0).unwrap();
        let coefficients = thread_rng()
            .sample_iter(&normal)
            .take(size)
            .map(|x| x as i64)
            .collect();
        PolynomialReborn::new(coefficients).coefficient_modulus(2)
    }

    pub fn keygen(size: usize, modulus: i64, polynomial_mod: PolynomialReborn) -> Key {
        let secret_key = Self::get_binary_polynomial(size);

        let a = Self::get_uniform_polynomial(size, modulus);
        let e = Self::get_normal_polynomial(size);


        let minus_a = a.clone().scale(-1);
        let product = minus_a.multiply(secret_key.clone()).coefficient_modulus(modulus).polynomial_modulus(polynomial_mod.clone());

        let b = product.add(e.scale(-1)).coefficient_modulus(modulus).polynomial_modulus(polynomial_mod);


        Key {
            public_key: (b,a),
            private_key: secret_key,
        }
    }
}
