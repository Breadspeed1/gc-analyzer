use core::f64;
use std::{
    f64::consts,
    ops::{Sub, SubAssign},
};

use itertools::Itertools;
use nalgebra::DVector;
use statrs::statistics::Statistics;

const PEAK_SIGMA_THRESHOLD_MULT: f64 = -2.;
const GROUPING_CONSTANT: f64 = 1.5;
const MIN_GROUP_RADIUS: f64 = 20.0;

pub trait PeakDetector {
    fn detect_peaks(&self, signal: &DVector<f64>) -> Vec<Peak>;
}

#[derive(Debug, Clone)]
pub struct Peak {
    pub width: f64,
    pub height: f64,
    pub prominence: f64,
    pub pos: f64,
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

    let mu = kernel.mean();

    kernel.iter_mut().for_each(|v| v.sub_assign(mu));

    kernel
}

fn find_minima(data: &DVector<f64>, threshold: f64) -> Vec<(usize, f64)> {
    data.iter()
        .enumerate()
        .tuple_windows()
        .filter_map(|((_, &l), (mi, &m), (_, &r))| {
            if m < l && m < r && m <= threshold {
                Some((mi, m))
            } else {
                None
            }
        })
        .collect()
}

fn combine_peaks(peaks: &[Peak]) -> Peak {
    let mut weighted_width = 0.0;
    let mut weighted_height = 0.0;
    let mut weighted_pos = 0.0;
    let mut total_prominence = 0.0;

    let mut mp = 0.;
    let mut best_pos = 0.;

    for p in peaks {
        weighted_width += p.width * p.prominence;
        weighted_height += p.height * p.prominence;
        weighted_pos += (p.pos + 0.5 * p.width) * p.prominence;
        total_prominence += p.prominence;

        if p.prominence > mp {
            mp = p.prominence;
            best_pos = p.pos
        }
    }

    Peak {
        width: weighted_width / total_prominence,
        height: weighted_height / total_prominence,
        pos: best_pos,
        prominence: total_prominence,
    }
}

impl DDOGPeakDetector {
    pub fn new(scales: Vec<f64>) -> Self {
        Self { scales }
    }
}

impl PeakDetector for DDOGPeakDetector {
    fn detect_peaks(&self, signal: &DVector<f64>) -> Vec<Peak> {
        let threshold = signal.view_range(0..250, 0).std_dev() * PEAK_SIGMA_THRESHOLD_MULT;

        let conv_minima = self
            .scales
            .iter()
            .map(|&scale| signal.convolve_same(generate_2dog_kernel(scale)))
            .map(|conv| {
                find_minima(&conv, threshold)
                    .into_iter()
                    .collect::<Vec<(usize, f64)>>()
            })
            .collect::<Vec<Vec<(usize, f64)>>>();

        let mut peaks: Vec<Peak> = conv_minima
            .into_iter()
            .enumerate()
            .flat_map(|(size_index, vec)| {
                vec.into_iter().map(move |(pos, prominence)| Peak {
                    width: self.scales[size_index],
                    height: signal[pos],
                    prominence: -prominence,
                    pos: pos as f64,
                })
            })
            .collect();

        let mut new_peaks: Vec<Peak> = vec![];

        for peak in &peaks {
            println!("{:?}", peak);
        }

        while peaks.len() > 0 {
            let mut queue = vec![peaks.remove(0)];
            let mut group = vec![];

            while let Some(ref_peak) = queue.pop() {
                let mut i = 0;

                while i < peaks.len() {
                    let p = &peaks[i];

                    let dist = p.pos.sub(&ref_peak.pos).abs();

                    if dist
                        < (GROUPING_CONSTANT * 0.5 * (ref_peak.width + p.width))
                            .max(MIN_GROUP_RADIUS)
                    {
                        queue.push(peaks.remove(i));
                    } else {
                        i += 1;
                    }
                }

                group.push(ref_peak);
            }

            new_peaks.push(combine_peaks(&group));
        }

        new_peaks
    }
}

#[cfg(test)]
mod tests {
    use plotters::style::{GREEN, RED};

    use crate::peak_detection::{DDOGPeakDetector, PeakDetector, generate_2dog_kernel};

    #[test]
    fn print_2dog_kernel() {
        let kernel = generate_2dog_kernel(80.);
        println!("{:.5}", kernel);

        assert_eq!(kernel.argmin().0, kernel.len() / 2);

        crate::simple_graph_vec("test-img/2dog_kernel_6sigma_test.png", &kernel);
    }

    #[test]
    fn test_peak_detection() {
        let data = crate::io::read_series("../../gc-data/R16443 - Jun 08 2025, 09;24.fusion-data")
            .unwrap();

        let convolution = data.convolve_same(generate_2dog_kernel(40.)) * 100.;

        let peaks = DDOGPeakDetector::new(vec![5., 10., 20., 40., 80.]).detect_peaks(&data);

        crate::simple_graph_vecs_with_peaks(
            "test-img/2dog_peaks_test.png",
            &[(&data, &RED), (&convolution, &GREEN)],
            &peaks,
        );

        panic!("give me stdout bitch");
    }
}
