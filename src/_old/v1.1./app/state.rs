use super::structs::windows::Window;

#[derive(Clone)]
pub enum AppState {
    Init,
    Initialized { window: Window },
}

impl AppState {
    pub fn initialized() -> Self {
        let window = Window::new(true);
        Self::Initialized { window }
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
