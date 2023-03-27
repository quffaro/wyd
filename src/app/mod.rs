use crate::app::structs::{projects::Project, todos::Todo, FilteredListItems, ListNav, TableItems};
use crate::{home_path, CONFIG_SEARCH_FOLDER, GITCONFIG_SUFFIX, PATH_DB};
use dirs::home_dir;
use ratatui::backend::Backend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::terminal::Frame;
use ratatui::widgets::{
    Block, BorderType, Borders, Cell, List, ListItem, Paragraph, Row, Table, Wrap,
};
use rusqlite::Connection;
use std::error;

pub mod regex;
pub mod structs;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub projects: TableItems<Project>,
    pub todos: FilteredListItems<Todo>,
    pub desc: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            projects: TableItems::<Project>::default(),
            todos: FilteredListItems::<Todo>::default(),
            desc: "".to_owned(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads data into the app
    pub fn load() -> Self {
        let conn = Connection::open(home_path(PATH_DB)).unwrap();
        Self {
            running: false,
            projects: TableItems::<Project>::load(&conn),
            todos: FilteredListItems::<Todo>::load(&conn),
            desc: "".to_owned(),
        }
    }
    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Renders the user interface widgets.
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        // This is where you add new widgets.
        // See the following resources:
        // - https://docs.rs/tui/latest/tui/widgets/index.html
        // - https://github.com/fdehau/tui-rs/tree/master/examples
        let size = frame.size();

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

        let title = Paragraph::new(
            "This is a tui-rs template.\nPress `Esc`, `Ctrl-C` or `q` to stop running.",
        )
        .block(
            Block::default()
                .title("Template")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL), // .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow).bg(Color::Black))
        .alignment(Alignment::Center);

        // CHUNK 0
        frame.render_widget(title, chunks[0]);

        // CHUNK 1
        let projects = render_projects(self);
        frame.render_stateful_widget(projects, chunks[1], &mut self.projects.state);

        let project_info_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[2]);

        let (todo_block, desc_block) = render_todo_and_desc(self);
        frame.render_stateful_widget(todo_block, project_info_chunk[0], &mut self.todos.state);
        frame.render_widget(desc_block, project_info_chunk[1]);
    }
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
fn render_todo_and_desc<'a>(app: &App) -> (List<'a>, Paragraph<'a>) {
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
