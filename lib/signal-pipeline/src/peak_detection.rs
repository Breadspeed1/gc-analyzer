use core::f64;
use std::f64::consts;

use nalgebra::DVector;

pub trait PeakDetector {
    fn detect_peaks(signal: &DVector<f64>) -> Vec<Peak>;
}

#[derive(Debug)]
pub struct Peak {
    pub width: f64,
    pub height: f64,
    pub prominence: f64,
}

/// Double Derivative of Gaussian peak detector (mexican hat)
pub struct DDOGPeakDetector {
    scales: Vec<f64>,
}

fn gauss_2nd_derivative(scale: f64, x: f64) -> f64 {
    let d = (x.powi(2) / scale.powi(4)) - scale.powi(-2);
    let c = (2. * consts::PI).sqrt().powi(-1);
    let g = (-x.powi(2) / (2. * scale.powi(2))).exp();

    d * c * g
}

fn generate_2dog_kernel(scale: f64) -> DVector<f64> {
    let n = 6 * scale as usize + 1;
    let mut kernel: DVector<f64> = DVector::zeros(n);
    let xcord = |i: usize| -> f64 {
        let i = i as f64;
        let coord = i - (n / 2) as f64;

        coord
    };

    (0..n).for_each(|v| kernel[v] = gauss_2nd_derivative(scale, xcord(v)));

    kernel
}

impl DDOGPeakDetector {
    pub fn new(scales: Vec<f64>) -> Self {
        Self { scales }
    }
}

impl PeakDetector for DDOGPeakDetector {
    fn detect_peaks(signal: &DVector<f64>) -> Vec<Peak> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::peak_detection::generate_2dog_kernel;

    #[test]
    fn print_2dog_kernel() {
        let kernel = generate_2dog_kernel(6.);
        println!("{:.5}", kernel);

        assert_eq!(kernel.argmin().0, kernel.len() / 2);
    }
}
