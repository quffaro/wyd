use ratatui::backend::Backend;
use ratatui::layout::Alignment;
use ratatui::style::{Color, Style};
use ratatui::terminal::Frame;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use std::error;
/// Application result type.
pub mod gitconfig;
pub mod projects;
pub mod todos;
pub mod windows;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
}

impl Default for App {
    fn default() -> Self {
        Self { running: true }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Renders the user interface widgets.
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        // This is where you add new widgets.
        // See the following resources:
        // - https://docs.rs/tui/latest/tui/widgets/index.html
        // - https://github.com/fdehau/tui-rs/tree/master/examples
        frame.render_widget(
            Paragraph::new(
                "This is a tui-rs template.\nPress `Esc`, `Ctrl-C` or `q` to stop running.",
            )
            .block(
                Block::default()
                    .title("Template")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .alignment(Alignment::Center),
            frame.size(),
        )
    }
}
