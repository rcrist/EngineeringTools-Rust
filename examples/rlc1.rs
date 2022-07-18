// rlc1.rs - Calculate the S-parameters for an RLC circuit
// Method: 2-port cascade analysis using ABCD matrices

// Math libraries
use ndarray::arr2;
use num::complex::Complex;
use std::f64::consts::PI;

// Resistor component
#[derive(Debug)]
struct Resistor {
    pub value: f64,
}

impl Resistor {
    fn Z(&self) -> f64 {
        let z = self.value;
        z
    }
}

// Inductor component
#[derive(Debug)]
struct Inductor {
    value: f64,
    frequency: f64,
}

impl Inductor {
    fn Z(&self) -> Complex<f64> {
        let z = Complex::new(0.0, 1.0 / (2.0 * PI * self.frequency * self.value));
        z
    }
}

// Capacitor component
#[derive(Debug)]
struct Capacitor {
    value: f64,
    frequency: f64,
}

impl Capacitor {
    fn Z(&self) -> Complex<f64> {
        let z = Complex::new(0.0, -1.0 * (2.0 * PI * self.frequency * self.value));
        z
    }
}

// Run the analysis
fn main() {
    let r1 = Resistor { value: 75.0 };
    let Zr1 = r1.Z();
    let ABCD1 = arr2(&[
        [Complex::new(1.0, 0.0), Complex::new(Zr1, 0.0)],
        [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
    ]);

    let l1 = Inductor {
        value: 5e-9,
        frequency: 2e9,
    };
    let Zl1 = l1.Z();
    let ABCD2 = arr2(&[
        [Complex::new(1.0, 0.0), Zl1],
        [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
    ]);

    let c1 = Capacitor {
        value: 1e-12,
        frequency: 2e9,
    };
    let Zc1 = c1.Z();
    let ABCD3 = arr2(&[
        [Complex::new(1.0, 0.0), Zc1],
        [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
    ]);

    // Display netlist
    println!("\nRLC Netlist:");
    println!("R1 = {:#?}", r1);
    println!("L1 = {:#?}", l1);
    println!("C1 = {:#?}\n", c1);

    // Multiply cascaded component ABCD matrices
    let mut ABCD = ABCD1.dot(&ABCD2);
    ABCD = ABCD.dot(&ABCD3);

    // println!("{:#?}", &ABCD);
    // println!("{}", &ABCD);

    // Convert ABCD to S
    // https://www.rfwireless-world.com/Terminology/abcd-matrix-vs-s-matrix.html

    // S-parameter matrix
    let mut S = arr2(&[
        [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
        [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
    ]);

    // Define A, B, C, D & denominator from the ABCD matrix
    let A: Complex<f64> = ABCD[[0, 0]];
    let B: Complex<f64> = ABCD[[0, 1]];
    let C: Complex<f64> = ABCD[[1, 0]];
    let D: Complex<f64> = ABCD[[1, 1]];

    let denom: Complex<f64> = &A + &B / 50.0 + &C * 50.0 + &D;

    // S-parmater equations from ABCD matrix
    S[[0, 0]] = (&A + &B / 50.0 - &C * 50.0 - &D) / &denom;
    S[[0, 1]] = 2.0 * (&A * &D - &B * &C) / &denom;
    S[[1, 0]] = 2.0 / &denom;
    S[[1, 1]] = (-&A + &B / 50.0 - &C * 50.0 + &D) / &denom;

    // Final S-parameter results
    println!("S-parameters:");
    println!("{}", &S);
}
