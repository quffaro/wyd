// use super::LoadingState;
use crate::app::structs::{
    config::{Config, WydColor},
    focus::{Mode, WindowBase},
    ListNav,
};
use crate::app::App;
use crate::{home_path, CONFIG_SEARCH_FOLDER, GITCONFIG_SUFFIX};
use ratatui::backend::Backend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::terminal::Frame;
use ratatui::widgets::{
    Block, BorderType, Borders, Cell, Clear, List, ListItem, Paragraph, Row, Table, Wrap,
};
use std::env;

pub mod base;
pub mod popup;

pub fn main_ui<'a, B: Backend>(app: &App, frame: &mut Frame<'_, B>) {}

fn wyd_to_color(color: WydColor) -> Color {
    match color {
        WydColor::Yellow => Color::Magenta,
        WydColor::Red => Color::Red,
        WydColor::Blue => Color::Blue,
        WydColor::Green => Color::Green,
        WydColor::Black => Color::Black,
        WydColor::Reset => Color::Reset,
    }
}

// fn style_color(app: &App) -> Color {
//     match app.config.clone() {
//         None => Color::Yellow,
//         Some(c) => wyd_to_color(c),
//     }
// }

pub fn render_loading<'a>(app: &App) -> Paragraph {
    let (text, state) = display_loading_gitcommit(app);
    let loading = Paragraph::new(text)
        .block(
            Block::default()
                // .title(format!("(wyd): {}", app.msg))
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL), // .border_type(BorderType::Rounded),
        )
        .style(
            Style::default()
                .fg(match state {
                    LoadingState::Loading => Color::Yellow,
                    LoadingState::Finished => Color::Green,
                })
                .bg(Color::Black),
        )
        .alignment(Alignment::Left);

    loading
}

fn display_loading_gitcommit<'a>(app: &App) -> (&str, LoadingState) {
    match app.jobs.gitcommit {
        super::LoadingState::Loading => match app.tick {
            0 => ("⠾ loading commits...", LoadingState::Loading),
            1 => ("⠽ loading commits...", LoadingState::Loading),
            2 => ("⠻ loading commits...", LoadingState::Loading),
            3 => ("⠯ loading commits...", LoadingState::Loading),
            _ => ("⠷ loading commits...", LoadingState::Loading),
        },
        _ => ("LOADED!", LoadingState::Finished),
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub fn centered_rect_category(percent_x: u16, percent_y: u16, r: Rect) -> (Rect, Rect) {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    let popup = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1];

    let y = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(85), Constraint::Length(15)])
        .split(popup);

    (y[0], y[1])
}

fn centered_rect_config(percent_x: u16, percent_y: u16, r: Rect) -> (Rect, Rect) {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    let popup = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1];

    let y = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(85), Constraint::Percentage(15)])
        .split(popup);

    (y[0], y[1])
}
