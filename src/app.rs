use crate::cli::Args;
use crate::config::AppConfig;
use crate::network::ping_host;
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
use std::net::Ipv4Addr;
use std::process::Command;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};
use std::{fs, io};

#[derive(Debug, Default)]
enum AppState {
    #[default]
    Info,
    Success,
}

enum AppEvent {
    PingResult(usize, bool),
}

enum ControlCommand {
    TriggerPing,
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
        let (tx_ui, rx_ui) = mpsc::channel();
        let (tx_ctrl, rx_ctrl) = mpsc::channel();

        let hosts_data: Vec<(usize, Ipv4Addr)> = self
            .config
            .hosts
            .iter()
            .enumerate()
            .map(|(i, h)| (i, h.ip))
            .collect();

        thread::spawn(move || {
            loop {
                for (index, ip) in &hosts_data {
                    let is_online = ping_host(*ip);
                    let _ = tx_ui.send(AppEvent::PingResult(*index, is_online));
                }

                match rx_ctrl.recv_timeout(Duration::from_secs(30)) {
                    Ok(ControlCommand::TriggerPing) => continue,
                    Err(mpsc::RecvTimeoutError::Timeout) => continue,
                    Err(_) => break,
                }
            }
        });

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') => self.exit(),
                            KeyCode::Char('r') => {
                                let _ = tx_ctrl.send(ControlCommand::TriggerPing);
                            }
                            _ => {}
                        }
                    }
                }
            }

            while let Ok(event) = rx_ui.try_recv() {
                match event {
                    AppEvent::PingResult(index, is_online) => {
                        if let Some(host) = self.config.hosts.get_mut(index) {
                            host.is_online = is_online;
                        }
                    }
                }
            }
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

        Paragraph::new("stand status")
            .block(Block::bordered().green())
            .green()
            .render(chunks[0], buf);

        let rows: Vec<Row> = self
            .config
            .hosts
            .iter()
            .map(|host| {
                let (status_text, color) = if host.is_online {
                    ("Online", Color::Green)
                } else {
                    ("Offline", Color::Red)
                };

                Row::new(vec![
                    Cell::from(host.name.as_str()),
                    Cell::from(host.ip.to_string()),
                    Cell::from(status_text),
                ])
                .style(Style::default().fg(color))
            })
            .collect();

        let table = Table::new(
            rows,
            [
                Constraint::Percentage(30),
                Constraint::Percentage(40),
                Constraint::Percentage(30),
            ],
        )
        .header(Row::new(vec!["host", "ip", "status"]).bold().green())
        .block(Block::bordered().title(" Hosts ").green())
        .on_black();

        table.render(chunks[1], buf);
    }
}
