use crate::app::structs::{
    config::{Config, WydColor},
    windows::{BaseWindow, Mode, WindowStatus},
    ListNav,
};
use crate::app::ui::{centered_rect, centered_rect_category, centered_rect_config};
use crate::app::{App, Popup};
use crate::{home_path, CONFIG_SEARCH_FOLDER, GITCONFIG_SUFFIX};
use ratatui::backend::Backend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::terminal::Frame;
use ratatui::text::{Span, Spans};
use ratatui::widgets::{
    Block, BorderType, Borders, Cell, Clear, List, ListItem, Paragraph, Row, Table, Tabs, Wrap,
};
use std::env;

pub fn render_popup_new_project<'a, B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let size = frame.size();
    let area = centered_rect(40, 10, size);
    frame.render_widget(Clear, area);

    let (owner, directory) = centered_rect_config(40, 10, size);

    let width = area.width.max(3) - 3;
    let scroll = app.input.visual_scroll(width as usize);
    let text = Paragraph::new(app.input.value())
        .style(Style::default().fg(match app.window.mode {
            Mode::Insert => Color::Yellow,
            Mode::Normal => Color::Green,
        }))
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .title("(project name)")
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL),
        );

    frame.render_widget(text, area);
    match app.window.mode {
        Mode::Normal => {}
        Mode::Insert => frame.set_cursor(
            area.x + ((app.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
            area.y + 1,
        ),
    }
}

pub fn render_theme_picker<'a, B: Backend>(app: &App, frame: &mut Frame<'_, B>) {
    let size = frame.size();
    let area = centered_rect(40, 10, size);
    frame.render_widget(Clear, area);

    let colors: Vec<ListItem> = vec![WydColor::Yellow, WydColor::Green]
        .iter()
        .map(|t| ListItem::new(format!("{}", t)))
        .collect();

    // IF THERE ARE NONE
    let color_list = List::new(colors)
        .block(Block::default().title("(select highlight theme)"))
        .highlight_style(
            Style::default()
                .fg(app.window.mode_color())
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    // frame.render_stateful_widget(colors, area, todo!)
}

pub fn render_popup_delete_project<'a, B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let size = frame.size();
    let area = centered_rect(40, 10, size);
    frame.render_widget(Clear, area);

    let (owner, directory) = centered_rect_config(40, 10, size);

    // OWNER
    let text = Paragraph::new(format!(
        "Are you sure you want to delete {}?",
        app.projects.current().unwrap().name
    ))
    .wrap(Wrap { trim: false })
    .block(
        Block::default()
            .title("(are you sure?)")
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::LightRed)),
    );

    let choices = vec!["Yes", "No"]
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(first, Style::default().fg(Color::Yellow)),
                Span::styled(rest, Style::default().fg(Color::Green)),
            ])
        })
        .collect();
    let tabs = Tabs::new(choices)
        .block(Block::default().borders(Borders::ALL).title(format!(
            "(are you sure you want to delete {}?",
            app.projects.current().unwrap().name
        )))
        .select(app.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );
    // SEARCH DIRECTORY
    frame.render_widget(tabs, area);
}

// TODO fix spelling
pub fn render_popup_wyd_config<'a, B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let size = frame.size();
    let area = centered_rect(40, 10, size);
    frame.render_widget(Clear, area);

    let (owner, directory) = centered_rect_config(40, 10, size);

    // OWNER
    let width = area.width.max(3) - 3;
    let scroll = app.input.visual_scroll(width as usize);
    let text = Paragraph::new(app.input.value())
        .style(Style::default().fg(match app.window.mode {
            Mode::Insert => Color::Yellow,
            Mode::Normal => Color::Green,
        }))
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .title("(what is your gh username?)")
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL),
        );

    // SEARCH DIRECTORY
    frame.render_widget(text, area);
    match app.window.mode {
        Mode::Normal => {}
        Mode::Insert => frame.set_cursor(
            area.x + ((app.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
            area.y + 1,
        ),
    }
}

pub fn render_popup_new_cat<'a, B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    match app.projects.current() {
        Some(_) => {
            let size = frame.size();
            let area = centered_rect(40, 40, size);
            frame.render_widget(Clear, area);

            let (big, small) = centered_rect_category(40, 40, size);

            let category_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(if app.window.popup == Popup::EditCat {
                    Color::Yellow
                } else {
                    Color::Gray
                }))
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
                .highlight_symbol(">> ");

            frame.render_stateful_widget(category_list, big, &mut app.categories.state);

            let text = Paragraph::new(app.input.value())
                .style(
                    Style::default().fg(match (app.window.popup, app.window.mode) {
                        (Popup::NewCat, Mode::Insert) => Color::Yellow,
                        (Popup::NewCat, Mode::Normal) => Color::Green,
                        _ => Color::Gray,
                    }),
                )
                .wrap(Wrap { trim: false })
                .block(
                    Block::default()
                        .title("(new category)")
                        .title_alignment(Alignment::Left)
                        .borders(Borders::ALL),
                );

            frame.render_widget(text, small);
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

pub fn render_popup_help_table<'a, B: Backend>(app: &App, frame: &mut Frame<'_, B>) {
    let size = frame.size();
    let area = centered_rect(80, 40, size);
    frame.render_widget(Clear, area); //this clears out the background

    let rows: Vec<Row> = vec![
        Row::new(vec![Cell::from("h"), Cell::from(":)")]),
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
                .style(Style::default().fg(if app.window.popup == Popup::Help {
                    Color::Yellow
                } else {
                    Color::Gray
                }))
                .border_type(BorderType::Plain),
        )
        .header(Row::new(vec!["Key", "Desc"]))
        .widths(&[Constraint::Percentage(10), Constraint::Percentage(90)]);

    frame.render_widget(help, area)
}

