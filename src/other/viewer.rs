// TODO:
// [] call github to see last push
// [] search TODO in directory
// [] press a to add project in current directory
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{fmt, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{
        Block, BorderType, Borders, Cell, Clear, List, ListItem, ListState, Paragraph, Row, Table,
        TableState,
    },
    Frame, Terminal,
};
use wyd::SEARCH_DIRECTORY_PREFIX;

use super::sql::{read_project, read_tmp, read_todo, update_tmp, write_tmp_to_project};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Status {
    Stable,
    Unstable,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait ListNavigate {
    fn get_items_len<'a>(&'a self) -> usize;
    fn get_state_selected<'a>(&'a self) -> Option<usize>;
    fn select_state<'a>(&'a mut self, idx: Option<usize>);
    //
    fn next(&mut self) {
        let i = match self.get_state_selected() {
            Some(i) => {
                if i >= self.get_items_len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.select_state(Some(i));
    }
    fn previous(&mut self) {
        let i = match self.get_state_selected() {
            Some(i) => {
                if i == 0 {
                    self.get_items_len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.select_state(Some(i));
    }
    fn unselect(&mut self) {
        self.select_state(None);
    }
}

#[derive(Clone, Debug)]
struct ListItems<T> {
    items: Vec<T>,
    state: ListState,
}

impl<T> ListNavigate for ListItems<T> {
    fn get_items_len<'a>(&'a self) -> usize {
        self.items.len()
    }
    fn get_state_selected<'a>(&'a self) -> Option<usize> {
        self.state.selected()
    }
    fn select_state<'a>(&'a mut self, idx: Option<usize>) {
        self.state.select(idx)
    }
}

#[derive(Clone, Debug)]
struct TableItems<T> {
    items: Vec<T>,
    state: TableState,
}

impl<T> ListNavigate for TableItems<T> {
    fn get_items_len<'a>(&'a self) -> usize {
        self.items.len()
    }
    fn get_state_selected<'a>(&'a self) -> Option<usize> {
        self.state.selected()
    }
    fn select_state<'a>(&'a mut self, idx: Option<usize>) {
        self.state.select(idx)
    }
}

#[derive(Clone, Debug)]
pub struct GitConfig {
    pub path: String,
    pub is_selected: bool,
}

