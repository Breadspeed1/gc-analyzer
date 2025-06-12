use std::collections::BTreeSet;

extern crate good_lp;

use good_lp::{
    Constraint, Expression, IntoAffineExpression, ProblemVariables, Solution, SolverModel,
    Variable, variable, variables,
};

use refrigerants::{GCReading, RefrigerantMixture, RefrigerantName};

pub type OptimizationResult<'a> = Result<(Vec<(f64, &'a RefrigerantMixture<'a>)>, f64), String>;

pub struct MixtureOptimization<'a> {
    problem_variables: ProblemVariables,
    ref_vars: Vec<(Variable, &'a RefrigerantMixture<'a>)>,
    component_expressions: Vec<Expression>,
    constraints: Vec<Constraint>,
}

impl<'a> MixtureOptimization<'a> {
    pub fn new(reading: &GCReading, mixtures: Vec<(&'a RefrigerantMixture, f64)>) -> Self {
        let component_set = generate_component_set(&mixtures);
        let mut problem_variables = variables! {};

        let vars: Vec<(Variable, Vec<f64>)> = mixtures
            .iter()
            .map(|(mix, min)| {
                (
                    problem_variables.add(variable().min(*min).max(1.)),
                    vectorize(mix, component_set.as_slice()),
                )
            })
            .collect();

        let ref_vars = vars
            .iter()
            .enumerate()
            .map(|(i, (var, _))| (var.clone(), mixtures[i].0))
            .collect::<Vec<_>>();

        let component_expressions = make_constraint_expressions(vars);

        let constraints = component_expressions
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, ex)| ex.leq(reading.get_component(component_set[i]).map_or(0., |&c| c)))
            .collect::<Vec<_>>();

        Self {
            problem_variables,
            ref_vars,
            component_expressions,
            constraints,
        }
    }

    pub fn optimize_usage(self) -> OptimizationResult<'a> {
        let obj = self.component_expressions.iter().sum::<Expression>();

        self.optimize(&obj)
    }

    fn optimize(self, objective_function: &Expression) -> OptimizationResult<'a> {
        let sol = self
            .problem_variables
            .maximise(objective_function)
            .using(good_lp::solvers::clarabel::clarabel)
            .with_all(self.constraints)
            .solve()
            .map_err(|e| e.to_string())?;

        let concentrations = self
            .ref_vars
            .iter()
            .map(|(var, mix)| (sol.value(*var), *mix))
            .collect::<Vec<_>>();

        Ok((concentrations, sol.eval(objective_function)))
    }

    pub fn optimize_max_refrigerant(self, name: &RefrigerantName) -> OptimizationResult<'a> {
        match self
            .ref_vars
            .iter()
            .find(|&(_, mix)| mix.identifier() == name)
        {
            Some((var, _)) => {
                let obj = self.component_expressions.iter().sum::<Expression>() + var;

                self.optimize(&obj)
            }
            None => {
                Err("Requested max refrigerant is not a part of the optimization problem.".into())
            }
        }
    }
}

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

fn make_constraint_expressions(vars: Vec<(Variable, Vec<f64>)>) -> Vec<Expression> {
    vars.into_iter()
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
        .expect("Failed to collect constraints.")
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

fn generate_component_set<'a>(
    targets: &[(&'a RefrigerantMixture, f64)],
) -> Vec<&'a RefrigerantName> {
    targets
        .iter()
        .flat_map(|(mix, _)| mix.component_set())
        .collect::<BTreeSet<&RefrigerantName>>()
        .into_iter()
        .collect::<Vec<&RefrigerantName>>()
}
