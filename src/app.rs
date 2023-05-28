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
    widgets::{Block, BorderType, Borders, ListState, Paragraph, Wrap},
    Frame, Terminal,
};

use crate::{
    config::Config,
    media::player::{MusicPlayer, Player},
    ui::{
        fs::{draw_fs_tree, FsExplorer},
        music_board::{draw_music_board, MusicController},
        radio::RadioExplorer,
        EventType,
    }, handler::handle_keyboard_event,
};

#[derive(PartialEq)]
pub enum ActiveModules {
    Fs,
    PlayList
}

pub struct App {
    pub player: MusicPlayer,
    pub radio_fs: RadioExplorer,
    pub fs: FsExplorer,
    pub music_controller: MusicController,
    pub active_modules: ActiveModules,
    config: Config,
    msg: String,
}

impl App {
    pub fn new() -> Option<Self> {
        Some(Self {
            fs: FsExplorer::default(Some(|err| {
                eprintln!("{}", err);
            }))
            .ok()?,
            player: MusicPlayer::new(),
            radio_fs: RadioExplorer::new(),
            music_controller: MusicController {
                state: ListState::default(),
            },
            active_modules: ActiveModules::Fs,
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
                            handle_keyboard_event(self, code);
                        }
                    }
                }
            }
            thread::sleep(self.config.refresh_rate);
            self.draw_frame(&mut terminal)?;
            if let Ok(event) = rd.try_recv() {
                self.handle_events(event);
            }
        }
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        Ok(())
    }

    fn handle_events(&mut self, event: EventType) {
        match event {
            EventType::Player => {
                let player = &mut self.player;
                player.tick();
            }
            EventType::Radio => {}
        }
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
            self.draw_body(frame, chunks[1]).unwrap();
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

    pub fn draw_body<B>(&mut self, frame: &mut Frame<B>, area: Rect) -> Result<(), Error>
    where
        B: Backend,
    {
        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(area);

        draw_fs_tree(self, frame, main_layout[0]);
        draw_music_board(self, frame, main_layout[1]);
        Ok(())
    }
}
