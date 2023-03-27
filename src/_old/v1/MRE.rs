use std::thread::{self, sleep, current};
use std::time;
use std::sync::mpsc::{self, TryRecvError};


struct App {
    msg: String,
    counter: u64
}

impl App {
    fn new() -> App {
        App {
            msg: "Waiting...".to_owned(),
            counter: 0
        }
    }
    fn tick(&mut self) {
        self.counter += 1;
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        thread::sleep(time::Duration::from_secs(10));
        tx.send(true).unwrap()
    });
    
    loop {
        match rx.try_recv() {
            Err(TryRecvError::Empty) => app.on_tick(),
            Ok(_) | Err(_) => {app.message = app.counter.to_string();}
        }
        terminal.draw(|rect| ui(rect, &mut app))?;
    }
}


fn ui<B: Backend>(rect: &mut Frame<B>, app: &mut App) {
    let size = rect.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(03),
            ]
            .as_ref(),
        )
        .split(size);

    let anim = Paragraph::new(match app.counter % 5 {
        0 => "⠾",
        1 => "⠽",
        2 => "⠻",
        3 => "⠯",
        _ => "⠷",
    });

    rect.render_widget(anim, chunks[0]);
}

