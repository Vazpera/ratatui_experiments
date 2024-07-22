use std::io;

use crate::tui;
use ratatui::crossterm::style::Stylize;
use ratatui::crossterm::terminal::size;
use ratatui::symbols::line::VERTICAL;
use ratatui::widgets::block::{Position, Title};
use ratatui::widgets::{self, BorderType, Gauge};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Alignment,
    layout::Layout,
    layout::Rect,
    prelude::Color,
    prelude::Constraint,
    prelude::Direction,
    prelude::Style,
    text::Line,
    text::Span,
    widgets::{Block, Paragraph, Row, Widget},
    Frame,
};

#[derive(Clone, Copy)]
pub struct App {
    exit: bool,
    x: u8,
    y: u8,
}

impl AsMut<App> for App {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<'a> Default for App {
    fn default() -> Self {
        Self {
            exit: false,
            x: 0,
            y: 0,
        }
    }
}

impl<'a> Widget for App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let size_limit = vec![Constraint::Max(18)];
        let layout = Layout::new(Direction::Horizontal, vec![Constraint::Max(18)]).split(area);
        let board_area =
            Layout::new(Direction::Vertical, vec![Constraint::Max(10)]).split(layout[0]);
        let mut lines: Vec<Line> = vec![];
        for i in 0..8 {
            let mut spans: Vec<Span> = vec![];
            for j in 0..8 {
                let grid_color = (i + j) % 2;
                spans.push(match (grid_color, self.x==j, self.y==i) {
                    (_, true, true) => Span::styled("  ", Style::default().bg(Color::from_u32(0x00888888))),
                    (0, _, _) => Span::styled("  ", Style::default().bg(Color::Black)),
                    (1, _, _) => Span::styled("  ", Style::default().bg(Color::White)),
                    (_, _, _) => Span::styled("  ", Style::default().bg(Color::Red)),
                });
                
            }
            lines.push(Line::from(spans));
        }
        let _ = Paragraph::new(lines)
            .block(Block::bordered().title("Board"))
            .render(board_area[0], buf);
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
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Left => self.x = (self.x + 7) % 8,
            KeyCode::Right => self.x = (self.x + 1) % 8,
            KeyCode::Up => self.y = (self.y + 7) % 8,
            KeyCode::Down => self.y = (self.y + 1) % 8,
            _ => {}
        }
    }
}
