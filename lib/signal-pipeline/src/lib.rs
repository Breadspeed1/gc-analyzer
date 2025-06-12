use nalgebra::DVector;
use nearly::assert_nearly;

pub mod io;
pub mod preprocess;

pub fn nearly_eq(a: &DVector<f64>, b: &DVector<f64>) {
    a.iter()
        .zip(b.iter())
        .for_each(|(&a, &b)| assert_nearly!(a == b));
}