impl TableItems<GitConfig> {
    fn new() -> TableItems<GitConfig> {
        TableItems {
            // items: Vec::<GitConfig>::new(),
            items: read_tmp().unwrap(),
            state: TableState::default(),
        }
    }
    fn toggle(&mut self) {
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

#[derive(Debug, Clone)]
pub struct Project {
    pub id: u8,
    pub path: String,
    pub name: String,
    pub category: String,
    pub status: Status,
    pub last_commit: String,
}

impl Project {
    fn cycle_status(&mut self) {
        self.status = match self.status {
            Status::Stable => Status::Unstable,
            Status::Unstable => Status::Stable,
            _ => Status::Unstable,
        }
    }
}

impl TableItems<Project> {
    fn new() -> TableItems<Project> {
        TableItems {
            items: read_project().expect("AA"),
            state: TableState::default(),
        }
    }
    fn toggle(&mut self) {
        let selected = self.state.selected().unwrap();
        for i in 0..self.items.len() {
            if i == selected {
                self.items[i].cycle_status();
            } else {
                continue;
            }
        }
    }
}

// TODO i would like to nest these guys
#[derive(Clone, Debug)]
pub struct Todo {
    pub id: u8,
    pub parent_id: u8,
    pub project_id: u8,
    pub todo: String,
    pub is_complete: bool,
}

impl ListItems<Todo> {
    fn new() -> ListItems<Todo> {
        ListItems {
            items: read_todo().expect("AA"),
            // match project_id {
            //     Some(i) => read_todo(i).expect("AA"),
            //     None    => read_todo(0).expect("AA")
            // },
            state: ListState::default(),
        }
    }
    // TODO can this be a method for ListNavigate?
    fn toggle(&mut self) {
        let selected = self.state.selected().unwrap();
        for i in 0..self.items.len() {
            if i == selected {
                self.items[i].is_complete = !self.items[i].is_complete;
            } else {
                continue;
            }
        }
    }
}

struct App {
    show_popup: bool,
    focused_window: String,
    message: String,
    configs: TableItems<GitConfig>,
    projects: TableItems<Project>,
    todos: ListItems<Todo>,
}

const WINDOW_CONFIGS: &str = "configs";
const WINDOW_PROJECTS: &str = "projects";
const WINDOW_TODO: &str = "todo";
const WINDOW_TODO_SEARCH: &str = "todo";
const WINDOW_ADD_TODO: &str = "add-todo";

// TODO does App need ListNavigate trait?
impl App {
    fn new() -> App {
        App {
            show_popup: false,
            focused_window: "projects".to_owned(),
            message: "hiii".to_owned(),
            configs: TableItems::<GitConfig>::new(),
            projects: TableItems::<Project>::new(),
            todos: ListItems::<Todo>::new(),
        }
    }
    fn next(&mut self) {
        // match self.selected_window {
        //     1 => {
        //         self.projects.next();
        //         self.todos = ListItems::<Todo>::new();
        //     }
        //     0 => self.configs.next(),
        //     _ => self.configs.next(),
        // }
        match self.focused_window {
            WINDOW_PROJECTS => self.projects.next(),
            WINDOW_TODO => self.configs.next(),
            _ => self.configs.next(),
        }
    }
    fn previous(&mut self) {
        match self.focused_window {
            WINDOW_PROJECTS => self.projects.previous(),
            WINDOW_TODO => self.configs.previous(),
            _ => self.configs.previous(),
        }
        // match self.selected_window {
        //     1 => self.projects.previous(),
        //     0 => self.configs.previous(),
        //     _ => self.configs.previous(),
        // }
    }
    fn popup(&mut self) {
        self.show_popup = !self.show_popup;
        if self.show_popup {
            self.focused_window = WINDOW_CONFIGS.to_owned();
        } else {
            self.focused_window = WINDOW_PROJECTS.to_owned();
            write_tmp_to_project();
            self.projects = TableItems::<Project>::new();
        }
    }
    fn popup_add_task(&mut self) {
        self.show_popup = !self.show_popup;
        if self.show_popup {
            self.focused_window = WINDOW_ADD_TODO
        } else {
            self.focused_window = WINDOW_TODO;
        }
    }
    fn default_select(&mut self) {
        self.projects.state.select(Some(0));
        self.configs.state.select(Some(0));
    }
    fn toggle(&mut self) {
        match self.focused_window {
            WINDOW_CONFIGS => self.configs.toggle(),
            WINDOW_TODO => self.projects.toggle(),
            _ => self.configs.toggle(),
        }
    }
    fn cycle_focus_next(&mut self) {
        self.focused_window = match self.focused_window.clone() {
            WINDOW_CONFIGS => WINDOW_CONFIGS.to_owned(),
            WINDOW_PROJECTS => WINDOW_TODO.to_owned(),
            WINDOW_TODO => WINDOW_TODO_SEARCH.to_owned(),
            WINDOW_TODO_SEARCH => WINDOW_PROJECTS.to_owned(),
            _ => WINDOW_PROJECTS.to_owned(),
        }
    }
    fn cycle_focus_previous(&mut self) {
        self.focused_window = match self.focused_window.clone() {
            WINDOW_CONFIGS => WINDOW_CONFIGS.to_owned(),
            WINDOW_PROJECTS => WINDOW_TODO_SEARCH.to_owned(),
            WINDOW_TODO_SEARCH => WINDOW_TODO.to_owned(),
            WINDOW_TODO => WINDOW_PROJECTS.to_owned(),
            _ => WINDOW_PROJECTS.to_owned(),
        }
    }
    fn filter_todo(&mut self) -> Vec<Todo> {
        let project_id = self.projects.get_state_selected().unwrap() as u8;
        self.todos
            .items
            .clone()
            .into_iter()
            .filter(|t| t.project_id == project_id)
            .collect()
    }
    fn quit(&mut self) {}
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
    app.default_select();
    // draw
    loop {
        terminal.draw(|rect| ui(rect, &mut app))?;

        // TODO I want to Tab through interfaces
        if let Event::Key(key) = event::read().expect("Key error") {
            match key.code {
                KeyCode::Char('q') => {
                    if app.focused_window == WINDOW_CONFIGS {
                        app.show_popup = false;
                        app.focused_window = WINDOW_PROJECTS.to_owned();
                    } else {
                        return Ok(());
                    }
                }
                // TODO add projects in current directory
                KeyCode::Char('p') => app.popup(),
                // TODO help box
                KeyCode::Char('h') => app.popup(),
                // KeyCode::Char('r') => app.reload(),
                KeyCode::Char('a') => {}
                // navigate
                KeyCode::Char(';') => app.cycle_focus_next(),
                KeyCode::Char('j') => app.cycle_focus_previous(),
                KeyCode::Char('l') => app.next(),
                KeyCode::Char('k') => app.previous(),
                KeyCode::Enter => app.toggle(),
                _ => {}
            }
        }
    }
}

//
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

    let title = Paragraph::new(&*app.message)
        .style(Style::default().fg(Color::LightCyan))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("(pwd)")
                .border_type(BorderType::Plain),
        );

