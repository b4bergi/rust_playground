mod polynomial;
mod fft;
mod polynomial_test;
mod key_generation;

fn main() {
    println!("Hello, world!");
    let test = key_generation::Key::keygen(10, 107, 107);

    println!("{:?}", test.private_key.coefficients);
}
