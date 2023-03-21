// TODO todos want date column
// TODO shift-a for git repo
// TODO SeaORM
// TODO Oso
// TODO ui folder
use crate::library::code::{
    App, BaseWindow, ListNavigate, Mode, PopupWindow, Window, WindowStatus, HIGHLIGHT_SYMBOL,
    SEARCH_DIRECTORY_PREFIX, SUBPATH_GIT_CONFIG, SUB_HOME_FOLDER, DATABASE,
};
use crate::library::request::request_string;
use crate::library::sql::init_tmp_git_config;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use dirs::home_dir;
use rusqlite::Connection;
use std::env::current_dir;
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
// use tokio::task;
use std::thread::{self, current};

pub fn viewer() -> Result<(), Box<dyn std::error::Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // App
    let conn = Connection::open(DATABASE).unwrap();
    let app = App::load(&conn);
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    // select
    app.default_select();

    // TODO passing the same connection is unsafe.
    thread::spawn(|| init_tmp_git_config());
    thread::spawn(|| request_string());

    let mut textarea = TextArea::default();

    loop {
        terminal.draw(|rect| {
            ui(rect, &mut app);
            ui_popup(rect, &mut textarea, &mut app);
        })?;

        // app.input(&mut textarea);
        // TODO i would like for this to be in its own rule
        match app.window {
            Window {
                popup: PopupWindow::None,
                base: _,
                ..
            } => match crossterm::event::read().expect("MAIN CAPTURE ERROR").into() {
                Input {
                    key: Key::Char('q'),
                    ..
                } => return Ok(()),
                Input {
                    key: Key::Char('h'),
                    ..
                } => app.popup(PopupWindow::Help, Some(Mode::Normal)),
                Input {
                    key: Key::Char('a'),
                    ..
                } => app.add_project_in_dir(false),
                Input {
                    key: Key::Char('A'),
                    ..
                } => app.add_project_in_dir(true),
                Input {
                    key: Key::Char('d'),
                    ..
                } => app.delete_todo(),
                Input {
                    key: Key::Char('e'),
                    ..
                } => app.popup(PopupWindow::EditDesc, Some(Mode::Insert)),
                Input {
                    key: Key::Char('r'),
                    ..
                } => app.popup(PopupWindow::EditCategory, Some(Mode::Normal)),
                Input {
                    key: Key::Char('R'),
                    ..
                } => app.popup(PopupWindow::NewCategory, Some(Mode::Insert)),
                Input {
                    key: Key::Char('t'),
                    ..
                } => app.popup(PopupWindow::AddTodo, Some(Mode::Insert)),
                Input {
                    key: Key::Char('p'),
                    ..
                } => app.popup(PopupWindow::SearchGitConfig, Some(Mode::Normal)),
                Input {
                    key: Key::Enter, ..
                } => app.toggle(),
                Input {
                    key: Key::Char(';'),
                    ..
                }
                | Input {
                    key: Key::Right, ..
                } => app.cycle_focus_next(),
                Input {
                    key: Key::Char('j'),
                    ..
                }
                | Input { key: Key::Left, .. } => app.cycle_focus_previous(),
                Input {
                    key: Key::Char('l'),
                    ..
                }
                | Input { key: Key::Up, .. }
                | Input {
                    key: Key::MouseScrollUp,
                    ..
                } => app.previous(),
                Input {
                    key: Key::Char('k'),
                    ..
                }
                | Input { key: Key::Down, .. }
                | Input {
                    key: Key::MouseScrollDown,
                    ..
                } => app.next(),
                _ => {}
            },
            Window {
                ref popup, base: _, ..
            } => match app.window.mode {
                Mode::Insert => app.popup_mode_insert(&mut textarea),
                Mode::Normal => app.popup_mode_normal(&mut textarea, popup.clone()),
            },
            _ => {}
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
                // todo list and description
                Constraint::Percentage(40),
            ]
            .as_ref(),
        )
        .split(size);

    // TODO wrap this into a rule
    let pwd = current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    let title = Paragraph::new(pwd)
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
    // TODO do we need to specify percentages if they are uniform?
    let todo_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[2]);

    let (left_todo_list, right_todo_search) = render_todo_and_desc(&app);
    rect.render_stateful_widget(left_todo_list, todo_chunks[0], &mut app.todos.state);
    rect.render_widget(right_todo_search, todo_chunks[1]);
}

