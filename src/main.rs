use std::{error::Error, ffi::OsString, path::Path, time::Instant};

use nalgebra::DVector;
use plotters::prelude::*;

mod math;
mod refrigerant;

fn main() {
    std::fs::read_dir("gc-data").unwrap().for_each(|f| {
        let f = f.unwrap();
        graph_data(read_data(f.path()), f.file_name()).unwrap();
    });
}

fn read_data(path: impl AsRef<Path>) -> DVector<f64> {
    signal_pipeline::io::read_series(path).unwrap()
}

fn graph_data(data: DVector<f64>, name: OsString) -> Result<(), Box<dyn Error>> {
    let time = Instant::now();
    let name = format!(
        "gc-data-img/{}.png",
        name.to_string_lossy()
            .to_string()
            .trim_end_matches(".fusion-data")
    );

    let root = BitMapBackend::new(&name, (480, 320)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("diggity", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..data.len(), -50.0..data.max())?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(data.iter().copied().enumerate(), &RED))?
        .label("y = DATYAAAA")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    root.present()?;

    println!("{}: {:?}", name, Instant::now() - time);

    Ok(())
}

// fn main() {

//     let mut config: Config = serde_json::de::from_reader(
//         File::open("config.json").expect("Could not find config file."),
//     )
//     .expect("Could not read config file.");

//     config.init_pure_mixtures();

//     config.sanity_check().expect("Config sanity check failed.");

//     println!("Please enter reading: ");

//     let mut input = String::new();

//     io::stdin()
//         .read_line(&mut input)
//         .expect("Unable to read user input");

//     let reading: GCReading = input.try_into().expect("Unable to parse user input");

//     let mut results: Vec<ClassificationResult> = config
//         .mixtures
//         .iter()
//         .filter_map(|mix| mix.classify(&reading))
//         .collect();

//     let mut results2: Vec<ClassificationResult> = config
//         .mixtures
//         .iter()
//         .filter_map(|mix| mix.classify_optimize(&reading))
//         .collect();

//     results.sort_by(|r1, r2| r1.purity.partial_cmp(&r2.purity).unwrap().reverse());
//     results2.sort_by(|r1, r2| r1.purity.partial_cmp(&r2.purity).unwrap().reverse());

//     println!("\nClassifications:");

//     results.iter().for_each(|res| println!("{}", res));

//     println!("\nOptimization Classifications:");

//     results2.iter().for_each(|res| println!("{}", res));

//     let (mut results3, total_usage) = MixtureOptimization::new(
//         &reading,
//         config
//             .mixtures
//             .iter()
//             .filter(|m| math::valid_comparison(&reading, m))
//             .map(|m| (m, 0.))
//             .collect(),
//     )
//     .optimize_usage()
//     .expect("idk");

//     println!("\nTotal Optimization ({:.3}% usage):", total_usage * 100.0);

//     results3.sort_by(|r1, r2| r1.0.partial_cmp(&r2.0).unwrap().reverse());

//     results3.iter().for_each(|(percent, mix)| {
//         println!(
//             "Name: {}, Percent: {:.3}%",
//             mix.identifier(),
//             percent * 100.0
//         )
//     });
// }

// #[derive(Deserialize, Debug)]
// struct Config<'a> {
//     pure_refrigerants: HashSet<RefrigerantName>,
//     mixtures: Vec<RefrigerantMixture<'a>>,
// }

// impl<'a> Config<'a> {
//     fn init_pure_mixtures(&mut self) {
//         for r in self.pure_refrigerants.iter() {
//             if !self.mixtures.iter().any(|m| m.identifier() == r) {
//                 self.mixtures.push(RefrigerantMixture::new(
//                     r.clone(),
//                     HashMap::from([(r.clone(), 1.0)]),
//                     ClassificationList::default(),
//                 ));
//             }
//         }
//     }

//     fn sanity_check(&self) -> Result<(), String> {
//         match self.mixtures.iter().find(|mix| {
//             mix.components()
//                 .map(|r| *r.1)
//                 .sum::<f64>()
//                 .sub(1.)
//                 .abs()
//                 .gt(&f64::EPSILON)
//         }) {
//             Some(mix) => {
//                 return Err(format!(
//                     "Mixture [{}]'s components do not add up to 1.0. (Make sure you are listing them with proportions instead of percentages!)",
//                     mix.identifier()
//                 ));
//             }
//             None => (),
//         }

//         Ok(())
//     }
// }
