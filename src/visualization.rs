use tui::backend::CrosstermBackend;
use tui::Terminal;
use std::io;

pub fn start_terminal_ui() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    Ok(())
}