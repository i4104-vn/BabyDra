use crate::core::app::App;
use crate::ui::theme::*;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem};
use ratatui::Frame;

pub fn render_action_list(frame: &mut Frame, area: Rect, app: &App) {
    let inner_height = area.height.saturating_sub(2) as usize;
    let total_items = app.menu_items.len();

    // Window scroll offset so the selected item is always visible even in small terminals
    let offset = if inner_height >= total_items || inner_height == 0 {
        0
    } else if app.selected_menu >= inner_height {
        app.selected_menu.saturating_sub(inner_height - 1)
    } else {
        0
    };

    let items: Vec<ListItem> = app
        .menu_items
        .iter()
        .enumerate()
        .skip(offset)
        .take(if inner_height > 0 { inner_height } else { total_items })
        .map(|(i, item)| {
            let is_selected = i == app.selected_menu;
            let is_quit = item.id == crate::core::state::ActionId::Quit;

            let (prefix, text_style, tag_style) = if is_selected {
                let tag_bg = if is_quit { COLOR_RED } else { COLOR_CYAN };
                (
                    " ▶ ",
                    Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD),
                    Style::default()
                        .fg(COLOR_BG)
                        .bg(tag_bg)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                let tag_fg = if is_quit { COLOR_RED } else { COLOR_TEXT_MUTED };
                (
                    "   ",
                    Style::default().fg(COLOR_TEXT_WHITE),
                    Style::default().fg(tag_fg).add_modifier(if is_quit {
                        Modifier::BOLD
                    } else {
                        Modifier::empty()
                    }),
                )
            };

            let line = Line::from(vec![
                Span::styled(
                    prefix,
                    Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD),
                ),
                Span::styled(format!(" {:<6} ", item.tag), tag_style),
                Span::raw(" "),
                Span::styled(item.title, text_style),
            ]);

            ListItem::new(line)
        })
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(COLOR_CYAN))
        .title(" Actions ")
        .title_style(Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD));

    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}
