use std::f64::consts::PI;
use num::complex::Complex;

fn main() {
    let complex_integer = Complex::new(10, 20);
    let complex_float = Complex::new(10.1, 20.1);

    println!("Complex integer: {}", complex_integer);
    println!("Complex float: {}", complex_float);

    let complex_num1 = Complex::new(10.0, 20.0); // Must use floats
    let complex_num2 = Complex::new(3.1, -4.2);

    let sum = complex_num1 + complex_num2;

    println!("Sum: {}", sum);

    let x = Complex::new(0.0, 2.0*PI);

    println!("e^(2i * pi) = {}", x.exp()); // =~1
}