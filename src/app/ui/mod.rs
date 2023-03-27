use crate::app::structs::ListNav;
use crate::app::App;
use crate::{home_path, CONFIG_SEARCH_FOLDER, GITCONFIG_SUFFIX, PATH_DB};
use ratatui::backend::Backend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::terminal::Frame;
use ratatui::widgets::{
    Block, BorderType, Borders, Cell, Clear, List, ListItem, Paragraph, Row, Table, Wrap,
};

pub fn main_ui<'a, B: Backend>(app: &App, frame: &mut Frame<'_, B>) {}

pub fn render_popup_todo<'a, B: Backend>(app: &App, frame: &mut Frame<'_, B>) {
    match app.projects.current() {
        Some(p) => {
            let size = frame.size();
            let area = centered_rect(40, 40, size);
            frame.render_widget(Clear, area);

            let text = Paragraph::new(app.input.value())
                .style(Style::default().fg(Color::Yellow))
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
    Paragraph::new("This is a tui-rs template.\nPress `Esc`, `Ctrl-C` or `q` to stop running.")
        .block(
            Block::default()
                .title("Template")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL), // .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow).bg(Color::Black))
        .alignment(Alignment::Center)
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
                .style(Style::default().fg(Color::Yellow).bg(Color::Black))
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
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">>");

    projects
}
pub fn render_todo_and_desc<'a>(app: &App) -> (List<'a>, Paragraph<'a>) {
    let todo_block = Block::default()
        .borders(Borders::ALL)
        .style(
            Style::default()
                .fg(
                    Color::Yellow, // if app.state.window.base == BaseWindow::Todo {
                                   // Color::Yellow
                                   // } else {
                                   // Color::White
                                   // }
                )
                .bg(Color::Black),
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
            .bg(
                Color::Yellow, // if app.window.base == BaseWindow::Todo {
                               // Color::Yellow
                               // } else {
                               // Color::White
                               // }
            )
            .fg(Color::Black)
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
                .fg(
                    Color::Yellow, // if app.window.base == BaseWindow::Description {
                                   // Color::Yellow
                                   // } else {
                                   // Color::White
                                   // }
                )
                .bg(Color::Black),
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });

    (left, right)
}
/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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
