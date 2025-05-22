pub mod vertex;
pub mod state;
pub mod windowing;

pub fn run() {
    super::log_init("info");
    windowing::run_app();
}
