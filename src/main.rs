mod curves;
mod rendering;
mod benchmark;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Run CPU/memory benchmark by computing the triangles necessary
    /// to represent a given fractal iteration (without rendering it)
    Bench {
        /// Type of fractal to use
        #[arg(
            short, long, value_enum,
            default_value_t = curves::Curves::KochSnowflake,
        )]
        r#type: curves::Curves,
        /// Iteration to compute, 1 meaning the initial state.
        /// Be careful when increasing this, you will eventually run out of memory.
        #[arg(
            short, long,
            default_value_t = 10,
            value_parser = clap::value_parser!(i8).range(1..)
        )]
        iteration: i8,
    },
}

fn main() {
    match Args::parse().command {
        Some(Command::Bench { iteration, r#type })
            => benchmark::run(iteration.try_into().unwrap(), r#type),
        None => rendering::run(),
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
