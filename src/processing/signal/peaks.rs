use nalgebra::DVector;
use serde::Deserialize;

pub struct PeakInfo {
    pub width: f64,
    pub height: f64,
    pub pos: f64,
}

#[derive(Deserialize)]
pub struct SDPeakFinder {
    lag: usize,
    influence: f64,
    threshold: f64,
}

pub trait PeakFinder {
    fn find_peaks(&self, data: &DVector<f64>) -> Vec<PeakInfo>;
}
