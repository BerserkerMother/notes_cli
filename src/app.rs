use tui::widgets::ListState;

use crate::Note;

pub struct App {
    page_state: AppState,
    pub should_exist: bool,
    note_state: ListState,
    pub buffer: String,
    pub editor_mode: bool,
    pub notes: Option<Vec<Note>>,
}

impl App {
    pub fn new() -> App {
        let mut note_state_list = ListState::default();
        note_state_list.select(Some(0));
        App {
            page_state: AppState::Home,
            should_exist: false,
            note_state: note_state_list,
            buffer: String::new(),
            editor_mode: false,
            notes: None,
        }
    }
    pub fn state(&self) -> &AppState {
        &self.page_state
    }
    pub fn is_state(&self, state: AppState) -> bool {
        self.page_state == state
    }
    pub fn set_state(&mut self, app_state: AppState) {
        self.page_state = app_state;
    }
    pub fn note_state(&self) -> &ListState {
        &self.note_state
    }
    pub fn set_note_state(&mut self, state: usize) {
        self.note_state.select(Some(state))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AppState {
    Home,
    Note,
    Add,
    Delete,
}

impl From<AppState> for usize {
    fn from(value: AppState) -> Self {
        match value {
            AppState::Home => 0,
            AppState::Note => 1,
            AppState::Add => 2,
            AppState::Delete => 3,
        }
    }
}
