extern crate num;

use std::f64::consts::PI;
use num::complex::Complex;

pub struct Fft;

impl Fft {
    fn fft(p: Vec<f64>) -> Vec<Complex<f64>> {
        let n = p.len();
        if n == 1 {
            return p.iter().map(|&x| Complex { re: x, im: 0.0 }).collect();
        }
        let deg = 2.0 * PI / (n as f64);
        let w_n = Complex::new(deg.cos(), deg.sin());
        let even = p.iter().step_by(2).map(|&x| x).collect();
        let odd = p.iter().skip(1).step_by(2).map(|&x| x).collect();
        let y_e = Self::fft(even);
        let y_o = Self::fft(odd);
        let mut y = vec![Complex::new(0.0, 0.0); n];
        let mut w = Complex::new(1.0, 0.0);
        let half = n / 2;
        for j in 0..half {
            let odd_term = w * y_o[j];
            y[j] = y_e[j] + odd_term;
            y[j + half] = y_e[j] - odd_term;
            w *= w_n;
        }
        y
    }

    fn inverse_fft(p: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
        let n = p.len();
        if n == 1 {
            return p;
        }

        let deg = -2.0 * PI / (n as f64);
        let w_n = Complex::new(deg.cos(), deg.sin());
        let even = p.iter().step_by(2).map(|&x| x).collect();
        let odd = p.iter().skip(1).step_by(2).map(|&x| x).collect();
        let y_e = Self::inverse_fft(even);
        let y_o = Self::inverse_fft(odd);
        let mut y = vec![Complex::new(0.0, 0.0); n];
        let mut w = Complex::new(1.0, 0.0);

        let half = n / 2;
        for j in 0..half {
            let odd_term = w * y_o[j];
            y[j] = y_e[j] + odd_term;
            y[j + half] = y_e[j] - odd_term;
            w *= w_n;
        }
        y
    }
    pub fn multiply(a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
        let n = a.len() + b.len() - 1;
        let size = n.next_power_of_two();

        let mut a = a;
        let mut b = b;

        a.resize(size, 0.0);
        b.resize(size, 0.0);

        let product_pointwise: Vec<Complex<f64>> = Self::fft(a).iter().zip(Self::fft(b).iter()).map(|(x, y)| x * y).collect();

        let new_coefficients: Vec<Complex<f64>> = Self::inverse_fft(product_pointwise.iter().map(|x| x / size as f64).collect());

        new_coefficients.iter().map(|x| x.re).collect()
    }
}
