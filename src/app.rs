use std::{io::stdout, sync::mpsc, thread};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use failure::Error;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame, Terminal,
};

use crate::{config::Config, ui::EventType};

pub struct App {
    config: Config,
    msg: String,
}

impl App {
    pub fn new() -> Option<Self> {
        Some(Self {
            config: Config::default(),
            msg: "Welcome to wy-media".to_string(),
        })
    }

    pub fn run(&mut self) -> Result<(), Error> {
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        enable_raw_mode()?;
        terminal.hide_cursor()?;
        self.draw_frame(&mut terminal)?;
        let (sd, rd) = mpsc::channel::<EventType>();
        let tick = self.config.tick_gap.clone();
        thread::spawn(move || loop {
            thread::sleep(tick);
            let _ = sd.send(EventType::Player);
            let _ = sd.send(EventType::Radio);
        });
        loop {
            if event::poll(self.config.refresh_rate)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => {
                            // todo empty cache function
                            break;
                        }
                        code => {
                            // TODO
                            // handle_keyboard_event(self, code);
                        }
                    }
                }
            }
            thread::sleep(self.config.refresh_rate);
            self.draw_frame(&mut terminal)?;
            // if let Ok(event) = rd.try_recv() {
            //     self.handle_events(event);
            // }
        }
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        Ok(())
    }
    pub fn draw_frame<B>(&mut self, terminal: &mut Terminal<B>) -> Result<(), Error>
    where
        B: Backend,
    {
        terminal.draw(|frame| {
            let size = frame.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(4)
                .constraints([Constraint::Length(3), Constraint::Percentage(100)].as_ref())
                .split(size);

            self.draw_header(frame, chunks[0]);
        })?;
        Ok(())
    }

    pub fn draw_header<B>(&mut self, frame: &mut Frame<B>, area: Rect)
    where
        B: Backend,
    {
        let block = Block::default()
            .title("Audio player")
            .borders(Borders::ALL)
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::White));
        let msg_p = Paragraph::new(Text::from(self.msg.as_str()))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
            .block(block)
            .wrap(Wrap { trim: true });
        frame.render_widget(msg_p, area)
    }

    pub fn set_msg(&mut self, msg: &str) {
        self.msg = String::from(msg);
    }
}
