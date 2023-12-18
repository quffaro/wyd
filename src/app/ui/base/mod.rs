use crate::app::structs::{
    // config::{Config, WydColor},
    focus::{Mode, WindowBase, WindowPopup},
    ListNav,
};
use crate::app::ui::wyd_to_color;
use crate::app::App;
use crate::{home_path, CONFIG_SEARCH_FOLDER, GITCONFIG_SUFFIX};
use ratatui::backend::Backend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::terminal::Frame;
use ratatui::widgets::{
    Block, BorderType, Borders, Cell, Clear, List, ListItem, Paragraph, Row, Table, Wrap,
};
use std::env;

pub fn render_title_and_search<'a>(app: &App) -> (Paragraph, Paragraph) {
    let left = render_search(app);

    let right = render_title(app);

    (left, right)
}

pub fn render_search<'a>(app: &App) -> Paragraph {
    let search = match app.focus.base {
        WindowBase::Search => app.input.value(),
        _ => "",
    };
    Paragraph::new(search)
        .block(
            Block::default()
                .title("(search)")
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL), // .border_type(BorderType::Rounded),
        )
        .style(
            Style::default()
                .fg(
                    if app.focus.base == WindowBase::Search && app.focus.popup == WindowPopup::None
                    {
                        Color::Yellow
                    } else {
                        Color::White
                    },
                )
                .bg(Color::Black)
                .add_modifier(Modifier::ITALIC),
        )
        .alignment(Alignment::Left)
}

pub fn render_title<'a>(app: &App) -> Paragraph {
    let path = env::current_dir().unwrap().display().to_string();
    Paragraph::new(path)
        .block(
            Block::default()
                .title("(wyd?)")
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL), // .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow).bg(Color::Black))
        .alignment(Alignment::Left)
}

pub fn render_projects<'a>(app: &App) -> Table<'a> {
    let home_dir = home_path(CONFIG_SEARCH_FOLDER);
    let rows: Vec<Row> = app
        .data
        .items
        .iter()
        // .filter(|s| s.name.contains(app.input.value()))
        .map(|p| {
            Row::new(vec![
                Cell::from(
                    p.name
                        .replace(&home_dir, "...")
                        .replace(GITCONFIG_SUFFIX, "") // TODO into constant
                        .clone(),
                ),
                // Cell::from(p.category.to_string().clone()),
                // Cell::from(p.status.to_string().clone()),
                // Cell::from(p.last_commit.to_string().clone()),
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
                        .fg(
                            if app.focus.base == WindowBase::Items
                                && app.focus.popup == WindowPopup::None
                            {
                                Color::Yellow
                            } else {
                                Color::White
                            },
                        )
                        .bg(Color::Black),
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
                .bg(if app.focus.popup == WindowPopup::None {
                    if app.focus.mode == Mode::Normal {
                        Color::Yellow
                    } else {
                        Color::Magenta
                    }
                } else {
                    Color::White
                })
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    projects
}

pub fn render_todo<'a>(app: &App) -> Table<'a> {
    let rows: Vec<Row> = vec![];
    // app
    // .data
    // // .todos
    // // .filtered
    // .iter()
    // .map(|t| {
    //     Row::new(vec![
    //         Cell::from(match t.is_complete {
    //             true => format!("[x] {}", t.todo.clone()),
    //             false => format!("[ ] {}", t.todo.clone()),
    //         }),
    //         Cell::from(t.priority.to_string()),
    //     ])
    // })
    // .collect();

    // dbg!(&rows);
    let todos = Table::new(rows)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .title("(todo)")
                .borders(Borders::ALL)
                .style(
                    Style::default()
                        .fg(
                            if app.focus.base == WindowBase::Todo
                                && app.focus.popup == WindowPopup::None
                            {
                                Color::Yellow
                                // app.config
                                //     .clone()
                                //     .and_then(|c| Some(wyd_to_color(c.color.bd)))
                                //     .unwrap()
                            } else {
                                Color::White
                            },
                        )
                        .bg(Color::Black),
                )
                .border_type(BorderType::Plain),
        )
        .header(Row::new(vec!["Name", ""]))
        .widths(&[Constraint::Percentage(90), Constraint::Percentage(10)])
        .highlight_style(
            Style::default()
                .bg(if app.focus.popup == WindowPopup::None {
                    Color::Yellow
                } else {
                    Color::White
                })
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    todos
}

pub fn render_todo_and_desc<'a>(app: &App) -> (List<'a>, Paragraph<'a>) {
    let todo_block = Block::default()
        .borders(Borders::ALL)
        .style(
            Style::default()
                .fg(if app.focus.base == WindowBase::Todo {
                    Color::Yellow
                } else {
                    Color::White
                })
                .bg(Color::Black),
        )
        .title("(todo)")
        .border_type(BorderType::Plain);

    let todo_items: Vec<ListItem> = vec![];
    // app
    // .data
    // .items
    // .iter()
    // .map(|t| {
    //     if t.is_complete {
    //         ListItem::new(format!("[x] {}", t.todo.clone()))
    //     } else {
    //         ListItem::new(format!("[ ] {}", t.todo.clone()))
    //     }
    // })
    // .collect();

    let left = List::new(todo_items).block(todo_block).highlight_style(
        Style::default()
            .fg(if app.focus.base == WindowBase::Todo {
                Color::Yellow
            } else {
                Color::White
            })
            .bg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    let project_desc = match app.data.get_state_selected() {
        Some(i) => match app.data.items.iter().nth(i) {
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
                .fg(if app.focus.base == WindowBase::Description {
                    Color::Yellow
                } else {
                    Color::White
                })
                .bg(Color::Black),
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });

    (left, right)
}
