use std::io;

use crate::tui;
use ratatui::widgets::block::{Position, Title};
use ratatui::widgets::{self, BorderType, Gauge};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Alignment,
    layout::Layout,
    layout::Rect,
    prelude::Constraint,
    prelude::Direction,
    prelude::Position,
    prelude::Style,
    style::Stylize,
    
    text::Line,
    text::Span,
    widgets::{Block, Paragraph, Row, Widget},
    Frame,
};

#[derive(Clone, Copy)]
pub struct App {
    exit: bool,
}

impl AsMut<App> for App {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<'a> Default for App {
    fn default() -> Self {
        Self { exit: false }
    }
}

impl<'a> Widget for App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {   
        let half_constraints = vec![Constraint::Percentage(50), Constraint::Percentage(50)];
        let horizontal_split = Layout::new(Direction::Horizontal, half_constraints.clone());
        let vertical_split = Layout::new(Direction::Vertical, half_constraints.clone());
        let layout = horizontal_split.clone().split(area);
        let right = vertical_split.split(layout[1].clone());
        let top_right = horizontal_split.clone().split(right[0].clone());

        let block = Block::bordered().border_type(BorderType::Thick);
        let _ = Paragraph::new(Line::from(vec!["Left".into()]))
            .alignment(Alignment::Center)
            .block(block.clone())
            .render(layout[0], buf);
        let _ = Paragraph::new(Line::from(vec!["Bottom Right".into()]))
            .alignment(Alignment::Center)
            .block(block.clone())
            .render(right[1], buf);
        let _ = Paragraph::new(Line::from(vec!["Top Middle".into()]))
            .alignment(Alignment::Center)
            .block(block.clone())
            .render(top_right[0], buf);
        let _ = Paragraph::new(Line::from(vec!["Top Right".into()]))
            .alignment(Alignment::Center)
            .block(block.clone())
            .render(top_right[1], buf);
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(*self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match (key_event.code) {
            (KeyCode::Char('q')) => self.exit = true,
            _ => {}
        }
    }
}
