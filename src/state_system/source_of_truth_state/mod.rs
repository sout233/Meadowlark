pub mod app_state;
pub mod project_state;

pub use app_state::*;
pub use project_state::*;

/// The state of the app/project which is considered the "source of truth".
///
/// All other state is derived from this "source of truth" state.
///
/// Only the `StateSystem` struct is allowed to mutate this.
pub struct SourceOfTruthState {
    pub app: AppState,
    pub current_project: Option<ProjectState>,
}

impl SourceOfTruthState {
    pub fn test_project() -> Self {
        Self { app: AppState::new(), current_project: Some(ProjectState::test_project()) }
    }
}
