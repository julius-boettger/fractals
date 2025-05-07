mod curves;
mod renderer;

fn main() {
    // set up logger with default level if env var RUST_LOG is unset
    env_logger::Builder::from_env(
        env_logger::Env::default()
            .default_filter_or("error,fractals=info")
    ).init();

    renderer::render();
}
