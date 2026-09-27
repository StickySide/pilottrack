use crate::flight;

use anyhow::Result;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::{Block, Paragraph, Widget};
use ratatui::{DefaultTerminal, Frame};

#[derive(Debug, Default)]
pub struct App {
    quit: bool,
    flight: flight::Flight,
}

// Ratatui App
impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.quit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.quit(),
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.quit = true
    }

    fn flight_info(&self) -> Vec<Line> {
        let not_available = "Not available";

        let flight_number = match &self.flight.flight_number {
            Some(number) => number,
            None => not_available,
        };

        let departure = match &self.flight.departure {
            Some(departure) => departure,
            None => not_available,
        };

        let arrival = match &self.flight.arrival {
            Some(departure) => departure,
            None => not_available,
        };

        vec![
            Line::from(vec!["Flight Number: ".bold(), flight_number.into()]),
            Line::from(vec!["Departure: ".bold(), departure.into()]),
            Line::from(vec!["Arrival: ".bold(), arrival.into()]),
        ]
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("PilotTrack".bold());
        let instructions = Line::from("<Q> to quit".blue());
        let flight_info = self.flight_info();

        let block = Block::bordered()
            .title_top(title.centered())
            .title_bottom(instructions.centered());

        Paragraph::new(flight_info).block(block).render(area, buf);
    }
}

impl From<flight::Flight> for App {
    fn from(flight: flight::Flight) -> App {
        App {
            quit: false,
            flight,
        }
    }
}
