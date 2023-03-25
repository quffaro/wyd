use std::sync::Arc;
use std::time::Duration;

use eyre::Result;

use super::IoEvent;
use crate::app::App;

pub struct IoAsyncHandler<'a> {
    app: Arc<tokio::sync::Mutex<App<'a>>>,
}

impl IoAsyncHandler<'_> {
    pub fn new(app: Arc<tokio::sync::Mutex<App>>) -> Self {
        Self { app }
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
