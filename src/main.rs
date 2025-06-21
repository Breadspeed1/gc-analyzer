use std::{fs::File, process::exit};

use clap::Parser;
use serde::Deserialize;
use throbber::Throbber;

use crate::processing::signal::SignalAnalyzer;

pub mod processing;
pub mod report;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the configuration file
    #[arg(short, long, default_value_t = String::from("config.json"))]
    config: String,

    /// Path to the gc data json
    #[arg(short, long)]
    data: String,

    /// Path to save the report to
    #[arg(short, long, default_value_t = String::from("report.html"))]
    report: String,
}

#[derive(Deserialize)]
struct Config {
    //config stuff
}

fn main() {
    let args = Args::parse();

    let mut th = Throbber::new().message("Parsing config.".into());

    let config: Config = serde_json::de::from_reader(
        File::open(args.config).expect("Unable to open configuration file."),
    )
    .expect("Unable to parse configuration file.");

    th.success("Done parsing config.".into());

    th.change_message("Loading analyzer".into());

    let analyzer = SignalAnalyzer::new(args.data);

    match analyzer {
        Ok(_) => th.success("Done loading analyzer.".into()),
        Err(err) => {
            th.fail(format!("{:?}", err));
            exit(1);
        }
    }

    let analyzer = analyzer.unwrap();

    /*
    Create SignalAnalyzer with signal config
    analyzer.load_file(raw file)
    raw_data_report = raw_analyzer.analyze()
    Create CategoryAnalyzer with raw data report and category config
    categorical_report = category_analyzer.analyze()
    Create Reporter with report config
    reporter.report(raw_data_report, categorical_report)
    */
}
