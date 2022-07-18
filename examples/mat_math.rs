use ndarray::arr2;
use num::complex::Complex;
use std::f64::consts::PI;

fn main() {
    let y: f64 = 10.0/10.0;
    let a = arr2(&[
        [Complex::new(y, y), Complex::new(y*2.0, y*2.0)],
        [Complex::new(y*3.0, y*3.0), Complex::new(y*4.0, y*4.0)],
    ]);

    let b = arr2(&[
        [Complex::new(1.0, 1.0), Complex::new(1.0, 1.0)],
        [Complex::new(1.0, 1.0), Complex::new(1.0, 1.0)],
    ]);

    println!("{}", a.dot(&b));

    let sum = &a + &b; // Borrow a & b for sum
    println!("{}", sum);

    let x = Complex::new(0.0, 2.0 * PI);
    println!("e^(2i * pi) = {}", x.exp()); // =~1
}
