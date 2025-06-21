pub mod processing;
pub mod report;

fn main() {
    println!("erm wtf");
    /*
    Parse cmd args (config location, raw file location, report output location)
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
