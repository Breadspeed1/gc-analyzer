use std::collections::BTreeSet;

use good_lp::{
    Expression, IntoAffineExpression, Solution, SolverModel, Variable, variable, variables,
};

use crate::refrigerant::{GCReading, RefrigerantMixture, RefrigerantName};

/// finds the farthest common component of the two mixtures
///
/// assumes mixtures have the same components
fn find_weakest_component(observed: &GCReading, target: &RefrigerantMixture) -> f64 {
    target
        .components()
        .map(|(name, concentration)| {
            let observed_concentration = observed.get_component(name).unwrap();
            (observed_concentration / concentration).min(1.0)
        })
        .reduce(|a, b| a.min(b))
        .unwrap()
}

pub fn valid_comparison(observed: &GCReading, target: &RefrigerantMixture) -> bool {
    observed
        .component_set()
        .is_superset(&target.component_set())
}

pub fn find_concentration(observed: &GCReading, target: &RefrigerantMixture) -> Option<f64> {
    if !valid_comparison(observed, target) {
        return None;
    }

    Some(find_weakest_component(observed, target))
}

fn is_low(&(name, &concentration): &(&RefrigerantName, &f64), target: &RefrigerantMixture) -> bool {
    concentration <= 0.05 && !target.component_set().contains(name)
}

pub fn find_max_low(observed: &GCReading, target: &RefrigerantMixture) -> f64 {
    observed
        .components()
        .filter(|v| is_low(v, target))
        .map(|(_, &v)| v)
        .reduce(|v1, v2| if v1 < v2 { v1 } else { v2 })
        .unwrap_or(0.0)
}

fn vectorize(mix: &RefrigerantMixture, keys: &[&RefrigerantName]) -> Vec<f64> {
    keys.iter()
        .map(|&key| mix.get_component(key).map_or(0., |v| *v))
        .collect()
}

pub fn optimize<'a>(
    observed: &GCReading,
    targets: Vec<(&'a RefrigerantMixture, f64)>,
) -> (Vec<(f64, &'a RefrigerantMixture)>, f64) {
    let mut pv = variables! {};

    let component_set = targets
        .iter()
        .flat_map(|(mix, _)| mix.component_set())
        .collect::<BTreeSet<&RefrigerantName>>()
        .into_iter()
        .collect::<Vec<&RefrigerantName>>();

    let vars: Vec<(Variable, Vec<f64>)> = targets
        .iter()
        .map(|(mix, min)| {
            (
                pv.add(variable().min(*min).max(1.)),
                vectorize(mix, component_set.as_slice()),
            )
        })
        .collect();

    let ref_vars = vars
        .iter()
        .enumerate()
        .map(|(i, (var, _))| (var.clone(), targets[i].0))
        .collect::<Vec<_>>();

    let expr = vars
        .into_iter()
        .map(|(var, vec)| {
            vec.into_iter()
                .map(move |val| val * var.into_expression())
                .collect::<Vec<_>>()
        })
        .reduce(|e1, e2| {
            e1.into_iter()
                .zip(e2.into_iter())
                .map(|(e1, e2)| e1 + e2)
                .collect()
        })
        .expect("Failed to collect constraints.");

    let constraints = expr
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, ex)| ex.leq(observed.get_component(component_set[i]).map_or(0., |&c| c)))
        .collect::<Vec<_>>();

    let obj = expr.into_iter().sum::<Expression>();

    let sol = pv
        .maximise(&obj)
        .using(good_lp::default_solver)
        .with_all(constraints)
        .solve()
        .expect("Failed to solve lineq");

    let concentrations = ref_vars
        .into_iter()
        .map(|(var, mix)| (sol.value(var), mix))
        .collect::<Vec<_>>();

    (concentrations, sol.eval(obj))
}
