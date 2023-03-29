use crate::app::structs::{
    windows::{BaseWindow, Mode, WindowStatus},
    ListNav,
};
use crate::app::{App, Popup};
use crate::{home_path, CONFIG_SEARCH_FOLDER, GITCONFIG_SUFFIX};
use ratatui::backend::Backend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::terminal::Frame;
use ratatui::widgets::{
    Block, BorderType, Borders, Cell, Clear, List, ListItem, Paragraph, Row, Table, Wrap,
};

use super::LoadingState;

pub fn main_ui<'a, B: Backend>(app: &App, frame: &mut Frame<'_, B>) {}

pub fn render_popup_wyd_confg<'a, B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let size = frame.size();
    let area = centered_rect(40, 10, size);
    frame.render_widget(Clear, area);

    let (owner, directory) = centered_rect_config(40, 10, size);

    // OWNER
    let text = Paragraph::new(app.input.value())
        .style(Style::default().fg(match app.window.mode {
            Mode::Insert => Color::Yellow,
            Mode::Normal => Color::Green,
        }))
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .title("(what is your gh username?)")
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL),
        );

    // SEARCH DIRECTORY
    frame.render_widget(text, area);
}

pub fn render_popup_new_cat<'a, B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    match app.projects.current() {
        Some(_) => {
            let size = frame.size();
            let area = centered_rect(40, 40, size);
            frame.render_widget(Clear, area);

            let (big, small) = centered_rect_category(40, 40, size);

            let category_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(if app.window.popup == Popup::EditCat {
                    Color::Yellow
                } else {
                    Color::Gray
                }))
                .title("(edit category)")
                .border_type(BorderType::Plain);

            let categories: Vec<ListItem> = app
                .categories
                .items
                .iter()
                .map(|t| ListItem::new(format!("{}", t.name)))
                .collect();

            // IF THERE ARE NONE
            let category_list = List::new(categories)
                .block(category_block)
                .highlight_style(
                    Style::default()
                        .fg(app.window.mode_color())
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            frame.render_stateful_widget(category_list, big, &mut app.categories.state);

            let text = Paragraph::new(app.input.value())
                .style(
                    Style::default().fg(match (app.window.popup, app.window.mode) {
                        (Popup::NewCat, Mode::Insert) => Color::Yellow,
                        (Popup::NewCat, Mode::Normal) => Color::Green,
                        _ => Color::Gray,
                    }),
                )
                .wrap(Wrap { trim: false })
                .block(
                    Block::default()
                        .title("(new category)")
                        .title_alignment(Alignment::Left)
                        .borders(Borders::ALL),
                );

            frame.render_widget(text, small);
        }
        None => {
            let size = frame.size();
            let area = centered_rect(40, 40, size);
            frame.render_widget(Clear, area);

            let msg = Paragraph::new("No project selected".to_owned());
            frame.render_widget(msg, area);
        }
    }
}

pub fn render_popup_help_table<'a, B: Backend>(app: &App, frame: &mut Frame<'_, B>) {
    let size = frame.size();
    let area = centered_rect(80, 40, size);
    frame.render_widget(Clear, area); //this clears out the background

    let rows: Vec<Row> = vec![
        Row::new(vec![Cell::from("h"), Cell::from(":)")]),
        Row::new(vec![Cell::from("a"), Cell::from("add project in pwd")]),
        Row::new(vec![
            Cell::from("A"),
            Cell::from("add project if it is a git project"),
        ]),
        Row::new(vec![
            Cell::from("p"),
            Cell::from("recursively search for git configs in Documents"),
        ]),
        Row::new(vec![
            Cell::from("e"),
            Cell::from("edit project description"),
        ]),
        Row::new(vec![Cell::from("r"), Cell::from("edit project category")]),
        Row::new(vec![
            Cell::from("t"),
            Cell::from("add todo item under project"),
        ]),
        Row::new(vec![Cell::from("d"), Cell::from("delete todo item")]),
        Row::new(vec![Cell::from("Enter"), Cell::from("toggle")]),
        Row::new(vec![Cell::from("Esc"), Cell::from("go to Normal mode")]),
        Row::new(vec![Cell::from("i"), Cell::from("go to Visual mode")]),
        Row::new(vec![
            Cell::from("w"),
            Cell::from("save and quit from popup"),
        ]),
        Row::new(vec![Cell::from("q"), Cell::from("quit popup without save")]),
    ];

    let help = Table::new(rows)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .title("(help)")
                .borders(Borders::ALL)
                .style(Style::default().fg(if app.window.popup == Popup::Help {
                    Color::Yellow
                } else {
                    Color::Gray
                }))
                .border_type(BorderType::Plain),
        )
        .header(Row::new(vec!["Key", "Desc"]))
        .widths(&[Constraint::Percentage(10), Constraint::Percentage(90)]);

    frame.render_widget(help, area)
}

