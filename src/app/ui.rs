use ratatui::backend::Backend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Span, Spans};
use ratatui::widgets::{Block, BorderType, Borders, Cell, LineGauge, Paragraph, Row, Table};
use ratatui::{symbols, Frame};

use super::actions::Actions;
use super::state::AppState;
use crate::app::App;

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

    // Title
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

    // PROJECTS
    // rect.render_stateful_widget(app, chunks[1]);
}

// fn ui<B: Backend>(rect: &mut Frame<B>, app: &mut App) {
//     let size = rect.size();
//     let chunks = Layout::default()
//         .direction(Direction::Vertical)
//         .margin(2)
//         .constraints(
//             [
//                 // greeting
//                 Constraint::Length(03),
//                 // table
//                 // Constraint::Percentage(50),
//                 // todo list and description
//                 // Constraint::Percentage(40),
//                 // status bar
//                 // Constraint::Length(1),
//             ]
//             .as_ref(),
//         )
//         .split(size);

//     // TODO wrap this into a rule
//     let pwd = current_dir()
//         .unwrap()
//         .into_os_string()
//         .into_string()
//         .unwrap();

//     // chunk 0: title
//     rect.render_widget(title, chunks[0]);

//     // chunk 1: projects
//     if app.projects.items.len() == 0 {
//         let no_projects = render_no_projects(&app);
//         rect.render_widget(no_projects, chunks[1]);
//     } else {
//         let projects = render_projects(&app);
//         rect.render_stateful_widget(projects, chunks[1], &mut app.projects.state);
//     }

//     // chunk 2: todo list
//     // TODO do we need to specify percentages if they are uniform?
//     let todo_chunks = Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
//         .split(chunks[2]);

//     let (left_todo_list, right_todo_search) = render_todo_and_desc(&app);
//     rect.render_stateful_widget(left_todo_list, todo_chunks[0], &mut app.todos.state);
//     rect.render_widget(right_todo_search, todo_chunks[1]);

//     // chunk 3 status
//     // let mut counter = Counter::new();

//     // let throb = Paragraph::new(match app.throbber {
//     //     LoadingState {
//     //         status: WindowStatus::NotLoaded,
//     //         count: x,
//     //     } => match x % 5 {
//     //         0 => "⠾",
//     //         1 => "⠽",
//     //         2 => "⠻",
//     //         3 => "⠯",
//     //         _ => "⠷",
//     //     },
//     //     _ => "LOADED!",
//     // });

//     let full = throbber_widgets_tui::Throbber::default()
//         .label("Loading commits from Github...")
//         .style(tui::style::Style::default().fg(tui::style::Color::Cyan))
//         .throbber_style(
//             tui::style::Style::default()
//                 .fg(tui::style::Color::Red)
//                 .add_modifier(tui::style::Modifier::BOLD),
//         )
//         .use_type(throbber_widgets_tui::WhichUse::Spin);

//     let done = Paragraph::new("Done!").style(Style::default().fg(tui::style::Color::Cyan));

//     match app.throbber.status {
//         WindowStatus::NotLoaded => {
//             rect.render_stateful_widget(full, chunks[3], &mut app.throbber.throb)
//         }
//         WindowStatus::Loaded => rect.render_widget(done, chunks[3]),
//     }

//     // rect.render_stateful_widget(throb, chunks[3], &mut app.throbber.count);
// }
fn render_projects<'a>(app: &App) -> Table<'a> {
    // let home_dir = home_path(SUB_HOME_FOLDER);
    // let rows: Vec<Row> = app
    //     .projects
    //     .items
    //     .iter()
    //     .map(|p| {
    //         Row::new(vec![
    //             Cell::from(
    //                 p.name
    //                     .replace(&home_dir, "...")
    //                     .replace(SUBPATH_GIT_CONFIG, "") // TODO into constant
    //                     .clone(),
    //             ),
    //             Cell::from(p.category.to_string().clone()),
    //             Cell::from(p.status.to_string().clone()),
    //             Cell::from(p.last_commit.to_string().clone()),
    //     })
    //     .collect();

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
