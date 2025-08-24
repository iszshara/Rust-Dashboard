use color_eyre::Result;
use linux_dashboard::ui::app;
use ratatui::{Terminal, backend::CrosstermBackend, crossterm::terminal};
use std::io::stdout;

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let app_result = app::run_ui(terminal);
    terminal::disable_raw_mode()?;
    app_result
}
