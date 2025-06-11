use nalgebra::DVector;

trait Smoother {
    fn smooth(&self, signal: &mut DVector<f64>);
}

struct MovingAverage {
    k: usize,
}

impl MovingAverage {
    fn new(k: usize) -> Option<Self> {
        if k > 0 { Some(Self { k }) } else { None }
    }
}

impl Smoother for MovingAverage {
    fn smooth(&self, signal: &mut DVector<f64>) {
        for i in 0..signal.len() {
            let l = i.saturating_sub(self.k / 2);
            let r = (i + self.k / 2).min(signal.len());

            signal[i] = signal.view_range(l..=r, 0).mean();
        }
    }
}

#[cfg(test)]
mod test {
    use nearly::assert_nearly;

    use super::*;

    fn nearly_eq(a: &DVector<f64>, b: &DVector<f64>) {
        a.iter()
            .zip(b.iter())
            .for_each(|(&a, &b)| assert_nearly!(a == b));
    }

    fn get_test_vec() -> DVector<f64> {
        DVector::from_column_slice(&[0., 1., 2.])
    }

    #[test]
    fn k_1() {
        let expected = get_test_vec();
        let mut res = expected.clone();
        MovingAverage::new(1).unwrap().smooth(&mut res);

        nearly_eq(&expected, &res);
    }

    #[test]
    fn k_0() {
        assert!(MovingAverage::new(0).is_none());
    }

    #[test]
    fn k_2() {
        let expected = DVector::from_column_slice(&[0.5, 1., 1.5]);
        let mut res = expected.clone();
        MovingAverage::new(1).unwrap().smooth(&mut res);

        nearly_eq(&expected, &res);
    }

    #[test]
    fn k_3() {
        let expected = DVector::from_column_slice(&[1., 1., 1.]);
        let mut res = expected.clone();
        MovingAverage::new(1).unwrap().smooth(&mut res);

        nearly_eq(&expected, &res);
    }
}
