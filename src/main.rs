use std::{collections::HashMap, fs::File};

use serde::Deserialize;
use simsimd::ProbabilitySimilarity;

mod math;

fn main() {
    let config = read_config();

    let mix = RefrigerantMixture {
        name: "".into(),
        components: HashMap::from_iter(
            vec![("R-32".into(), 51f64), ("R-125".into(), 49f64)].into_iter(),
        ),
    };

    println!("{:?}", eval(&config, &mix));
}

fn eval<'a>(
    config: &'a Config,
    observation: &RefrigerantMixture,
) -> Vec<(&'a RefrigerantMixture, f64)> {
    config
        .mixtures
        .iter()
        .map(|mix| (mix, sim(observation, mix)))
        .collect()
}

fn sim(observed: &RefrigerantMixture, expected: &RefrigerantMixture) -> f64 {
    let (mut ov, mut ev) = math::vectorize(observed, expected);
    let observed_distance =
        ProbabilitySimilarity::jensenshannon(ov.as_slice(), ev.as_slice()).unwrap();

    observed_distance
}

fn read_config() -> Config {
    let mut config: Config = serde_json::de::from_reader(
        File::open("config.json").expect("Unable to find config file!"),
    )
    .expect("Unable to read config file!");

    config.pure_refrigerants.iter_mut().for_each(normalize_name);
    config.mixtures.iter_mut().for_each(|m| {
        normalize_name(&mut m.name);
        m.components = m
            .components
            .clone()
            .into_iter()
            .map(|(mut k, v)| {
                normalize_name(&mut k);
                (k, v)
            })
            .collect();
    });

    config
}

fn normalize_name(name: &mut String) {
    *name = name.to_uppercase().replace(" ", "");
}

#[derive(Deserialize, Debug)]
struct RefrigerantMixture {
    name: String,
    pub components: HashMap<String, f64>,
}

#[derive(Deserialize, Debug)]
struct Config {
    pure_refrigerants: Vec<String>,
    mixtures: Vec<RefrigerantMixture>,
}
