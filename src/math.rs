use std::collections::HashSet;

use rand::random;

use crate::RefrigerantMixture;

pub fn vectorize(
    observed: &RefrigerantMixture,
    expected: &RefrigerantMixture,
) -> (Vec<f64>, Vec<f64>) {
    let expected_keys = &expected.components.keys().collect();
    let observed_keys = observed.components.keys().collect::<HashSet<&String>>();
    let all = observed_keys
        .union(expected_keys)
        .collect::<HashSet<&&String>>();

    let mut o: Vec<f64> = vec![0f64; all.len()];
    let mut e: Vec<f64> = vec![0f64; all.len()];

    for (i, &name) in all.iter().enumerate() {
        o[i] = *observed.components.get(*name).unwrap_or(&0f64);
        e[i] = *expected.components.get(*name).unwrap_or(&0f64);
    }

    (o, e)
}

pub fn shuffle(a: &mut Vec<f64>, b: &mut Vec<f64>) {
    a.iter_mut().enumerate().for_each(|(i, v)| {
        if random() {
            let t = *v;
            *v = b[i];
            b[i] = t;
        }
    });
}
