use crate::cli::Args;
use crate::config::AppConfig;
use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Cell, Paragraph, Row, Table, Widget},
};
use std::{fs, io};

#[derive(Debug, Default)]
enum AppState {
    #[default]
    Info,
    Success,
}

#[derive(Debug)]
pub struct App {
    config: AppConfig,
    state: AppState,
    exit: bool,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config,
            state: AppState::Info,
            exit: false,
        }
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);

        let title = Paragraph::new("stand status")
            .block(Block::bordered().green().on_black())
            .green()
            .on_black();
        title.render(chunks[0], buf);

        let rows = vec![
            Row::new(vec!["server-01", "192.168.0.1", "Online"]).green(),
            Row::new(vec!["server-02", "192.168.0.2", "Offline"]).red(),
        ];

        let table = Table::new(
            rows,
            [
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ],
        )
        .header(Row::new(vec!["host", "ip", "status"]).bold().green())
        .block(Block::bordered().title(" Hosts ").green().on_black())
        .on_black();

        table.render(chunks[1], buf);
    }
}
