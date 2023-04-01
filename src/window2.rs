use tui::{Terminal, Frame, backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, widgets::{Block, Borders, Paragraph, Text}};
use std::io;

fn main() -> Result<(), io::Error> {
    // Set up the terminal backend
    let backend = CrosstermBackend::new()?;
    let mut terminal = Terminal::new(backend)?;

    // Clear the terminal
    terminal.clear()?;

    // Define the UI layout
    loop {
        let size = terminal.size()?;
        let mut frame = terminal
            .draw()
            .into_inner();

        frame.clear();

        let text = Text::raw("Hello, TUI world!");
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL))
            .alignment(tui::layout::Alignment::Center);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
            .split(size);

        frame.render_widget(paragraph, chunks[0]);
        terminal.draw()?;
    }
}
