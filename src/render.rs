use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    widgets::{List, ListState, Widget as TWidget},
    Frame,
};

pub struct Render;

impl Render {
    pub fn render(
        frame: &mut Frame<CrosstermBackend<Stdout>>,
        area: Rect,
        widget: impl TWidget,
    ) -> () {
        frame.render_widget(widget, area)
    }
    pub fn render_stateful(
        frame: &mut Frame<CrosstermBackend<Stdout>>,
        area: Rect,
        widget: List,
        state: &mut ListState,
    ) {
        frame.render_stateful_widget(widget, area, state);
    }
}
