use std::collections::HashSet;

use crate::refrigerant::{RefrigerantComponent, RefrigerantMixture};

/// finds the farthest common component of the two mixtures
///
/// assumes mixtures have the same components
fn find_weakest_component(observed: &RefrigerantMixture, target: &RefrigerantMixture) -> f64 {
    target
        .components()
        .map(|c| {
            let observed_concentration = observed.get_component(c.name()).unwrap().concentration();
            (observed_concentration / c.concentration()).min(1.0)
        })
        .reduce(|a, b| a.min(b))
        .unwrap()
}

fn valid_comparison(observed: &RefrigerantMixture, target: &RefrigerantMixture) -> bool {
    target
        .components()
        .collect::<HashSet<&RefrigerantComponent>>()
        .is_subset(
            &observed
                .components()
                .collect::<HashSet<&RefrigerantComponent>>(),
        )
}

pub fn find_concentration(
    observed: &RefrigerantMixture,
    target: &RefrigerantMixture,
) -> Option<f64> {
    if !valid_comparison(observed, target) {
        return None;
    }

    Some(find_weakest_component(observed, target))
}
