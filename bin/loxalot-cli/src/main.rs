use clap::Parser;
use log::{debug, LevelFilter};

#[derive(Parser)]
#[clap(name = "loxalot", version)]
struct Cli {
    /// The input path of a script to run.
    #[clap(parse(from_os_str))]
    script_path: Option<std::path::PathBuf>,

    /// Verbosity level (can be specified multiple times)
    #[clap(long, short, global = true, parse(from_occurrences))]
    verbose: usize,
}

fn setup_logger(verbosity: usize) {
    let mut env_logger_builder = env_logger::Builder::new();
    let log_filter_level = match verbosity {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };
    env_logger_builder.filter_level(log_filter_level);
    env_logger_builder.format_timestamp(None);
    env_logger_builder.init();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    setup_logger(cli.verbose);
    debug!("starting");



    Ok(())
}

fn run_file(file: std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

fn run_prompt() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

fn run(content: String)  -> Result<(), Box<dyn std::error::Error>> {
    
}
