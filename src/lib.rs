use std::io::stdout;
use std::sync::Arc;
use std::time::Duration;

use app::{App, AppReturn};
use inout::IoEvent;
use inputs::events::Events;
use inputs::InputEvent;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::app::ui;

pub mod app;
pub mod inout;
pub mod inputs;

pub async fn ui(app: &Arc<tokio::sync::Mutex<App>>) -> Result<(), Box<dyn std::error::Error>> {
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
            InputEvent::Tick => app.update_on_tick().await,
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
