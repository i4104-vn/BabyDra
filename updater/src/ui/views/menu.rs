use crate::core::app::App;
use crate::ui::theme::*;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem};
use ratatui::Frame;

pub fn render_action_list(frame: &mut Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = app
        .menu_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let is_selected = i == app.selected_menu;

            let (prefix, text_style, tag_style) = if is_selected {
                (
                    " ▶ ",
                    Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD),
                    Style::default()
                        .fg(COLOR_BG)
                        .bg(COLOR_CYAN)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                (
                    "   ",
                    Style::default().fg(COLOR_TEXT_WHITE),
                    Style::default().fg(COLOR_TEXT_MUTED),
                )
            };

            let line = Line::from(vec![
                Span::styled(
                    prefix,
                    Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD),
                ),
                Span::styled(format!(" {:<6} ", item.tag), tag_style),
                Span::raw(" "),
                Span::raw(item.icon),
                Span::raw(" "),
                Span::styled(item.title, text_style),
            ]);

            ListItem::new(vec![line, Line::from("")])
        })
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(COLOR_CYAN))
        .title(" 📋 Actions ")
        .title_style(Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD));

    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}
