use tui::{
    backend::Backend,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::app::App;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {}

fn render<'a>() {
    let contexts = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Contexts")
        .border_type(BorderType::Plain);
}
