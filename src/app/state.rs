#[derive(Clone)]
pub enum AppState {
    Init,
    Initialized { count: u32 },
}

impl AppState {
    pub fn initialized() -> Self {
        let count = 0;
        Self::Initialized { count }
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self, &Self::Initialized { .. })
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::Init
    }
}
