use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Row, Table},
    Frame,
};

use crate::app::App;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Create main layout
    let chunks = Layout::default()
        .constraints([Constraint::Min(36), Constraint::Min(0)].as_ref())
        .direction(Direction::Horizontal)
        .split(f.size());

    // Create and render contexts widget
    let contexts: Vec<Row> = app
        .contexts
        .items
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
        .block(Block::default().borders(Borders::ALL).title("Contexts"))
        .widths(&[Constraint::Min(11), Constraint::Min(11), Constraint::Min(6)])
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    f.render_stateful_widget(contexts_widget, chunks[0], &mut app.contexts.state);

    // Create and render paths widget
    let selected_context = app.contexts.state.selected();
    let paths: Vec<ListItem> = app.contexts.items[selected_context.unwrap()]
        .paths
        .iter()
        .map(|path| ListItem::new(path.clone()))
        .collect();

    let path_widget = List::new(paths).block(Block::default().borders(Borders::ALL).title("Paths"));

    f.render_widget(path_widget, chunks[1]);
}
