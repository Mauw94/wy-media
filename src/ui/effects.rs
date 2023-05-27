use rand::Rng;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{BarChart, Block, BorderType, Borders},
    Frame,
};

use crate::{app::App, media::player::Player};

pub fn draw_chart_effects<B>(app: &mut App, frame: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let player = &mut app.player;

    let mut rng = rand::thread_rng();
    let mut cols = vec![];
    for _ in 0..20 {
        let mut i = rng.gen_range(0..10);
        if !player.is_playing() {
            i = 0;
        }
        cols.push(("_", i));
    }
    let items = BarChart::default()
        .bar_width(4)
        .bar_gap(1)
        .bar_style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .data(&cols)
        .value_style(Style::default().add_modifier(Modifier::ITALIC))
        .label_style(Style::default().add_modifier(Modifier::ITALIC))
        .max(10)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Wave")
                .title_alignment(tui::layout::Alignment::Center),
        );
    frame.render_widget(items, area);
}
