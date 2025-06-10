pub mod backend;
pub mod ui;

use ui::app;

fn main() -> color_eyre::Result<()> {
    ui::app::run_ui()
}
