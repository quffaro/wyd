// TODO:
// [] call github to see last push
// [] search TODO in directory
// [] press a to add project in current directory
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{
        Block, BorderType, Borders, Cell, Clear, List, ListItem, Paragraph, Row, Table, Wrap,
    },
    Frame, Terminal,
};
use tui_textarea::{Input, Key, TextArea};
use wyd::{
    WindowStatus, CONFIG_PATH_SUFFIX, SEARCH_DIRECTORY_PREFIX, WINDOW_DESCRIPTION,
    WINDOW_POPUP_ADD_TODO, WINDOW_POPUP_CONFIGS, WINDOW_POPUP_DESC, WINDOW_POPUP_EDIT,
    WINDOW_PROJECTS, WINDOW_TODO,
};

use super::{
    sql::{update_project_desc, write_new_todo, write_tmp_to_project},
    structs::{FilteredListItems, GitConfig, ListNavigate, Project, TableItems, Todo, Window},
};

struct App {
    show_popup: bool,
    window: Window,
    message: String,
    configs: TableItems<GitConfig>,
    projects: TableItems<Project>,
    todos: FilteredListItems<Todo>,
}

// TODO does App need ListNavigate trait?
impl App {
    fn new() -> App {
        App {
            show_popup: false,
            window: Window {
                focus: WINDOW_PROJECTS.to_owned(),
                status: WindowStatus::NotLoaded,
            },
            message: "hii".to_owned(),
            configs: TableItems::<GitConfig>::new(),
            projects: TableItems::<Project>::new(),
            todos: FilteredListItems::<Todo>::new(),
        }
    }
    fn next(&mut self) {
        match self.window.focus.as_str() {
            WINDOW_PROJECTS => {
                self.projects.next();
                //
                let idx = self.projects.get_state_selected().unwrap();
                let project = &self.projects.items[idx];
                let items = self.todos.items.clone();
                self.todos.filtered = items
                    .into_iter()
                    .filter(|t| t.project_id == project.id)
                    .collect();
                self.todos.select_state(Some(0));
            }
            WINDOW_TODO => self.todos.next(),
            WINDOW_POPUP_CONFIGS => self.configs.next(),
            _ => self.configs.next(),
        }
    }
    fn previous(&mut self) {
        match self.window.focus.as_str() {
            WINDOW_PROJECTS => {
                self.projects.previous();
                //
                let idx = self.projects.get_state_selected().unwrap();
                let project = &self.projects.items[idx];
                let items = self.todos.items.clone();
                self.todos.filtered = items
                    .into_iter()
                    .filter(|t| t.project_id == project.id)
                    .collect();
                self.todos.select_state(Some(0));
            }
            WINDOW_TODO => self.todos.previous(),
            WINDOW_POPUP_CONFIGS => self.configs.previous(),
            _ => self.configs.previous(),
        }
    }
    fn popup(&mut self) {
        self.show_popup = !self.show_popup;
        if self.show_popup {
            self.window.focus = WINDOW_POPUP_CONFIGS.to_owned();
        } else {
            self.window.focus = WINDOW_PROJECTS.to_owned();
            write_tmp_to_project();
            self.projects = TableItems::<Project>::new();
        }
    }
    fn close_popup(&mut self) {
        self.show_popup = !self.show_popup;
        self.window = Window {
            focus: WINDOW_PROJECTS.to_owned(),
            status: WindowStatus::NotLoaded,
        }
    }
    fn popup_edit(&mut self) {
        self.show_popup = !self.show_popup;
        if self.show_popup {
            self.window.focus = WINDOW_POPUP_EDIT.to_owned();
        } else {
            self.window.focus = WINDOW_PROJECTS.to_owned();
        }
    }
    fn popup_desc(&mut self) {
        self.show_popup = !self.show_popup;
        if self.show_popup {
            self.window = Window {
                focus: WINDOW_POPUP_DESC.to_owned(),
                status: WindowStatus::NotLoaded,
            }
        } else {
            self.window = Window {
                focus: WINDOW_PROJECTS.to_owned(),
                status: WindowStatus::NotLoaded,
            }
        }
    }
    // TODO we need to track the previous
    fn popup_add_task(&mut self) {
        let idx = self.projects.get_state_selected().unwrap();
        let project = &self.projects.items.iter().nth(idx);
        match project {
            Some(p) => {
                self.show_popup = !self.show_popup;
                if self.show_popup {
                    self.window.focus = WINDOW_POPUP_ADD_TODO.to_owned()
                }
            }
            None => self.message = "Add a project first".to_owned(),
        }
    }
    fn popup_task_write_and_close(&mut self, todo: String) {
        self.window.focus = WINDOW_TODO.to_owned();
        let idx = self.projects.get_state_selected().unwrap();
        let project = &self.projects.items.iter().nth(idx);
        match project {
            Some(p) => {
                write_new_todo(vec![Todo {
                    id: 0,
                    parent_id: 0,
                    project_id: p.id,
                    todo: todo,
                    is_complete: false,
                }]);
                // TODO I want to simplify this...
                self.todos = FilteredListItems::<Todo>::new();
                self.todos.select_filter_state(Some(0), p.id);
                self.todo_sort();
            }
            None => (),
        }
    }
    fn popup_desc_write_and_close(&mut self, desc: String) {
        self.window = Window {
            focus: WINDOW_PROJECTS.to_owned(),
            status: WindowStatus::NotLoaded,
        };
        let idx = self.projects.get_state_selected().unwrap();
        let project = &self.projects.items.iter().nth(idx);
        match project {
            Some(p) => {
                update_project_desc(p, desc).expect("A");
                // reload projects but retain selection
                self.projects = TableItems::<Project>::new();
                self.projects.state.select(Some(idx));
            }
            None => (),
        }
    }
    fn todo_sort(&mut self) {
        self.todos.sort_by_complete()
    }
    fn default_select(&mut self) {
        self.projects.state.select(Some(0));
        self.configs.state.select(Some(0));

        let idx = self.projects.get_state_selected().unwrap();
        let project = &self.projects.items.iter().nth(idx);
        match project {
            Some(p) => self.todos.select_filter_state(Some(0), p.id),
            None => (),
        }
    }
    fn toggle(&mut self) {
        match self.window.focus.as_str() {
            WINDOW_POPUP_CONFIGS => self.configs.toggle(),
            WINDOW_PROJECTS => self.projects.toggle(),
            WINDOW_TODO => self.todos.toggle(),
            _ => self.configs.toggle(),
        }
    }
    fn cycle_focus_next(&mut self) {
        self.window.focus = match self.window.focus.clone().as_str() {
            // WINDOW_POPUP_CONFIGS => WINDOW_POPUP_CONFIGS.to_owned(),
            WINDOW_PROJECTS => WINDOW_TODO.to_owned(),
            WINDOW_TODO => WINDOW_DESCRIPTION.to_owned(),
            WINDOW_DESCRIPTION => WINDOW_PROJECTS.to_owned(),
            _ => self.window.focus.clone(),
        }
    }
    fn cycle_focus_previous(&mut self) {
        self.window.focus = match self.window.focus.clone().as_str() {
            // WINDOW_POPUP_CONFIGS => WINDOW_POPUP_CONFIGS.to_owned(),
            WINDOW_PROJECTS => WINDOW_DESCRIPTION.to_owned(),
            WINDOW_DESCRIPTION => WINDOW_TODO.to_owned(),
            WINDOW_TODO => WINDOW_PROJECTS.to_owned(),
            _ => self.window.focus.clone(),
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

///
pub fn viewer() -> Result<(), Box<dyn std::error::Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // App
    let app = App::new();
    let _res = run_app(&mut terminal, app);

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

fn ui_popup<B: Backend>(rect: &mut Frame<B>, textarea: &mut TextArea, app: &mut App) {
    if app.show_popup {
        let idx = app.projects.get_state_selected().unwrap();
        let project = &app.projects.items.iter().nth(idx);

        match app.window.focus.as_str() {
            WINDOW_POPUP_ADD_TODO => match project {
                Some(p) => {
                    let size = rect.size();
                    let area = centered_rect(40, 40, size);

                    rect.render_widget(Clear, area); //s

                    textarea.set_block(
                        Block::default()
                            .borders(Borders::ALL)
                            .style(Style::default().fg(Color::Yellow))
                            .title(format!("Add task for {}", p.id)),
                    );
                    let widget = textarea.widget();
                    rect.render_widget(widget, area);
                }
                None => (),
            },
            WINDOW_POPUP_DESC => match project {
                Some(p) => {
                    let size = rect.size();
                    let area = centered_rect(40, 40, size);
                    rect.render_widget(Clear, area); //s

                    match app.window.status {
                        WindowStatus::NotLoaded => {
                            textarea.insert_str(p.desc.as_str());
                            app.window.status = WindowStatus::Loaded;
                        }
                        _ => (),
                    }

                    // textarea.insert_str(p.desc.as_str());
                    textarea.set_block(
                        Block::default()
                            .borders(Borders::ALL)
                            .style(Style::default().fg(Color::Yellow))
                            .title(format!("Desc {}", p.id)),
                    );
                    let widget = textarea.widget();
                    rect.render_widget(widget, area);
                }
                None => (),
            },
            WINDOW_POPUP_EDIT => match project {
                Some(p) => {
                    let size = rect.size();
                    let area = centered_rect(40, 40, size);
                    rect.render_widget(Clear, area); //s

                    textarea.set_block(
                        Block::default()
                            .borders(Borders::ALL)
                            .style(Style::default().fg(Color::Yellow))
                            .title(format!("Edit {}", p.id)),
                    );
                    let widget = textarea.widget();
                    rect.render_widget(widget, area);
                }
                None => (),
            },
            _ => (),
        }
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    // select
    app.default_select();

    let mut textarea = TextArea::default();
    // draw
    loop {
        terminal.draw(|rect| {
            ui(rect, &mut app);
            // TODO todos want date column
            ui_popup(rect, &mut textarea, &mut app);
        })?;

        match app.window.focus.as_str() {
            // TODO write without committing...
            WINDOW_POPUP_ADD_TODO => match crossterm::event::read()?.into() {
                Input { key: Key::Esc, .. } => {
                    app.popup_task_write_and_close(textarea.lines().join("\n").to_owned());
                    textarea = TextArea::default();
                }
                Input {
                    key: Key::Char('`'),
                    ..
                } => {
                    app.close_popup();
                    textarea = TextArea::default();
                }
                Input {
                    key: Key::Enter, ..
                } => {}
                input => {
                    textarea.input(input);
                }
            },
            WINDOW_POPUP_EDIT => match crossterm::event::read()?.into() {
                Input { key: Key::Esc, .. } => {
                    app.popup_task_write_and_close(textarea.lines().join("\n").to_owned());
                    textarea = TextArea::default();
                }
                Input {
                    key: Key::Char('`'),
                    ..
                } => {
                    app.close_popup();
                    textarea = TextArea::default();
                }
                Input {
                    key: Key::Enter, ..
                } => {}
                input => {
                    textarea.input(input);
                }
            },
            WINDOW_POPUP_DESC => match crossterm::event::read()?.into() {
                Input { key: Key::Esc, .. } => {
                    app.popup_desc_write_and_close(textarea.lines().join("\n").to_owned());
                    textarea = TextArea::default();
                }
                Input {
                    key: Key::Char('`'),
                    ..
                } => {
                    app.close_popup();
                    textarea = TextArea::default();
                }
                Input {
                    key: Key::Enter, ..
                } => {}
                input => {
                    textarea.input(input);
                }
            },
            _ => {
                if let Event::Key(key) = event::read().expect("Key error") {
                    match key.code {
                        KeyCode::Char('q') => {
                            if app.window.focus == WINDOW_POPUP_CONFIGS {
                                app.show_popup = false;
                                app.window.focus = WINDOW_PROJECTS.to_owned();
                            } else {
                                return Ok(());
                            }
                        }
                        KeyCode::Char('e') => app.popup_edit(),
                        KeyCode::Char('r') => app.popup_desc(),
                        // TODO add projects in current directory
                        KeyCode::Char('p') => app.popup(),
                        // TODO help box
                        KeyCode::Char('h') => {}
                        // KeyCode::Char('r') => app.reload(),
                        KeyCode::Char('a') => app.popup_add_task(),
                        KeyCode::Char(';') | KeyCode::Right => app.cycle_focus_next(),
                        KeyCode::Char('j') | KeyCode::Left => app.cycle_focus_previous(),
                        KeyCode::Char('l') | KeyCode::Down => app.next(),
                        KeyCode::Char('k') | KeyCode::Up => app.previous(),
                        KeyCode::Enter => app.toggle(),
                        _ => {}
                    }
                }
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
                // todo list and description
                Constraint::Percentage(40),
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
        let no_projects = render_no_projects();
        rect.render_widget(no_projects, chunks[1]);
    } else {
        let projects = render_projects(&app);
        rect.render_stateful_widget(projects, chunks[1], &mut app.projects.state);
    }

    // chunk 2: todo list
    // TODO do we need to specify percentages if they are uniform?
    let todo_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[2]);

    let (left_todo_list, right_todo_search) = render_todo(&app);
    rect.render_stateful_widget(left_todo_list, todo_chunks[0], &mut app.todos.state);
    rect.render_widget(right_todo_search, todo_chunks[1]);

    // popup
    // which popup is decided here
    if app.show_popup && app.window.focus == WINDOW_POPUP_CONFIGS.to_owned() {
        // TODO fuzzy find
        let area = centered_rect(80, 40, size);
        rect.render_widget(Clear, area); //this clears out the background

        // which popup is decided here
        let popup = render_config_paths(&app);
        rect.render_stateful_widget(popup, area, &mut app.configs.state);
    };
}

// TODO remove app reference. not needed
fn render_no_projects<'a>() -> Paragraph<'a> {
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
                Cell::from(
                    p.name
                        .replace(SEARCH_DIRECTORY_PREFIX, "...")
                        .replace("/.git/config", "") // TODO into constant
                        .clone(),
                ),
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
                .title("(projects)")
                .borders(Borders::ALL)
                .style(Style::default().fg(if app.window.focus == WINDOW_PROJECTS {
                    Color::Yellow
                } else {
                    Color::White
                }))
                .border_type(BorderType::Plain),
        )
        .header(Row::new(vec!["Name", "Cat", "Status", "Last Commit"]))
        .widths(&[
            Constraint::Length(50),
            // Constraint::Length(40),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(20),
        ])
        .highlight_style(
            Style::default()
                .bg(if app.window.focus == WINDOW_PROJECTS {
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

// add task

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

fn render_todo<'a>(app: &App) -> (List<'a>, Paragraph<'a>) {
    let todo_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(
            // TODO this should be a rule
            if app.window.focus == WINDOW_TODO {
                Color::Yellow
            } else {
                Color::White
            },
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
            .bg(if app.window.focus == WINDOW_TODO {
                Color::Yellow
            } else {
                Color::Gray
            })
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    let idx = app.projects.get_state_selected().unwrap();
    let project_desc = match app.projects.items.iter().nth(idx) {
        Some(p) => p.desc.to_owned(),
        None => "".to_owned(),
    };

    let right = Paragraph::new(project_desc)
        .block(Block::default().title("Paragraph").borders(Borders::ALL))
        .style(
            Style::default()
                .fg(if app.window.focus == WINDOW_DESCRIPTION {
                    Color::Yellow
                } else {
                    Color::Gray
                })
                .bg(Color::Black),
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });

    (left, right)
}
