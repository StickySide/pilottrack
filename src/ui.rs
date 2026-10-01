use std::time::Duration;

use crate::flight;

use anyhow::Result;

use chrono::Utc;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::symbols::merge::MergeStrategy;
use ratatui::text::{Line, ToText};
use ratatui::widgets::{Block, Paragraph, Widget};
use ratatui::{DefaultTerminal, Frame};

#[derive(Debug, Default)]
pub struct App {
    quit: bool,
    flight: flight::Flight,
    status: Option<String>,
    last_updated: Option<chrono::DateTime<Utc>>,
}

// Ratatui App
impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.quit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
            match self.last_updated {
                None => {
                    self.flight.update()?;
                    self.last_updated = Some(chrono::Utc::now());
                }
                Some(time) => {
                    // Todo: Magic number for live flight update here...
                    if chrono::Utc::now() - time > chrono::Duration::minutes(1) {
                        self.flight.update()?;
                        self.last_updated = Some(chrono::Utc::now());
                    }
                }
            }
        }
        Ok(())
    }

    pub fn flight(&mut self, flight: flight::Flight) {
        self.flight = flight;
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(1000))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            }
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

    fn flight_info(&self) -> Vec<Line<'_>> {
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
        let last_updated: Line<'_> = match self.last_updated {
            None => Line::from("Never".to_owned()),
            Some(time) => {
                let time_since_update = chrono::Utc::now() - time;
                Line::from(time_since_update.num_seconds().to_string())
            }
        };

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Fill(1), Constraint::Max(3)])
            .split(area);

        let flight_block = Block::bordered()
            .title_top(title.centered())
            .title_bottom(instructions.centered());

        let update_block = Block::bordered().title_top(Line::from("Last Updated").centered());

        Paragraph::new(flight_info)
            .block(flight_block)
            .render(layout[0], buf);

        Paragraph::new(last_updated)
            .centered()
            .block(update_block)
            .render(layout[1], buf);
    }
}

impl From<flight::Flight> for App {
    fn from(flight: flight::Flight) -> App {
        App {
            quit: false,
            flight,
            status: None,
            last_updated: None,
        }
    }
}

// impl Default for App {
//     fn default() -> Self {

//     }
// }
