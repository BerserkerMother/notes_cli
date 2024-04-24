use std::io::Stdout;
mod app;
mod editor_handler;
mod handler;
mod render;
mod repository;
mod service;
mod widgets;
pub use handler::AppHandler;
pub use handler::Event;
pub use repository::Note;
use tui::{backend::CrosstermBackend, layout::Rect, Terminal};

use self::{app::App, repository::Repository};

pub type CrossTerminal = Terminal<CrosstermBackend<Stdout>>;

pub type ResultDynError<T> = Result<T, Box<dyn std::error::Error>>;
pub fn get_handler(f: Rect) -> ResultDynError<AppHandler> {
    let db = Repository::new("./notes.db")?;
    db.initialize_db()?;
    let app = App::new();
    Ok(AppHandler::new(app, db, f))
}
