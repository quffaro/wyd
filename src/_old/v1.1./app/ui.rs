use ratatui::backend::Backend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{
    Block, BorderType, Borders, Cell, List, ListItem, Paragraph, Row, Table, Wrap,
};
use ratatui::{symbols, Frame};

use super::actions::Actions;
use super::state::AppState;
use super::structs::{windows::BaseWindow, ListNav, PlainListItems, TableItems};
use crate::app::App;

use crate::app::structs::projects::Project;
use crate::{home_path, CONFIG_SEARCH_FOLDER, GITCONFIG_SUFFIX, PATH_DB};
use rusqlite::Connection;

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
                Constraint::Min(10),
                Constraint::Length(3),
                Constraint::Length(12),
            ]
            .as_ref(),
        )
        .split(size);

    // CHUNK 0: TITLE
    let title = Paragraph::new(format!("{}", "A"))
        .style(Style::default().fg(Color::LightCyan))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("(pwd)")
                .border_type(BorderType::Plain),
        );
    rect.render_widget(title, chunks[0]);

    // CHUNK 1: PROJECTS
    let projects = render_projects(app);
    // let mut project_state =
    // TableItems::<Project>::load(&Connection::open(home_path(PATH_DB)).unwrap());
    rect.render_stateful_widget(projects, chunks[1], &mut app.projects.state);

    // CHUNK 2:
    // let project_info_chunk = Layout::default()
    //     .direction(Direction::Horizontal)
    //     .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
    //     .split(chunks[2]);

    // let (left_todo_list, right_todo_search) = render_todo_and_desc(&app);
    // rect.render_stateful_widget(left_todo_list, project_info_chunk[0], &mut app.todos.state);
    // rect.render_widget(right_todo_search, project_info_chunk[1]);
}
fn render_projects<'a>(app: &App) -> Table<'a> {
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

    let projects = Table::new(vec![])
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .title("(projects)")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Yellow))
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
fn render_todo_and_desc<'a>(app: &App) -> (List<'a>, Paragraph<'a>) {
    let todo_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(
            Color::Yellow, // if app.state.window.base == BaseWindow::Todo {
                           // Color::Yellow
                           // } else {
                           // Color::White
                           // }
        ))
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
        .style(Style::default().fg(
            Color::Yellow, // if app.window.base == BaseWindow::Description {
                           // Color::Yellow
                           // } else {
                           // Color::White
                           // }
        ))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });

    (left, right)
}
