use crate::refrigerant::RefrigerantMixture;

/// finds the farthest common component of the two mixtures
///
/// assumes mixtures have the same components
fn find_weakest_component(observed: &RefrigerantMixture, target: &RefrigerantMixture) -> f64 {
    target
        .components()
        .map(|(name, concentration)| {
            let observed_concentration = observed.get_component(name).unwrap();
            (observed_concentration / concentration).min(1.0)
        })
        .reduce(|a, b| a.min(b))
        .unwrap()
}

fn valid_comparison(observed: &RefrigerantMixture, target: &RefrigerantMixture) -> bool {
    observed
        .component_set()
        .is_superset(&target.component_set())
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