fn ui_popup<B: Backend>(rect: &mut Frame<B>, textarea: &mut TextArea, app: &mut App) {
    let project = app.projects.current();

    // POPUP
    match app.window.popup {
        PopupWindow::AddTodo => match project {
            Some(p) => {
                let size = rect.size();
                let area = centered_rect(40, 40, size);
                rect.render_widget(Clear, area);

                textarea.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(app.window.mode_color()))
                        .title(format!("Add task for {}", p.name)),
                );
                let widget = textarea.widget();
                rect.render_widget(widget, area);
            }
            None => {
                let size = rect.size();
                let area = centered_rect(40, 40, size);
                rect.render_widget(Clear, area);

                let msg = Paragraph::new("No project selected".to_owned());
                rect.render_widget(msg, area);
            }
        },
        PopupWindow::EditDesc => match project {
            Some(p) => {
                let size = rect.size();
                let area = centered_rect(40, 40, size);
                rect.render_widget(Clear, area);

                match app.window.status {
                    WindowStatus::NotLoaded => {
                        textarea.insert_str(p.desc.as_str());
                        app.window.status = WindowStatus::Loaded;
                    }
                    _ => (),
                }

                textarea.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(app.window.mode_color()))
                        .title(format!("Description for {}", p.id)),
                );
                let widget = textarea.widget();
                rect.render_widget(widget, area);
            }
            None => {
                let size = rect.size();
                let area = centered_rect(40, 40, size);
                rect.render_widget(Clear, area);

                let msg = Paragraph::new("No project selected".to_owned());
                rect.render_widget(msg, area);
            }
        },
        PopupWindow::NewCategory => match project {
            Some(p) => {
                let size = rect.size();
                let area = centered_rect(40, 10, size);
                rect.render_widget(Clear, area);

                textarea.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(app.window.mode_color()))
                        .title(format!("(new category for {})", p.id)),
                );
                let widget = textarea.widget();
                rect.render_widget(widget, area);
            }
            None => {
                let size = rect.size();
                let area = centered_rect(40, 40, size);
                rect.render_widget(Clear, area);

                let msg = Paragraph::new("No project selected".to_owned());
                rect.render_widget(msg, area);
            }
        }
        PopupWindow::EditCategory => match project {
            Some(_) => {
                let size = rect.size();
                let clear = centered_rect(40, 40, size);
                rect.render_widget(Clear, clear);

                let (area, input) = centered_rect_category(40, 40, size);

                let category_block = Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(app.window.mode_color()))
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
                    .highlight_symbol(HIGHLIGHT_SYMBOL);

                rect.render_stateful_widget(category_list, area, &mut app.categories.state);

                textarea.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(app.window.mode_color()))
                        .title(format!("(new) category")),
                );
                let widget = textarea.widget();
                rect.render_widget(widget, input);
            }
            None => (),
        },
        PopupWindow::SearchGitConfig => {
            // TODO performance

            let size = rect.size();
            let area = centered_rect(80, 40, size);
            rect.render_widget(Clear, area); //this clears out the background

            // which popup is decided here
            let popup = render_popup_config_paths(&app);
            rect.render_stateful_widget(popup, area, &mut app.configs.state);
        }
        PopupWindow::Help => {
            let size = rect.size();
            let area = centered_rect(60, 60, size);
            rect.render_widget(Clear, area);

            let popup = render_popup_help_table(app);
            rect.render_widget(popup, area);
        }
        _ => (),
    }
}

fn render_no_projects<'a>(app: &App) -> Paragraph<'a> {
    let msg = "".to_owned();
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
                // TODO implement a style rule
                .style(
                    Style::default().fg(if app.window.base == BaseWindow::Project {
                        Color::Yellow
                    } else {
                        Color::White
                    }),
                )
                .title("(press `p` to search for git configs)")
                .border_type(BorderType::Plain),
        );

    no_projects
}

fn render_popup_help_table<'a>(app: &App) -> Table<'a> {
    let rows: Vec<Row> = vec![
        Row::new(vec![Cell::from("h"), Cell::from("help")]),
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
                .style(
                    Style::default().fg(if app.window.popup == PopupWindow::Help {
                        Color::Yellow
                    } else {
                        Color::Gray
                    }),
                )
                .border_type(BorderType::Plain),
        )
        .header(Row::new(vec!["Key", "Desc"]))
        .widths(&[Constraint::Percentage(10), Constraint::Percentage(90)]);

    help
}

// render projects
fn render_projects<'a>(app: &App) -> Table<'a> {
    let home_dir = format!(
        "{}{}",
        home_dir().unwrap().into_os_string().into_string().unwrap(),
        SUB_HOME_FOLDER
    );
    let rows: Vec<Row> = app
        .projects
        .items
        .iter()
        .map(|p| {
            Row::new(vec![
                Cell::from(
                    p.name
                        .replace(&home_dir, "...")
                        .replace(SUBPATH_GIT_CONFIG, "") // TODO into constant
                        .clone(),
                ),
                Cell::from(p.category.to_string().clone()),
                Cell::from(p.status.to_string().clone()),
                Cell::from(p.last_commit.to_string().clone()),
                // Cell::from(format!(
                //     "{} {}:{}",
                //     iso8601::datetime(&p.last_commit).unwrap().date,
                //     iso8601::datetime(&p.last_commit).unwrap().time.hour,
                //     iso8601::datetime(&p.last_commit).unwrap().time.minute,
                // )),
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
                    Style::default().fg(if app.window.base == BaseWindow::Project {
                        Color::Yellow
                    } else {
                        Color::Gray
                    }),
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
                .bg(if app.window.base == BaseWindow::Project {
                    Color::Yellow
                } else {
                    Color::White
                })
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(HIGHLIGHT_SYMBOL);

    projects
}

fn render_todo_and_desc<'a>(app: &App) -> (List<'a>, Paragraph<'a>) {
    let todo_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(if app.window.base == BaseWindow::Todo {
            Color::Yellow
        } else {
            Color::White
        }))
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
            .bg(if app.window.base == BaseWindow::Todo {
                Color::Yellow
            } else {
                Color::White
            })
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
            Style::default().fg(if app.window.base == BaseWindow::Description {
                Color::Yellow
            } else {
                Color::White
            }),
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });

    (left, right)
}

fn render_popup_config_paths<'a>(app: &App) -> Table<'a> {
    // TODO simplify
    let home_dir = format!(
        "{}{}",
        home_dir().unwrap().into_os_string().into_string().unwrap(),
        SUB_HOME_FOLDER
    );
    let rows: Vec<Row> = app
        .configs
        .items
        .iter()
        .map(|p| {
            // TODO fix
            let y = p
                .path
                .replace(&home_dir, ".../")
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
        .highlight_symbol(HIGHLIGHT_SYMBOL);

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

fn centered_rect_category(percent_x: u16, percent_y: u16, r: Rect) -> (Rect,Rect) {
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
        .constraints(
            [Constraint::Percentage(85), Constraint::Length(15)]
        )
        .split(popup);

    (y[0], y[1])
}
