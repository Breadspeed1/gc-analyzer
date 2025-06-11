use nalgebra::{DVector, Vector};

trait Smoother {
    fn smooth(&self, signal: &mut DVector<f64>);
}