pub fn render_popup_search_config<'a, B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // TODO performance

    let size = frame.size();
    let area = centered_rect(80, 40, size);
    frame.render_widget(Clear, area); //this clears out the background

    // which popup is decided here
    let popup = render_popup_config_paths(&app);
    frame.render_stateful_widget(popup, area, &mut app.configs.state);
}

fn render_popup_config_paths<'a>(app: &App) -> Table<'a> {
    // TODO simplify
    let rows: Vec<Row> = app
        .configs
        .items
        .iter()
        .map(|p| {
            // TODO fix
            let y = p
                .path
                // .replace(&home_path(), ".../")
                .replace("/.git.config", "")
                .clone();
            Row::new(vec![match p.is_selected {
                true => Cell::from(format!("[x] {}", y)),
                false => Cell::from(format!("[ ] {}", y)),
            }])
        })
        .collect();

    let title = format!(
        "{} {}/{}",
        "Possible",
        app.configs.state.selected().map_or_else(|| 0, |x| x + 1),
        rows.len()
    );

    let paths = Table::new(rows)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .style(
                    Style::default()
                        .fg(app.window.mode_color())
                        .bg(Color::Black),
                )
                .border_type(BorderType::Plain),
        )
        .widths(&[Constraint::Percentage(100)])
        .highlight_style(
            Style::default()
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(" >>");

    paths
}

pub fn render_popup_edit_desc<'a, B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    match app.projects.current() {
        Some(p) => {
            let size = frame.size();
            let area = centered_rect(40, 40, size);
            frame.render_widget(Clear, area);

            match app.is_popup_loading {
                true => {
                    app.input = tui_input::Input::new(p.desc.clone());
                    app.is_popup_loading = false;
                }
                _ => {}
            }

            let text = Paragraph::new(app.input.value())
                .style(Style::default().fg(match app.window.mode {
                    Mode::Insert => Color::Yellow,
                    Mode::Normal => Color::Green,
                }))
                .wrap(Wrap { trim: false })
                .block(
                    Block::default()
                        .title("(edit description)")
                        .title_alignment(Alignment::Left)
                        .borders(Borders::ALL),
                );

            frame.render_widget(text, area);
        }
        None => {
            let size = frame.size();
            let area = centered_rect(40, 40, size);
            frame.render_widget(Clear, area);

            let msg = Paragraph::new("No project selected".to_owned());
            frame.render_widget(msg, area);
        }
    }
}

pub fn render_popup_todo<'a, B: Backend>(app: &App, frame: &mut Frame<'_, B>) {
    match app.projects.current() {
        Some(_) => {
            let size = frame.size();
            let area = centered_rect(40, 40, size);
            frame.render_widget(Clear, area);

            let text = Paragraph::new(app.input.value())
                .style(Style::default().fg(match app.window.mode {
                    Mode::Insert => Color::Yellow,
                    Mode::Normal => Color::Green,
                }))
                .block(
                    Block::default()
                        .title("(add todo)")
                        .title_alignment(Alignment::Left)
                        .borders(Borders::ALL),
                );

            frame.render_widget(text, area);
        }
        None => {
            let size = frame.size();
            let area = centered_rect(40, 40, size);
            frame.render_widget(Clear, area);

            let msg = Paragraph::new("No project selected".to_owned());
            frame.render_widget(msg, area);
        }
    }
}

