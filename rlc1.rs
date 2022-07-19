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
    fn res_z(&self) -> f64 {
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
    fn ind_z(&self) -> Complex<f64> {
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
    fn cap_z(&self) -> Complex<f64> {
        let z = Complex::new(0.0, -1.0 * (2.0 * PI * self.frequency * self.value));
        z
    }
}

// Run the analysis
fn main() {
    let r1 = Resistor { value: 75.0 };
    let z_r1 = r1.res_z();
    let abcd_r1 = arr2(&[
        [Complex::new(1.0, 0.0), Complex::new(z_r1, 0.0)],
        [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
    ]);

    let l1 = Inductor {
        value: 5e-9,
        frequency: 2e9,
    };
    let z_l1 = l1.ind_z();
    let abcd_l1 = arr2(&[
        [Complex::new(1.0, 0.0), z_l1],
        [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
    ]);

    let c1 = Capacitor {
        value: 1e-12,
        frequency: 2e9,
    };
    let z_c1 = c1.cap_z();
    let abcd_c1 = arr2(&[
        [Complex::new(1.0, 0.0), z_c1],
        [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
    ]);

    // Display netlist
    println!("\nRLC Netlist:");
    println!("R1 = {:#?}", r1);
    println!("L1 = {:#?}", l1);
    println!("C1 = {:#?}\n", c1);

    // Multiply cascaded component ABCD matrices
    let mut abcd_ckt = abcd_r1.dot(&abcd_l1);
    abcd_ckt = abcd_ckt.dot(&abcd_c1);

    // println!("{:#?}", &ABCD);
    // println!("{}", &ABCD);

    // Convert ABCD to S
    // https://www.rfwireless-world.com/Terminology/abcd-matrix-vs-s-matrix.html

    // S-parameter matrix
    let mut s = arr2(&[
        [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
        [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
    ]);

    // Define A, B, C, D & denominator from the ABCD matrix
    let a: Complex<f64> = abcd_ckt[[0, 0]];
    let b: Complex<f64> = abcd_ckt[[0, 1]];
    let c: Complex<f64> = abcd_ckt[[1, 0]];
    let d: Complex<f64> = abcd_ckt[[1, 1]];

    let denom: Complex<f64> = &a + &b / 50.0 + &c * 50.0 + &c;

    // S-parmater equations from ABCD matrix
    s[[0, 0]] = (&a + &b / 50.0 - &c * 50.0 - &d) / &denom;
    s[[0, 1]] = 2.0 * (&a * &d - &b * &c) / &denom;
    s[[1, 0]] = 2.0 / &denom;
    s[[1, 1]] = (-&a + &b / 50.0 - &c * 50.0 + &d) / &denom;

    // Final S-parameter results
    println!("S-parameters:");
    println!("{}", &s);
}
