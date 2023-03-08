// TODO:
// [] call github to see last push
// [] search TODO in directory
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rusqlite::Connection;
use std::{fmt, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{
        Block, BorderType, Borders, Cell, Clear, Paragraph, Row, Table, TableState,
    },
    Frame, Terminal,
};

// TODO move to config 
const SEARCH_DIRECTORY_PREFIX: &str = "/home/cuffaro/Documents";

#[derive(PartialEq, Eq, Debug, Clone)]
enum Status {
    Stable,
    Unstable,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone,Debug)]
struct TableItems<T> {
    items: Vec<T>,
    state: TableState
}

impl TableItems<T> {
    fn new(items: Vec<T>) -> TableItems<T> {
        TableItems<T> {
            items,
            state: TableState::default(),
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    pub fn unselect(&mut self) {
        self.state.select(None);
    }
    pub fn toggle(&mut self, f Function) {
        let selected = self.state.selected().unwrap();
        for i in 0..self.items.len() {
            if i == selected {
                continue;
                // TODO execute function here 
            } else {
                continue;
            }
        }
    }
}


#[derive(Clone, Debug)]
struct TmpGitPath {
    path: String,
    is_selected: bool,
}


#[derive(Clone, Debug)]
struct TmpGitPathTable {
    items: Vec<TmpGitPath>,
    state: TableState,
}


impl TmpGitPathTable {
    fn new(items: Vec<TmpGitPath>) -> TmpGitPathTable {
        TmpGitPathTable {
            items,
            state: TableState::default(),
        }
    }
    fn load() -> TmpGitPathTable {
        TmpGitPathTable {
            items: read_tmp().unwrap(),
            state: TableState::default(),
        }
    }
    // should we generalize 
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    pub fn unselect(&mut self) {
        self.state.select(None);
    }
    pub fn toggle(&mut self) {
        let selected = self.state.selected().unwrap();
        for i in 0..self.items.len() {
            if i == selected {
                self.items[i].is_selected = !self.items[i].is_selected;
                // move project db commit to popup toggle
                update_tmp(&self.items[i]);
            } else {
                continue;
            }
        }
    }
}

// item on the list
#[derive(Debug, Clone)]
struct Project {
    name: String,
    path: String,
    cat: String,
    status: Status,
    last_commit: String,
}

impl Project {
    pub fn change_status(&mut self) {
        if self.status == Status::Stable {
            self.status = Status::Unstable
        } else {
            self.status = Status::Stable
        }
    }
    // fn get_last_commit(&mut self) {
    //     self.last_commit = crate::other::request::request().unwrap().to_string()
    // }
}

#[derive(Clone, Debug)]
struct ProjectTable {
    items: Vec<Project>,
    state: TableState,
}

impl ProjectTable {
    fn load() -> ProjectTable {
        ProjectTable {
            items: read_projects().unwrap(),
            state: TableState::default(),
        }
    }
    fn reload(&mut self) -> ProjectTable {
        ProjectTable {
            items: read_projects().unwrap(),
            state: TableState::default(),
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    pub fn unselect(&mut self) {
        self.state.select(None);
    }
    pub fn toggle(&mut self) {
        let selected = self.state.selected().unwrap();
        for i in 0..self.items.len() {
            if i == selected {
                self.items[i].change_status();
            } else {
                continue;
            }
        }
    }
    // TODO get last commits
}

struct App {
    show_popup: bool,
    selected_window: u8,
    message: String,
    configs: TmpGitPathTable,
    projects: ProjectTable,
}

impl App {
    fn new() -> App {
        App {
            show_popup: false,
            selected_window: 0,
            message: "hiii".to_string(),
            configs: TmpGitPathTable::load(),
            projects: ProjectTable::load(),
        }
    }
    fn reload(&mut self) {
        // self.projects.reload();
        self.message = format!("{:#?}", self.projects.items)
    }
    fn next(&mut self) {
        if self.selected_window == 1 {
            self.configs.next()
        } else {
            self.projects.next()
        }
    }
    fn previous(&mut self) {
        if self.selected_window == 1 {
            self.configs.previous()
        } else {
            self.projects.previous()
        }
    }
    fn popup_configs(&mut self) {
        self.show_popup = !self.show_popup;
        if self.show_popup {
            self.selected_window = 1
        } else {
            self.selected_window = 0;
            self.projects.reload();
            self.configs.unselect();
            // TODO write new projects
        }
    }
    fn toggle(&mut self) {
        if self.selected_window == 1 {
            self.configs.toggle();
        } else {
            self.message = self.projects.items.len().to_string();
            self.projects.toggle();
        }
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
    app.projects.state.select(Some(0));
    app.configs.state.select(Some(0));
    // draw
    loop {
        terminal.draw(|rect| ui(rect, &mut app))?;

        if let Event::Key(key) = event::read().expect("Key error") {
            match key.code {
                KeyCode::Char('q') => {
                    if app.selected_window == 1 {
                        app.show_popup = false;
                        app.selected_window = 0;
                    } else {
                        return Ok(());
                    }
                }
                KeyCode::Char('p') => app.popup_configs(),
                KeyCode::Char('r') => app.reload(),
                KeyCode::Enter => app.toggle(),
                KeyCode::Down => app.next(),
                KeyCode::Up => app.previous(),
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
                Constraint::Percentage(40),
                // Constraint::Percentage(15),
            ]
            .as_ref(),
        )
        .split(size);

    let greeting = format!("{}", app.message);
    let title = Paragraph::new("hii")
        .style(Style::default().fg(Color::LightCyan))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("(pwd)")
                .border_type(BorderType::Plain),
        );

    // TODO LIST
    let msg = Paragraph::new(greeting)
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
    rect.render_stateful_widget(projects, chunks[1], &mut app.projects.state);
    rect.render_widget(msg, chunks[2]);

    if app.show_popup {
        let block = render_paths(&app);
        // Block::default().title("Initialize").borders(Borders::ALL);
        let area = centered_rect(80, 40, size);
        rect.render_widget(Clear, area); //this clears out the background
        rect.render_stateful_widget(block, area, &mut app.configs.state);
    };
}

fn render_projects<'a>(app: &App) -> Table<'a> {
    let rows: Vec<Row> = app
        .projects
        .items
        .iter()
        .map(|p| {
            Row::new(vec![
                Cell::from(p.name.replace(SEARCH_DIRECTORY_PREFIX, "").clone()),
                Cell::from(p.path.clone()),
                Cell::from(p.cat.clone()),
                Cell::from(p.status.to_string().clone()),
                Cell::from(p.last_commit.clone()),
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
            Constraint::Length(40),
            Constraint::Length(50),
            Constraint::Length(05),
            Constraint::Length(20),
            Constraint::Length(20),
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
        .highlight_symbol(">> ");

    projects
}

fn render_paths<'a>(app: &App) -> Table<'a> {
    let rows: Vec<Row> = app
        .configs
        .items
        .iter()
        .map(|p| {
            Row::new(vec![
                // TODO remove substring
                Cell::from(p.path.replace(SEARCH_DIRECTORY_PREFIX, "...").clone()),
                Cell::from(p.is_selected.to_string().clone()),
            ])
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
                .style(Style::default().fg(Color::Yellow).bg(Color::Black))
                .border_type(BorderType::Plain),
        )
        .widths(&[Constraint::Length(80), Constraint::Length(20)])
        .highlight_style(
            Style::default()
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    paths
}

// use this i think
fn read_projects() -> Result<Vec<Project>, rusqlite::Error> {
    let conn = Connection::open("projects.db")?;

    let mut stmt = conn.prepare("select path, name from project")?;
    let tgp = stmt
        .query_map([], |row| {
            {
                Ok(Project {
                    path: row.get(0)?,
                    name: row.get(1)?,
                    cat: "".to_string(),
                    status: Status::Stable,
                    last_commit: "Awaiting work to finish".to_owned(),
                    // last_commit: crate::other::request::request().unwrap().to_string(),
                    // if row.get(5)? == Status::Stable.to_string() {
                    // Status::Stable
                    // } else {
                    // Status::Unstable
                    // },
                })
            }
        })
        .expect("query failed")
        .collect();

    tgp
}

fn read_tmp() -> Result<Vec<TmpGitPath>, rusqlite::Error> {
    let conn = Connection::open("projects.db")?;

    let mut stmt =
        conn.prepare("select path, is_selected from tmp_git_config where is_selected = 0")?;
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

fn update_tmp(tmp: &TmpGitPath) -> Result<(), rusqlite::Error> {
    let conn = Connection::open("projects.db")?;

    conn.execute(
        "update tmp_git_config set is_selected = ?1 where path = ?2;",
        (&tmp.is_selected, &tmp.path),
    )
    .expect("A");

    write_tmp_to_project(&tmp)
    // Ok(())
    // TODO might need to default items
}

fn write_tmp_to_project(tmp: &TmpGitPath) -> Result<(), rusqlite::Error> {
    let conn = Connection::open("projects.db")?;
    // match Regex::new(r#"/^(.+)\/([^\/]+)$/gm"#) {
    //     Ok(r) => {
    //         let re = r;
    //     }
    //     Err(e) => {
    //         let path = "error.txt";
    //         let mut output = File::create(path).unwrap();
    //         write!(output, "{}", format!("{:?}", e))
    //     }
    // }

    let mut stmt = conn.prepare(
        "insert or replace into project (path,name,cat,status) values (?1, ?2, ?3, ?4);",
    )?;
    // for x in &tmp {
    // TODO get name of parent directory
    // let caps = re.captures(&tmp.path).unwrap();
    stmt.execute([
        &tmp.path,
        &tmp.path,
        // &caps.get(1).map_or("", |m| m.as_str()).to_owned(),
        // .map_or(&"".to_owned(), |m| &m.as_str().to_string()),
        &"Unknown".to_owned(),
        &"Unstable".to_owned(),
    ])
    .expect("A");
    // }
    // println!("HELLO");

    Ok(())
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
