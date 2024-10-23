use std::io;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};

fn main() -> io::Result<()> {
    // Init & Clear Terminal
    let mut terminal = ratatui::init();
    terminal.clear()?;

    // Run terminal
    let app_result = run(terminal);
    ratatui::restore();

    app_result

    // println!("TODO LIST CLI - MEMORY");
    // println!("English");
    // println!("Spanish");
    // println!("Select language");
}

// Function Run
fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let greeting = Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                .white()
                .on_black();
            frame.render_widget(greeting, frame.area());
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}