pub fn render_title<'a>(app: &App) -> Paragraph {
    Paragraph::new("This is a tui-rs template.")
        .block(
            Block::default()
                .title(format!("(wyd): {}", app.msg))
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL), // .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow).bg(Color::Reset))
        .alignment(Alignment::Left)
}

pub fn render_projects<'a>(app: &App) -> Table<'a> {
    let home_dir = home_path(CONFIG_SEARCH_FOLDER);
    let rows: Vec<Row> = app
        .projects
        .items
        .iter()
        .map(|p| {
            Row::new(vec![
                Cell::from(
                    p.name
                        .replace(&home_dir, "...")
                        .replace(GITCONFIG_SUFFIX, "") // TODO into constant
                        .clone(),
                ),
                Cell::from(p.category.to_string().clone()),
                Cell::from(p.status.to_string().clone()),
                Cell::from(p.last_commit.to_string().clone()),
            ])
        })
        .collect();

    let projects = Table::new(rows)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .title("(projects)")
                .borders(Borders::ALL)
                .style(
                    Style::default()
                        .fg(if app.window.base == BaseWindow::Project {
                            Color::Yellow
                        } else {
                            Color::White
                        })
                        .bg(Color::Reset),
                )
                .border_type(BorderType::Plain),
        )
        .header(Row::new(vec!["Name", "Cat", "Status", "Last Commit"]))
        .widths(&[
            Constraint::Percentage(40),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(40),
        ])
        .highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Reset)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    projects
}

pub fn render_todo_and_desc<'a>(app: &App) -> (List<'a>, Paragraph<'a>) {
    let todo_block = Block::default()
        .borders(Borders::ALL)
        .style(
            Style::default()
                .fg(if app.window.base == BaseWindow::Todo {
                    Color::Yellow
                } else {
                    Color::White
                })
                .bg(Color::Reset),
        )
        .title("(todo)")
        .border_type(BorderType::Plain);

    let todo_items: Vec<ListItem> = app
        .todos
        .filtered
        .iter()
        .map(|t| {
            if t.is_complete {
                ListItem::new(format!("[x] {}", t.todo.clone()))
            } else {
                ListItem::new(format!("[ ] {}", t.todo.clone()))
            }
        })
        .collect();

    let left = List::new(todo_items).block(todo_block).highlight_style(
        Style::default()
            .fg(if app.window.base == BaseWindow::Todo {
                Color::Yellow
            } else {
                Color::White
            })
            .bg(Color::Reset)
            .add_modifier(Modifier::BOLD),
    );

    let project_desc = match app.projects.get_state_selected() {
        Some(i) => match app.projects.items.iter().nth(i) {
            Some(p) => p.desc.to_owned(),
            None => "".to_owned(),
        },
        None => "".to_owned(),
    };

    let right = Paragraph::new(project_desc)
        .block(
            Block::default()
                .title("(description)")
                .borders(Borders::ALL),
        )
        .style(
            Style::default()
                .fg(if app.window.base == BaseWindow::Description {
                    Color::Yellow
                } else {
                    Color::White
                })
                .bg(Color::Reset),
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });

    (left, right)
}

pub fn render_loading<'a>(app: &App) -> Paragraph {
    let (text, state) = display_loading_gitcommit(app);
    let loading = Paragraph::new(text)
        .block(
            Block::default()
                // .title(format!("(wyd): {}", app.msg))
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL), // .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(match state {LoadingState::Loading => Color::Yellow, LoadingState::Finished => Color::Green }).bg(Color::Reset))
        .alignment(Alignment::Left);

    loading
}

fn display_loading_gitcommit<'a>(app: &App) -> (&str, LoadingState) {
    match app.jobs.gitcommit {
        super::LoadingState::Loading => match rand::random::<u8>() % 5 {
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
