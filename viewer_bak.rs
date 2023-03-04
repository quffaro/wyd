use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
// use git2::{Commit, ObjectType, Repository};
use std::{io, path::PathBuf};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{
        Block, BorderType, Borders, Cell, Clear, ListState, Paragraph, Row, Table, TableState,
    },
    Frame, Terminal,
};
//
use glob::{glob, Paths, PatternError};
use rusqlite::Connection;
use shellexpand;

#[derive(Debug, Clone)]
enum Status {
    Stable,
    Unstable,
}

#[derive(Clone, Debug)]
struct TmpGitPath {
    path: String,
    is_selected: bool,
}

// item on the list
#[derive(Debug, Clone)]
struct Project {
    id: usize,
    name: String,
    path: String,
    git: String,
    cat: String,
    status: Status,
}

// fn get_projects() -> Vec<Project> {
//     let conn = Connection::open("projects.db")?;

//     let mut stmt = conn.prepare("SELECT id, path FROM project")?;
//     let project_iter = stmt.query_map([], |row| {
//         Ok(Project {
//             id: row.get(0)?,
//             name: row.get(1)?,
//             path: row.get(2)?,
//             git: row.get(3)?,
//             cat: row.get(4)?,
//             status: match row.get(5)? {
//                 "Stable" => Status::Stable,
//                 "Unstable" => Status::Unstable,
//             },
//         })
//     })?;
// }

fn load_items() -> Vec<Project> {
    vec![
        Project {
            id: 1,
            name: "null".to_string(),
            path: "idk".to_string(),
            git: "null".to_string(),
            cat: "null".to_string(),
            status: Status::Stable,
        },
        Project {
            id: 2,
            name: "project 2".to_string(),
            path: "where?".to_string(),
            git: "null".to_string(),
            cat: "Paper".to_string(),
            status: Status::Unstable,
        },
    ]
}


        

struct App {
    show_popup: bool,
    selected_window: u8,
    message: String,
    configs: TableState,
    table: TableState,
    // TODO rename items with projects
    items: Vec<Project>,
}

