mod core;
mod tui;
use tui::run_tui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_tui()
}
