use crate::key_generation::Key;
use crate::polynomial_reborn::PolynomialReborn;

pub struct HomomorphicEncryption;

impl HomomorphicEncryption {
    pub fn encrypt(
        public_key: (PolynomialReborn, PolynomialReborn),
        size: usize,
        ciphertext_modulus: i64,
        plaintext_modulus: i64,
        polynomial_modulus: PolynomialReborn,
        int_to_encrypt: i64,
    ) -> (PolynomialReborn, PolynomialReborn) {
        let mut m = vec![int_to_encrypt.rem_euclid(plaintext_modulus)];
        m.resize(size, 0);

        let delta = ciphertext_modulus / plaintext_modulus;

        let scaled_m = PolynomialReborn::new(
            m.iter()
                .map(|m| (m * delta).rem_euclid(ciphertext_modulus))
                .collect(),
        );

        let e1 = Key::get_normal_polynomial(size);
        let e2 = Key::get_normal_polynomial(size);
        let u = Key::get_binary_polynomial(size);

        let temp1 = public_key
            .0
            .multiply(u.clone())
            .coefficient_modulus(ciphertext_modulus)
            .polynomial_modulus(polynomial_modulus.clone());
        let temp2 = temp1
            .add(e1)
            .coefficient_modulus(ciphertext_modulus)
            .polynomial_modulus(polynomial_modulus.clone());
        let ct0 = temp2
            .add(scaled_m)
            .coefficient_modulus(ciphertext_modulus)
            .polynomial_modulus(polynomial_modulus.clone());

        let temp3 = public_key
            .1
            .multiply(u)
            .coefficient_modulus(ciphertext_modulus)
            .polynomial_modulus(polynomial_modulus.clone());
        let ct1 = temp3
            .add(e2)
            .coefficient_modulus(ciphertext_modulus)
            .polynomial_modulus(polynomial_modulus.clone());

        (ct0, ct1)
    }

    pub fn decrypt(secret_key: PolynomialReborn, size: usize, ciphertext_modulus: i64,
                   plaintext_modulus: i64, polynomial_modulus: PolynomialReborn,
                   ciphertext: (PolynomialReborn, PolynomialReborn)) -> i64 {
        let temp1 = ciphertext.1.multiply(secret_key).coefficient_modulus(ciphertext_modulus).polynomial_modulus(polynomial_modulus.clone());
        let scaled_pt = temp1.add(ciphertext.0).coefficient_modulus(ciphertext_modulus).polynomial_modulus(polynomial_modulus.clone());

        let decrypted_polynomial = PolynomialReborn::new(scaled_pt.coefficients.iter()
                                                             .map(|x| ((x*plaintext_modulus)/ciphertext_modulus).rem_euclid(plaintext_modulus))
                                                                 .collect());

        decrypted_polynomial.coefficients[0]
    }
}