impl App {
    fn new() -> App {
        App {
            show_popup: false,
            selected_window: 0,
            message: "0".to_string(),
            configs: TableState::default(),
            table: TableState::default(),
            items: load_items(),
        }
    }
    fn next(&mut self) {
        if self.selected_window == 0 {
            self.table.select(self.table.selected().map(|s| {
                if s + 1 == self.items.len() {
                    0
                } else {
                    s + 1
                }
            }))
        } else {
            self.configs.select(self.configs.selected().map(|s| s + 1))
        }
    }
    fn previous(&mut self) {
        if self.selected_window == 0 {
            self.table.select(self.table.selected().map(|s| {
                if s == 0 {
                    self.items.len() - 1
                } else {
                    s - 1
                }
            }))
        } else {
            self.configs.select(self.configs.selected().map(|s| s - 1))
        }
    }
    fn toggle_config(&mut self) {
        let selected = self.selected().unwrap();

        let conn = Connection::open("projects.db");

        let mut stmt = conn.prepare("INSERT INTO projects (name, path) VALUES (?)")?;
        stmt.execute([

        Ok(())

    }

}

pub fn viewer() -> Result<(), Box<dyn std::error::Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // App
    let app = App::new();
    let res = run_app(&mut terminal, app);

    // Exit App
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    // select
    app.table.select(Some(0));
    app.configs.select(Some(0));
    // draw
    loop {
        terminal.draw(|rect| ui(rect, &mut app))?;

        if let Event::Key(key) = event::read().expect("Key error") {
            match key.code {
                KeyCode::Char('q') => {
                    if app.selected_window == 1 {
                        app.show_popup = false;
                        app.selected_window = 0
                    } else {
                        return Ok(());
                    }
                }
                KeyCode::Char('p') => {
                    app.show_popup = !app.show_popup;
                    if app.show_popup {
                        app.selected_window = 1
                    } else {
                        app.selected_window = 0
                    };
                }
                KeyCode::Char('Enter') => {
                    if app.selected_window = 1 {
                        app.toggle_config();
                    } else {
                        app.toggle_project();
                    }
                }
                KeyCode::Down => {
                    app.next();
                    app.message = app.table.selected().unwrap().to_string();
                }
                KeyCode::Up => {
                    app.previous();
                    app.message = app.table.selected().unwrap().to_string();
                }
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(rect: &mut Frame<B>, app: &mut App) {
    let size = rect.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                // greeting
                Constraint::Length(03),
                // table
                Constraint::Percentage(50),
                // todo list
                Constraint::Percentage(25),
                Constraint::Percentage(15),
            ]
            .as_ref(),
        )
        .split(size);

    let greeting = format!("{}", app.message);
    let title = Paragraph::new(greeting)
        .style(Style::default().fg(Color::LightCyan))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("(pwd)")
                .border_type(BorderType::Plain),
        );

    rect.render_widget(title, chunks[0]);
    let projects = render_projects(&app);
    rect.render_stateful_widget(projects, chunks[1], &mut app.table);
    if app.show_popup {
        let block = render_paths(&app);
        // Block::default().title("Initialize").borders(Borders::ALL);
        let area = centered_rect(80, 40, size);
        rect.render_widget(Clear, area); //this clears out the background
        rect.render_stateful_widget(block, area, &mut app.configs);
    }
}

fn render_projects<'a>(app: &App) -> Table<'a> {
    let rows: Vec<Row> = app
        .items
        .iter()
        .map(|p| {
            Row::new(vec![
                Cell::from(p.name.clone()),
                Cell::from(p.path.clone()),
                Cell::from(p.cat.clone()),
                // Cell::from(p.status.clone()),
            ])
        })
        .collect();

    let projects = Table::new(rows)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .title("Projects")
                .borders(Borders::ALL)
                .style(Style::default().fg(if app.selected_window == 0 {
                    Color::Yellow
                } else {
                    Color::White
                }))
                .border_type(BorderType::Plain),
        )
        // .header(Row::new(vec!["Col1", "Col2"]))
        .widths(&[
            Constraint::Length(30),
            Constraint::Length(10),
            Constraint::Length(10),
        ])
        .highlight_style(
            Style::default()
                .bg(if app.selected_window == 0 {
                    Color::Yellow
                } else {
                    Color::Gray
                })
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">>");

    projects
}

fn render_paths<'a>(app: &App) -> Table<'a> {
    let rows: Vec<Row> = read_config()
        .unwrap()
        .iter()
        .map(|p| {
            Row::new(vec![
                Cell::from(p.path.clone()),
                Cell::from(p.is_selected.to_string().clone()),
            ])
        })
        .collect();

    // TODO FIX INDEXING
    let title = format!(
        "{} {}/{}",
        "Possible",
        app.configs.selected().unwrap() + 1,
        rows.len()
    );

    let paths = Table::new(rows)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Yellow).bg(Color::Black))
                .border_type(BorderType::Plain),
        )
        .widths(&[Constraint::Length(80), Constraint::Length(20)])
        .highlight_style(
            Style::default()
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">>");

    paths
}

//
fn read_configs() -> Result<Vec<String>, rusqlite::Error> {
    let conn = Connection::open("projects.db")?;

    let mut stmt = conn.prepare("select path from tmp_git_config")?;
    let tgp_iter = stmt.query_map([], |row| row.get(0))?.collect();

    tgp_iter
}

fn read_config() -> Result<Vec<TmpGitPath>, rusqlite::Error> {
    let conn = Connection::open("projects.db")?;

    let mut stmt = conn.prepare("select path, is_selected from tmp_git_config")?;
    let tgp = stmt
        .query_map([], |row| {
            Ok(TmpGitPath {
                path: row.get(0)?,
                is_selected: row.get(1)?,
            })
        })?
        .collect();

    tgp
}

// TODO make generic
struct VecInfo {
    vec: Vec<String>,
    len: usize,
}

// fn vec_enrich(v: Vec<String>) -> VecInfo {
//     let y = VecInfo {
//         vec: v,
//         len: v.len(),
//     };

//     y
// }

// TODO be replaced by query
pub fn fetch_configs() -> Vec<String> {
    let expanded_path = shellexpand::tilde("~/Documents/");
    let pattern: PathBuf = [&expanded_path, "**/.git/config"].iter().collect();

    let tmp: Vec<String> = glob(pattern.to_str().unwrap())
        .expect("expectation!!")
        .filter_map(|x| x.ok())
        .map(|x| x.into_os_string().into_string().unwrap())
        .collect();

    tmp
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
