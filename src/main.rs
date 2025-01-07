mod core;
mod tui;
use tui::run_tui;
use core::get_config;



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_config();
    run_tui(config)
}
