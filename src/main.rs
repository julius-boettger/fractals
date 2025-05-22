mod curves;
mod rendering;
mod benchmark;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if let Some(first_arg) = args.get(1) {
        if first_arg == "bench" {
            benchmark::run();
            std::process::exit(0);
        }
    }

    rendering::run();
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
