mod polynomial;
mod fft;
mod polynomial_test;
mod key_generation;
mod polynomial_reborn;

fn main() {
    println!("Hello, world!");
    let test = key_generation::Key::keygen(10, 23, polynomial_reborn::PolynomialReborn::new(vec![]));

    println!("{:?}", test.private_key.coefficients);
    println!("{:?}", test.public_key.0.coefficients);
}
