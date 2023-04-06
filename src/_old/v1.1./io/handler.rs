use std::sync::Arc;
use std::time::Duration;

use eyre::Result;

use super::IoEvent;
use crate::app::App;

use crate::app::structs::projects::Project;
use crate::app::structs::TableItems;
use crate::PATH_DB;
use rusqlite::Connection;

pub struct IoAsyncHandler {
    app: Arc<tokio::sync::Mutex<App>>,
    // TODO DATA SHOULD BE MOVED HERE
    projects: TableItems<Project>,
}

impl IoAsyncHandler {
    pub fn new(app: Arc<tokio::sync::Mutex<App>>) -> Self {
        let conn = &Connection::open(PATH_DB).unwrap();
        Self {
            app,
            projects: TableItems::<Project>::load(conn),
        }
    }

    pub async fn handle_io_event(&mut self, io_event: IoEvent) {
        let result = match io_event {
            IoEvent::Initialize => self.do_initialize().await,
        };

        // LOGGING

        let mut app = self.app.lock().await;
        app.loaded();
    }

    async fn do_initialize(&mut self) -> Result<()> {
        let mut app = self.app.lock().await;
        // TODO
        tokio::time::sleep(Duration::from_secs(1)).await;
        app.initialized();

        Ok(())
    }
}
