use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io,
};

use math::MixtureOptimization;
use refrigerant::{
    ClassificationList, ClassificationResult, GCReading, RefrigerantMixture, RefrigerantName,
};
use serde::Deserialize;

mod math;
mod refrigerant;

fn main() {
    let mut config: Config = serde_json::de::from_reader(
        File::open("config.json").expect("Could not find config file."),
    )
    .expect("Could not read config file.");

    config.init_pure_mixtures();

    println!("Please enter reading: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read user input");

    let reading: GCReading = input.try_into().expect("Unable to parse user input");

    let mut results: Vec<ClassificationResult> = config
        .mixtures
        .iter()
        .filter_map(|mix| mix.classify(&reading))
        .collect();

    let mut results2: Vec<ClassificationResult> = config
        .mixtures
        .iter()
        .filter_map(|mix| mix.classify_optimize(&reading))
        .collect();

    results.sort_by(|r1, r2| r1.purity.partial_cmp(&r2.purity).unwrap().reverse());
    results2.sort_by(|r1, r2| r1.purity.partial_cmp(&r2.purity).unwrap().reverse());

    println!("\nClassifications:");

    results.iter().for_each(|res| println!("{}", res));

    println!("\nOptimization Classifications:");

    results2.iter().for_each(|res| println!("{}", res));

    let (mut results3, total_usage) = MixtureOptimization::new(
        &reading,
        config
            .mixtures
            .iter()
            .filter(|m| math::valid_comparison(&reading, m))
            .map(|m| (m, 0.))
            .collect(),
    )
    .optimize_usage()
    .expect("idk");

    println!("\nTotal Optimization ({:.3}% usage):", total_usage * 100.0);

    results3.sort_by(|r1, r2| r1.0.partial_cmp(&r2.0).unwrap().reverse());

    results3.iter().for_each(|(percent, mix)| {
        println!(
            "Name: {}, Percent: {:.3}%",
            mix.identifier(),
            percent * 100.0
        )
    });
}

#[derive(Deserialize, Debug)]
struct Config {
    pure_refrigerants: HashSet<RefrigerantName>,
    mixtures: Vec<RefrigerantMixture>,
}

impl Config {
    fn init_pure_mixtures(&mut self) {
        for r in self.pure_refrigerants.iter() {
            if !self.mixtures.iter().any(|m| m.identifier() == r) {
                self.mixtures.push(RefrigerantMixture::new(
                    r.clone(),
                    HashMap::from([(r.clone(), 1.0)]),
                    ClassificationList::default(),
                ));
            }
        }
    }
}
