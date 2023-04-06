#[derive(Debug, Clone, Copy)]
pub struct JobRoster {
    pub gitcommit: LoadingState,
    pub gitconfig: LoadingState,
}

impl JobRoster {
    pub fn new() -> Self {
        Self {
            gitcommit: LoadingState::Loading,
            gitconfig: LoadingState::Loading,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LoadingState {
    Loading,
    Finished,
}
