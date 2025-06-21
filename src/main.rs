use clap::Parser;
use serde::Deserialize;

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

    println!("{:?}", args);

    /*
    Load config (signal & categorical)
    Create SignalAnalyzer with signal config
    analyzer.load_file(raw file)
    raw_data_report = raw_analyzer.analyze()
    Create CategoryAnalyzer with raw data report and category config
    categorical_report = category_analyzer.analyze()
    Create Reporter with report config
    reporter.report(raw_data_report, categorical_report)
    */
}
