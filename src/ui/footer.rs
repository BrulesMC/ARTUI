use crate::Focus;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_footer(f: &mut Frame, area: Rect, focus: Focus, saved_active: bool) {
    let text = if saved_active {
        "Saved!"
    } else {
        match focus {
            Focus::Profiles => "s save | a apply | h/l/tab switch | j/k move | space set active",
            Focus::AsusCtl => "s save | a apply | h/l/tab switch | j/k move | c/g toggle",
            Focus::RyzenAdj => {
                "s save | a apply | h/l/tab switch | j/k move | space/enter toggle bool"
            }
        }
    };
    let mut block = Block::default().borders(Borders::ALL).title("Status");
    block = block.border_style(Style::default().add_modifier(Modifier::BOLD));

    let para = Paragraph::new(text).block(block);
    f.render_widget(para, area);
}
