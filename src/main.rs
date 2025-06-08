use std::{collections::HashMap, fs::File};

use refrigerant::{MixtureIdentifier, RefrigerantMixture, RefrigerantName};
use serde::Deserialize;

mod math;
mod refrigerant;

fn main() {
    let config: Config = serde_json::de::from_reader(
        File::open("config.json").expect("Could not find config file."),
    )
    .expect("Could not read config file.");

    let test = RefrigerantMixture::new(
        MixtureIdentifier::ID(1),
        HashMap::from_iter(
            vec![
                (RefrigerantName::new(&String::from("r-125")).unwrap(), 0.3),
                (RefrigerantName::new(&String::from("r-32")).unwrap(), 0.2),
            ]
            .into_iter(),
        ),
        0.0,
    );

    println!("{:?}", math::find_concentration(&test, &config.mixtures[0]));
}

#[derive(Deserialize, Debug)]
struct Config {
    pure_refrigerants: Vec<String>,
    mixtures: Vec<RefrigerantMixture>,
}
