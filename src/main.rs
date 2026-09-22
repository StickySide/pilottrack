mod calendar;
mod config;
mod flight;
mod flightstats;

use anyhow::{Context, Result};
use clap::{Args, Parser};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::Widget;
use ratatui::{
    DefaultTerminal, Frame,
    style::Stylize,
    widgets::{Block, Paragraph},
};

#[derive(Parser, Debug)]
#[command(name = "PilotTrack")]
#[command(version = "0.1.0")]
#[command(about = "App to track United pilots scheduled flights")]
struct Cli {
    #[command(flatten)]
    source: Source,
    /// Departure time. Format: 'MM/DD/YY' Defaults to current date and time
    #[arg(short, long)]
    departure_time: Option<String>,
    /// Filename to use for saved calendar.
    #[arg(long, default_value_t = String::from("calendar.ics"))]
    filename: String,
    /// Ratatui test run
    #[arg(short, long)]
    ratatui: bool,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct Source {
    /// Grab next flight from configured pilots calender schedule
    #[arg(short, long)]
    calendar: bool,

    /// Flight number of flight to track. Format: '1234'
    #[arg(short, long)]
    flight_number: Option<String>,
}

#[derive(Debug, Default)]
struct App {
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

    // Handle the different key inputs
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

fn main() -> Result<()> {
    let cli: Cli = Cli::parse();

    // Get flight from calendar or one-shot
    let mut flight = {
        if cli.source.calendar == true {
            // Init config
            let config = config::Config::from_env()?;
            let filename = cli.filename;

            // Grab calendar from United
            let calendar =
                calendar::get_calendar_from_url(&config.url, &config.username, &config.password)?;

            // Save calendar
            calendar::save_calendar_to_file(&calendar, &filename)?;

            // Grab next flight from calendar
            let flight = calendar::get_next_flight(&calendar);

            flight
        } else {
            // Parse or default requested departure time
            let departure_time = match cli.departure_time {
                Some(s) => chrono::NaiveDate::parse_from_str(&s, "%m/%d/%Y")?
                    .and_hms_opt(0, 0, 0)
                    .ok_or(anyhow::anyhow!(
                        "Couldnt add default 'hms' to departure date:"
                    ))?,
                None => chrono::Local::now().naive_local(),
            };

            let mut flight = flight::Flight::default();
            // Todo: Handle this unwrap
            flight.flight_number = Some(cli.source.flight_number.unwrap());
            flight.scheduled_departure = Some(departure_time);
            flight
        }
    };

    // Update flight with live info

    let live_update =
        flightstats::get_live_update(&flight.scheduled_departure, &flight.flight_number)?;
    flight.live_update(live_update);
    println!("{flight:#?}");

    let mut app = App::from(flight);

    if cli.ratatui == true {
        let app_result = ratatui::run(|terminal| app.run(terminal));
        return app_result;
    }

    Ok(())
}
