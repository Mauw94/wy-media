use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
    Frame,
};

use crate::app::{ActiveModules, App};

pub struct PlayListExplorer {
    pub index: ListState,
}

impl PlayListExplorer {
    pub fn default() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self { index: list_state }
    }
}

pub fn draw_play_list<B>(app: &mut App, frame: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let mut items = vec![];
    let player = &mut app.player;
    for item in &player.play_list.lists {
        items.push(ListItem::new(item.name.as_str()));
    }

    let mut blck = Block::default()
        .borders(Borders::ALL)
        .title("Playlist")
        .border_type(BorderType::Rounded)
        .title_alignment(Alignment::Center);

    if app.active_modules == ActiveModules::PlayList {
        blck = blck.border_style(Style::default().fg(Color::Cyan));
    }

    let list = List::new(items)
        .block(blck)
        .highlight_style(Style::default().bg(Color::Cyan))
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, &mut player.play_list.index);
}
