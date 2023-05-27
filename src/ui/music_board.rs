use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::Spans,
    widgets::{Block, BorderType, Borders, LineGauge, ListState, Paragraph},
    Frame,
};

use crate::{app::App, media::player::Player};

use super::{effects::draw_chart_effects, play_list::draw_play_list, progress::draw_progress};

pub struct MusicController {
    pub state: ListState,
}

pub fn draw_music_board<B>(app: &mut App, frame: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let main_layout_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Percentage(80),
            Constraint::Percentage(20),
        ])
        .split(area);

    draw_header(app, frame, main_layout_chunks[0]);

    let mid_layout_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(main_layout_chunks[1]);

    draw_chart_effects(app, frame, mid_layout_chunks[0]);
    draw_play_list(app, frame, mid_layout_chunks[1]);
    draw_progress(app, frame, main_layout_chunks[2]);
}

pub fn draw_header<B>(app: &mut App, frame: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let player = &app.player;
    let main_layout_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(area);

    let playing_text;
    if let Some(item) = player.playing_song() {
        playing_text = String::from(item.name.as_str());
    } else {
        playing_text = String::from("None");
    }

    let text = Paragraph::new(playing_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Now playing")
                .title_alignment(Alignment::Center),
        )
        .style(Style::default().add_modifier(Modifier::SLOW_BLINK));

    let sub_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(main_layout_chunks[0]);

    let sound_volume_percentage = app.player.volume();
    let bar = LineGauge::default()
        .ratio(sound_volume_percentage.into())
        .label("VOL")
        .line_set(symbols::line::THICK)
        .block(
            Block::default()
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL),
        )
        .gauge_style(
            Style::default()
                .fg(Color::LightCyan)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(text, sub_layout[0]);
    frame.render_widget(bar, sub_layout[1]);
    let mut p = Paragraph::new(vec![Spans::from("▶(s) >>|(n) EXT(q) HLP(h)")])
        .style(Style::default())
        .alignment(Alignment::Center);
    if player.is_playing() {
        p = Paragraph::new(vec![Spans::from("||(s) >>|(n) EXT(q) HELP(h)")])
            .alignment(Alignment::Center);
    }
    let blck = Block::default()
        .borders(Borders::ALL)
        .title("Panel")
        .border_type(BorderType::Rounded)
        .title_alignment(Alignment::Center);

    p = p.block(blck);
    frame.render_widget(p, main_layout_chunks[1]);
}
