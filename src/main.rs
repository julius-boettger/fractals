mod curves;
mod rendering;
mod benchmark;

use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Run CPU benchmark
    #[arg(short, long)]
    bench: bool,
}

fn main() {
    if Args::parse().bench {
        benchmark::run();
    } else {
        rendering::run();
    }
}

/// set up logging with given level for this project if env var `RUST_LOG` is unset
pub fn log_init(level: &str) {
    let project_name = env!("CARGO_PKG_NAME");
    let logging_filter = format!("error,{project_name}={level}");
    env_logger::Builder::from_env(
        env_logger::Env::default()
            .default_filter_or(logging_filter)
    ).init();
}
