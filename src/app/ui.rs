use ratatui::backend::Backend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Span, Spans};
use ratatui::widgets::{Block, BorderType, Borders, Cell, LineGauge, Paragraph, Row, Table};
use ratatui::{symbols, Frame};

use super::actions::Actions;
use super::state::AppState;
use crate::app::App;

pub fn draw<B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let size = rect.size();

    // Vertical layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                // Constraint::Min(10),
                // Constraint::Length(3),
                // Constraint::Length(12),
            ]
            .as_ref(),
        )
        .split(size);

    // Title
    let title = Paragraph::new("A");
    rect.render_widget(title, chunks[0]);
}