pub fn render_popup_search_config<'a, B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // TODO performance

    let size = frame.size();
    let area = centered_rect(80, 40, size);
    frame.render_widget(Clear, area); //this clears out the background

    // which popup is decided here
    let popup = render_popup_config_paths(&app);
    frame.render_stateful_widget(popup, area, &mut app.configs.state);
}

fn render_popup_config_paths<'a>(app: &App) -> Table<'a> {
    // TODO simplify
    let rows: Vec<Row> = app
        .configs
        .items
        .iter()
        .map(|p| {
            // TODO fix
            let y = p
                .path
                // .replace(&home_path(), ".../")
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
        .highlight_symbol(" >>");

    paths
}

pub fn render_popup_edit_desc<'a, B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    match app.projects.current() {
        Some(p) => {
            let size = frame.size();
            let area = centered_rect(40, 40, size);
            frame.render_widget(Clear, area);

            match app.is_popup_loading {
                true => {
                    app.input = tui_input::Input::new(p.desc.clone());
                    app.is_popup_loading = false;
                }
                _ => {}
            }

            let width = area.width.max(3) - 3;
            let scroll = app.input.visual_scroll(width as usize);
            let text = Paragraph::new(app.input.value())
                .style(Style::default().fg(match app.window.mode {
                    Mode::Insert => Color::Yellow,
                    Mode::Normal => Color::Green,
                }))
                .wrap(Wrap { trim: false })
                .block(
                    Block::default()
                        .title("(edit description)")
                        .title_alignment(Alignment::Left)
                        .borders(Borders::ALL),
                );

            frame.render_widget(text, area);
            match app.window.mode {
                Mode::Normal => {}
                Mode::Insert => frame.set_cursor(
                    area.x + ((app.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
                    area.y + 1,
                ),
            }
        }
        None => {
            // let size = frame.size();
            // let area = centered_rect(40, 40, size);
            // frame.render_widget(Clear, area);

            // let msg = Paragraph::new("No project selected".to_owned());
            // frame.render_widget(msg, area);
        }
    }
}

pub fn render_popup_read_todo<'a, B: Backend>(app: &App, frame: &mut Frame<'_, B>) {
    match app.todos.current() {
        Some(t) => {
            let size = frame.size();
            let area = centered_rect(40, 40, size);
            frame.render_widget(Clear, area);
            let width = area.width.max(3) - 3;
            let scroll = app.input.visual_scroll(width as usize);

            let text = Paragraph::new(t.todo.clone())
                .style(Style::default().fg(Color::Yellow))
                .wrap(Wrap { trim: false })
                .block(
                    Block::default()
                        .title("(read todo)")
                        .title_alignment(Alignment::Left)
                        .borders(Borders::ALL),
                );

            frame.render_widget(text, area);
            // match app.window.mode {
            //     Mode::Normal => {}
            //     Mode::Insert => frame.set_cursor(
            //         area.x + ((app.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
            //         area.y + 1,
            //     ),
            // }
        }
        None => {
            // let size = frame.size();
            // let area = centered_rect(40, 40, size);
            // frame.render_widget(Clear, area);

            // let msg = Paragraph::new("No project selected".to_owned());
            // frame.render_widget(msg, area);
        }
    }
}
pub fn render_popup_todo<'a, B: Backend>(app: &App, frame: &mut Frame<'_, B>) {
    match app.projects.current() {
        Some(_) => {
            let size = frame.size();
            let area = centered_rect(40, 40, size);
            frame.render_widget(Clear, area);
            let width = area.width.max(3) - 3;
            let scroll = app.input.visual_scroll(width as usize);

            let text = Paragraph::new(app.input.value())
                .style(Style::default().fg(match app.window.mode {
                    Mode::Insert => Color::Yellow,
                    Mode::Normal => Color::Green,
                }))
                .wrap(Wrap { trim: false })
                .block(
                    Block::default()
                        .title("(add todo)")
                        .title_alignment(Alignment::Left)
                        .borders(Borders::ALL),
                );

            frame.render_widget(text, area);
            match app.window.mode {
                Mode::Normal => {}
                Mode::Insert => frame.set_cursor(
                    area.x + ((app.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
                    area.y + 1,
                ),
            }
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
