use std::io::Stdout;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui::{backend::CrosstermBackend, layout::Rect, Frame};

use crate::{
    app::{App, AppState},
    editor_handler,
    render::{self, Render},
    repository::Repository,
    widgets::Widget,
    Note,
};

/// handles the app
pub struct AppHandler {
    app: App,
    db: Repository,
    widget: Widget,
}

impl AppHandler {
    pub fn new(app: App, db: Repository, size: Rect) -> AppHandler {
        let widget = Widget::new(size);
        AppHandler { app, db, widget }
    }
    fn set_notes(&mut self) {
        let notes = self.db.get_notes().expect("can't access db");
        if notes.len() == 0 {
            self.app.notes = None;
        } else {
            self.app.notes = Some(notes)
        };
    }
    pub fn should_exit(&self) -> bool {
        return self.app.should_exist;
    }
    pub fn is_editor_mode(&self) -> bool {
        return self.app.editor_mode;
    }
    pub fn handle_event(
        &mut self,
        event: Event<KeyEvent>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self.app.state() {
            AppState::Add => self.handle_adding(event)?,
            _ => self.handle_normal_input(event)?,
        };
        Ok(())
    }
    pub fn handle_edit(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let title = self.app.buffer.clone();
        self.app.buffer.clear();
        let text = editor_handler::edit_with_vim()?;
        let note = Note::new(None, title, text);
        self.db.add(vec![note])?;
        self.app.editor_mode = false;
        self.app.set_state(AppState::Note);
        Ok(())
    }
    fn handle_normal_input(
        &mut self,
        event: Event<KeyEvent>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match event {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    self.app.should_exist = true;
                }
                KeyCode::Char('h') => self.app.set_state(AppState::Home),
                KeyCode::Char('n') => {
                    self.app.set_state(AppState::Note);
                    self.set_notes()
                }
                KeyCode::Char('a') => self.app.set_state(AppState::Add),
                KeyCode::Char('d') => {
                    if self.app.is_state(AppState::Note) {
                        self.handle_delete()?;
                    }
                }
                KeyCode::Down => self.handle_down()?,
                KeyCode::Up => self.handle_up()?,
                // KeyCode::Char('p') => active_menu_item = MenuItem::Pets,
                _ => {}
            },
            Event::Tick => {}
        };
        Ok(())
    }
    fn handle_delete(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let note_id = self.app.note_state().selected().ok_or("no selected note")?;
        let note = &self.app.notes.as_ref().expect("this should not happen!")[note_id];
        self.db.delete(note.id.ok_or("this should not happened")?)?;
        // self.app.set_note_state(note_id);
        self.set_notes();
        Ok(())
    }
    fn handle_adding(&mut self, event: Event<KeyEvent>) -> Result<(), Box<dyn std::error::Error>> {
        match event {
            Event::Input(event) => match event.code {
                KeyCode::Char(c) => {
                    if event.modifiers.contains(KeyModifiers::CONTROL) {
                        // self.app.buffer.clear();
                        self.app.editor_mode = true;
                    } else {
                        self.app.buffer.push(c);
                    }
                }
                KeyCode::Backspace => {
                    self.app.buffer.pop();
                }
                // KeyCode::Enter => {
                //     self.app.buffer.push_str("\n");
                // }
                KeyCode::Esc => {
                    self.app.set_state(AppState::Home);
                    self.app.buffer.clear()
                }
                _ => (),
            },
            _ => (),
        };
        Ok(())
    }
    fn handle_up(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        {
            if let Some(selected) = self.app.note_state().selected() {
                let num_notes = self
                    .app
                    .notes
                    .as_ref()
                    .ok_or("this should not happen!")?
                    .len();
                if num_notes == 0 {
                    self.app.set_note_state(0);
                    return Ok(());
                }
                if selected > 0 {
                    self.app.set_note_state(selected - 1);
                } else {
                    self.app.set_note_state(num_notes - 1);
                }
            }
        }
        Ok(())
    }
    fn handle_down(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(selected) = self.app.note_state().selected() {
            let num_notes = self
                .app
                .notes
                .as_ref()
                .ok_or("this should not happen!")?
                .len();
            if num_notes == 0 {
                self.app.set_note_state(0);
                return Ok(());
            }
            if selected >= num_notes - 1 {
                self.app.set_note_state(0);
            } else {
                self.app.set_note_state(selected + 1);
            }
        }
        Ok(())
    }

    pub fn render_main_frame(
        &self,
        frame: &mut Frame<CrosstermBackend<Stdout>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Render::render(
            frame,
            self.widget.header,
            self.widget.render_tabs(self.app.state().clone()),
        );
        Render::render(frame, self.widget.footer, self.widget.render_copyright());
        Ok(())
    }
    pub fn render(
        &self,
        frame: &mut Frame<CrosstermBackend<Stdout>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self.app.state() {
            AppState::Home => {
                self.render_main_frame(frame)?;
                render::Render::render(frame, self.widget.home_area(), self.widget.render_home())
            }
            AppState::Note => {
                self.render_main_frame(frame)?;
                let notes = match self.app.notes.as_ref() {
                    Some(s) => s,
                    None => return Ok(()),
                };
                let note_area = self.widget.notes_area();
                let (left, right) = self.widget.render_notes(self.app.note_state(), notes);
                render::Render::render(frame, note_area[1], right);
                render::Render::render_stateful(
                    frame,
                    note_area[0],
                    left,
                    &mut self.app.note_state().clone(),
                );
            }
            AppState::Add => {
                let area = self.widget.add_note_area();
                render::Render::render(frame, area, self.widget.render_add_note(&self.app.buffer));
            }
            _ => (),
        }
        Ok(())
    }
}

pub enum Event<T> {
    Input(T),
    Tick,
}
