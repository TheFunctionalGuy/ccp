use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Row, Table},
    Frame,
};

use crate::app::App;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(36), Constraint::Percentage(100)].as_ref())
        .split(f.size());

    // Decide layout style
    let (context_color, path_color) = match app.window_state {
        crate::app::Window::Contexts => (Color::Green, Color::Reset),
        crate::app::Window::Paths => (Color::Reset, Color::Green),
    };

    // Create and render contexts widget
    let contexts: Vec<Row> = app
        .contexts
        .iter()
        .map(|i| {
            Row::new([
                format!("0x{:08x}", i.pc),
                format!("0x{:08x}", i.lr),
                format!("{:>6}", format!("[{}]", i.count)),
            ])
        })
        .collect();

    let contexts_widget = Table::new(contexts)
        .header(Row::new(vec!["PC", "LR", "Count"]).bottom_margin(1))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(context_color))
                .title("Contexts"),
        )
        .widths(&[Constraint::Min(11), Constraint::Min(11), Constraint::Min(6)])
        .highlight_style(Style::default().fg(Color::Green))
        .highlight_symbol(">> ");

    f.render_stateful_widget(contexts_widget, chunks[0], &mut app.context_state);

    // Create and render paths widget
    let selected_context = app.context_state.selected();
    let paths: Vec<ListItem> = app.contexts[selected_context.unwrap()]
        .paths
        .iter()
        .enumerate()
        .map(|(index, path)| ListItem::new(format!(" {:>3}  {}", index + 1, path)))
        .collect();

    let path_widget = List::new(paths)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(path_color))
                .title("Paths"),
        )
        .highlight_style(Style::default().fg(Color::Green))
        .highlight_symbol(">>");

    f.render_stateful_widget(path_widget, chunks[1], &mut app.path_state);
}
