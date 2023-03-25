use std::io::stdout;
use std::sync::Arc;
use std::time::Duration;

use app::{App, AppReturn};
use const_format::formatcp;
use dirs::home_dir;
use eyre::Result;
use inputs::events::Events;
use inputs::InputEvent;
use io::IoEvent;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::app::ui;

pub mod app;
pub mod inputs;
pub mod io;
pub mod sql;

pub const CONFIG: &str = "/.config/wyd/";
pub const PAT: &str = "pat.txt";
pub const DB: &str = "wyd.db";
pub const PATH_PAT: &str = formatcp!("{}{}", CONFIG, PAT);
pub const PATH_DB: &str = formatcp!("{}{}", CONFIG, DB);

pub const GITCONFIG_SUFFIX: &str = ".git/config";
pub const GLOB_GITCONFIG_SUFFIX: &str = formatcp!("**/{}", GITCONFIG_SUFFIX);
pub const CONFIG_SEARCH_FOLDER: &str = "/Documents/";

pub fn home_path(path: &str) -> String {
    format!(
        "{}{}",
        home_dir().unwrap().into_os_string().into_string().unwrap(),
        path.to_owned()
    )
}

pub async fn start_ui(app: &Arc<tokio::sync::Mutex<App<'_>>>) -> Result<()> {
    let stdout = stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    let tick_rate = Duration::from_millis(200);
    let mut events = Events::new(tick_rate);

    {
        let mut app = app.lock().await;
        app.dispatch(IoEvent::Initialize).await;
    }

    loop {
        let mut app = app.lock().await;

        terminal.draw(|rect| ui::draw(rect, &app))?;

        let result = match events.next().await {
            InputEvent::Input(key) => app.do_action(key).await,
            InputEvent::Nothing => app.do_nothing().await,
        };

        if result == AppReturn::Exit {
            events.close();
            break;
        }
    }

    terminal.clear()?;
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
