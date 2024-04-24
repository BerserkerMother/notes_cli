use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
        Wrap,
    },
};

use crate::{app::AppState, Note};

pub struct Widget {
    pub header: Rect,
    pub main_part: Rect,
    pub footer: Rect,
    pub all: Rect,
}

impl Widget {
    pub fn new(size: Rect) -> Widget {
        let chunks = Layout::default()
            .direction(tui::layout::Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(2),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(size);
        Widget {
            header: chunks[0],
            main_part: chunks[1],
            footer: chunks[2],
            all: size,
        }
    }
    pub fn render_copyright<'a>(&self) -> Paragraph<'a> {
        let copyright = Paragraph::new("Ali Shokohi!")
            .style(Style::default().fg(Color::LightCyan))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Koni")
                    .title_alignment(Alignment::Center)
                    .style(Style::default().fg(Color::LightCyan))
                    .border_type(BorderType::Plain),
            );
        copyright
    }
    pub fn render_tabs<'a>(&self, active_menu_item: AppState) -> Tabs<'a> {
        let menu_titles = vec!["Home", "Notes", "Add", "Delete", "Quit"];
        let menu = menu_titles
            .iter()
            .map(|t| {
                let (first, rest) = t.split_at(1);
                Spans::from(vec![
                    Span::styled(
                        first,
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::UNDERLINED),
                    ),
                    Span::styled(rest, Style::default().fg(Color::White)),
                ])
            })
            .collect();

        let tabs = Tabs::new(menu)
            .select(active_menu_item.into())
            .block(Block::default().title("Menu").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .divider(Span::raw("|"));
        tabs
    }
    pub fn render_add_note<'a>(&self, buffer: &'a str) -> Paragraph<'a> {
        let text = Paragraph::new(vec![Spans::from(vec![Span::styled(
            buffer,
            Style::default().fg(Color::White),
        )])])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Yellow))
                .title("title")
                .border_type(tui::widgets::BorderType::Double) // Use double lines for the border.
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .alignment(Alignment::Center) // Center the text horizontally.
        .wrap(Wrap { trim: true });
        text
    }
    pub fn add_note_area(&self) -> Rect {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Percentage(45),
                    Constraint::Percentage(10),
                    Constraint::Percentage(45),
                ]
                .as_ref(),
            )
            .split(self.all);

        let middle_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(30),
                    Constraint::Percentage(40),
                    Constraint::Percentage(30),
                ]
                .as_ref(),
            )
            .split(chunks[1]);
        return middle_chunk[1];
    }

    /// app high level widgets
    pub fn render_home<'a>(&self) -> Paragraph<'a> {
        let home = Paragraph::new(vec![
    Spans::from(vec![Span::raw("")]),
    Spans::from(vec![Span::raw("Welcome")]),
    Spans::from(vec![Span::raw("")]),
    Spans::from(vec![Span::raw("to")]),
    Spans::from(vec![Span::raw("")]),
    Spans::from(vec![Span::styled(
        "pet-CLI",
        Style::default().fg(Color::LightBlue),
    )]),
    Spans::from(vec![Span::raw("")]),
    Spans::from(vec![Span::raw("Press 'n' to access notes, 'a' to add new note and 'd' to delete the currently selected note.")]),
])
.alignment(Alignment::Center)
.block(
    Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Home")
        .border_type(BorderType::Plain),
);
        home
    }
    pub fn home_area(&self) -> Rect {
        self.main_part
    }
    pub fn notes_area(&self) -> Vec<Rect> {
        Layout::default()
            .direction(tui::layout::Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
            .split(self.main_part)
    }
    pub fn render_notes<'a>(
        &self,
        note_list_state: &ListState,
        note_list: &Vec<Note>,
    ) -> (List<'a>, Table<'a>) {
        let notes = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Notes")
            .border_type(BorderType::Plain);

        let items: Vec<_> = note_list
            .iter()
            .map(|note| {
                ListItem::new(Spans::from(vec![Span::styled(
                    note.title.clone(),
                    Style::default(),
                )]))
            })
            .collect();
        let selected_note = note_list
            .get(
                note_list_state
                    .selected()
                    .expect("there is always a selected note"),
            )
            .expect("exists")
            .clone();

        let list = List::new(items).block(notes).highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

        let note_detail = Table::new(vec![Row::new(vec![Cell::from(Span::raw(
            selected_note.text,
        ))])])
        .header(Row::new(vec![Cell::from(Span::styled(
            "Text",
            Style::default().add_modifier(Modifier::BOLD),
        ))]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Detail")
                .border_type(BorderType::Plain),
        )
        .widths(&[Constraint::Percentage(100)]);

        (list, note_detail)
    }
}
