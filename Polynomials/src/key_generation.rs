use crate::polynomial::Polynomial;
use rand::{thread_rng, Rng};
use rand_distr::{Bernoulli, Normal, Uniform};
pub struct Key {
    pub public_key: (Polynomial, Polynomial),
    pub private_key: Polynomial,
}

impl Key {
    fn get_binary_polynomial(size: usize) -> Polynomial {
        let bernoulli = Bernoulli::new(0.5).unwrap();
        let coefficients: Vec<i32> = thread_rng()
            .sample_iter(&bernoulli)
            .take(size)
            .map(|x| x as i32)
            .collect();
        Polynomial::new(coefficients, 2)
    }

    fn get_uniform_polynomial(size: usize, modulus: i32) -> Polynomial {
        let between = Uniform::from(0..modulus);
        let coefficients: Vec<i32> = thread_rng().sample_iter(&between).take(size).collect();
        Polynomial::new(coefficients, modulus)
    }

    fn get_normal_polynomial(size: usize) -> Polynomial {
        let normal = Normal::new(0.0, 2.0).unwrap();
        let coefficients = thread_rng()
            .sample_iter(&normal)
            .take(size)
            .map(|x| x as i32)
            .collect();
        Polynomial::new(coefficients, 2)
    }

    pub fn keygen(size: usize, modulus: i32, polynomial_mod: i32) -> Key {
        let secret_key = Self::get_binary_polynomial(size);

        let a = Self::get_uniform_polynomial(size, modulus);
        let e = Self::get_normal_polynomial(size);

        let temp = // TODO use -a
            Polynomial::multiply_with_modulus(-a.clone(), secret_key.clone(), modulus); //, polynomial_mod); //TODO implement polynomial modulus
        let b = Polynomial::add_with_modulus(temp, -e, modulus );//, polynomial_mod);

        Key {
            public_key: (b, a),
            private_key: secret_key,
        }
    }
}
