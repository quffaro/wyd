// TODO todos want date column
use crate::refactor::new_lib::{
    App, BaseWindow, ListNavigate, Mode, PopupWindow, Window, WindowStatus,
};
use crate::refactor::new_lib::{HIGHLIGHT_SYMBOL, SEARCH_DIRECTORY_PREFIX};
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

pub fn viewer() -> Result<(), Box<dyn std::error::Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // App
    let app = App::init();
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

    let mut textarea = TextArea::default();

    loop {
        terminal.draw(|rect| {
            ui(rect, &mut app);
            ui_popup(rect, &mut textarea, &mut app);
        })?;

        // app.input(&mut textarea);
        match app.window {
            Window {
                popup: PopupWindow::None,
                base: _,
                ..
            } => {
                if let Event::Key(key) = event::read().expect("Key Error") {
                    match key.code {
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        KeyCode::Char('a') => app.add_project_in_dir(),
                        KeyCode::Char('d') => app.delete_todo(),
                        KeyCode::Char('e') => app.popup(PopupWindow::EditDesc),
                        KeyCode::Char('r') => app.popup(PopupWindow::EditCategory),
                        KeyCode::Char('t') => app.popup(PopupWindow::AddTodo),
                        KeyCode::Char(';') | KeyCode::Right => app.cycle_focus_next(),
                        KeyCode::Char('j') | KeyCode::Left => app.cycle_focus_previous(),
                        KeyCode::Char('l') | KeyCode::Down => app.next(),
                        KeyCode::Char('k') | KeyCode::Up => app.previous(),
                        KeyCode::Enter => app.toggle(),
                        _ => {}
                    }
                }
            }
            Window {
                popup: ref popup,
                base: _,
                ..
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

    let title = Paragraph::new(format!("{:#?} {:#?}", app.window.popup, app.window.mode))
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
                        .title(format!("Add task for {}", p.id)),
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
        PopupWindow::EditCategory => match project {
            Some(_) => {
                let size = rect.size();
                let area = centered_rect(40, 40, size);
                rect.render_widget(Clear, area);

                let category_block = Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(app.window.mode_color()))
                    .title("(todo)")
                    .border_type(BorderType::Plain);

                let categories: Vec<ListItem> = app
                    .categories
                    .items
                    .iter()
                    .map(|t| ListItem::new(format!("{}", t)))
                    .collect();

                let category_list = List::new(categories)
                    .block(category_block)
                    .highlight_style(
                        Style::default()
                            .fg(app.window.mode_color())
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol(HIGHLIGHT_SYMBOL);

                rect.render_stateful_widget(category_list, area, &mut app.categories.state);
            }
            None => (),
        },
        _ => (),
    }
}

fn render_no_projects<'a>(app: &App) -> Paragraph<'a> {
    let msg = "\n\n\n\n\n\n\n(press `p` to search for git configs)".to_owned();
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
                .title("(projects)")
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
                Cell::from(p.category.to_string().clone()),
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
            Constraint::Length(50),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(20),
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
                .style(
                    Style::default()
                        .fg(app.window.mode_color())
                        .bg(Color::Black),
                )
                .border_type(BorderType::Plain),
        )
        .widths(&[Constraint::Length(50), Constraint::Length(20)])
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