    // chunk 0: title
    rect.render_widget(title, chunks[0]);

    // chunk 1: projects
    if app.projects.items.len() == 0 {
        let no_projects = render_no_projects(&app);
        rect.render_widget(no_projects, chunks[1]);
    } else {
        let projects = render_projects(&app);
        rect.render_stateful_widget(projects, chunks[1], &mut app.projects.state);
    }

    // chunk 2: todo list
    let todo_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[2]);

    let (left_todo_list, right_todo_search) = render_todo(&app);
    rect.render_widget(left_todo_list, todo_chunks[0]);
    rect.render_widget(right_todo_search, todo_chunks[1]);

    // popup
    // TODO fuzzy find
    if app.show_popup {
        let block = render_config_paths(&app);
        // Block::default().title("Initialize").borders(Borders::ALL);
        let area = centered_rect(80, 40, size);
        rect.render_widget(Clear, area); //this clears out the background
        rect.render_stateful_widget(block, area, &mut app.configs.state);
    };
}

fn render_no_projects<'a>(app: &App) -> Paragraph<'a> {
    let msg = "Press `p` to add projects".to_owned();
    let no_projects = Paragraph::new(msg)
        .style(
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::ITALIC),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("(Projects)")
                .border_type(BorderType::Plain),
        );

    no_projects
}

// render projects
fn render_projects<'a>(app: &App) -> Table<'a> {
    let rows: Vec<Row> = app
        .projects
        .items
        .iter()
        .map(|p| {
            Row::new(vec![
                Cell::from(p.name.replace(SEARCH_DIRECTORY_PREFIX, "...").clone()),
                // Cell::from(p.path.replace(SEARCH_DIRECTORY_PREFIX, "...").clone()),
                Cell::from(p.category.clone()),
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
                .style(
                    Style::default().fg(if app.focused_window == WINDOW_PROJECTS {
                        Color::Yellow
                    } else {
                        Color::White
                    }),
                )
                .border_type(BorderType::Plain),
        )
        .header(Row::new(vec!["Name", "Cat", "Status", "Last Commit"]))
        .widths(&[
            Constraint::Length(30),
            // Constraint::Length(40),
            Constraint::Length(20),
            Constraint::Length(20),
            Constraint::Length(20),
        ])
        .highlight_style(
            Style::default()
                .bg(if app.focused_window == WINDOW_PROJECTS {
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

// render paths
fn render_config_paths<'a>(app: &App) -> Table<'a> {
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
        .widths(&[Constraint::Length(50), Constraint::Length(20)])
        .highlight_style(
            Style::default()
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">>");

    paths
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

fn render_todo<'a>(app: &App) -> (List<'a>, List<'a>) {
    let todo_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(
            // TODO this should be a rule
            if app.focused_window == WINDOW_TODO {
                Color::Yellow
            } else {
                Color::White
            },
        ))
        .title("(todo)")
        .border_type(BorderType::Plain);

    // filter
    let todo_items: Vec<ListItem> = app
        .todos
        .items
        .iter()
        .map(|x| ListItem::new(x.todo.clone()))
        .collect();

    let left = List::new(todo_items).block(todo_block).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    let search_todo_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(
            // TODO this can be a function
            if app.focused_window == WINDOW_TODO_SEARCH {
                Color::Yellow
            } else {
                Color::White
            },
        ))
        .title("(todo)")
        .border_type(BorderType::Plain);

    let search_todo_items = vec![];

    let right = List::new(search_todo_items)
        .block(search_todo_block)
        .highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

    (left, right)
}
