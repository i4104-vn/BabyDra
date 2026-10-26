use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use crate::ui::THEME;

/// Draws step 2: choose the git branch to install from.
pub fn draw_branch_step(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(6),
            Constraint::Length(8),
        ])
        .split(area);

    let (search_border_color, search_title, search_content) = if app.branch_search_active {
        (
            THEME.mint,
            " 🔍 Search Branch (ACTIVE) — Type to filter | [Esc] Exit search ",
            Line::from(vec![
                Span::styled(
                    " Filter: ",
                    Style::default().fg(THEME.mint).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    &app.branch_search_query,
                    Style::default().fg(THEME.text_bright).add_modifier(Modifier::BOLD),
                ),
                Span::styled("█", Style::default().fg(THEME.mint)),
            ]),
        )
    } else if !app.branch_search_query.is_empty() {
        (
            THEME.cyan,
            " 🔍 Search Branch — Press [/] to edit search | [Esc] Clear ",
            Line::from(vec![
                Span::styled(" Filter: ", Style::default().fg(THEME.cyan)),
                Span::styled(
                    &app.branch_search_query,
                    Style::default().fg(THEME.text_bright),
                ),
                Span::styled("  [Press Esc to clear filter]", Style::default().fg(THEME.text_dim)),
            ]),
        )
    } else {
        (
            THEME.border_normal,
            " 🔍 Search Branch — Press [/] or [f] to search ",
            Line::from(vec![
                Span::styled(" Press ", Style::default().fg(THEME.text_dim)),
                Span::styled(" [/] ", THEME.key_badge_cyan()),
                Span::styled(" or ", Style::default().fg(THEME.text_dim)),
                Span::styled(" [f] ", THEME.key_badge_cyan()),
                Span::styled(
                    " to search and filter branches by name...",
                    Style::default().fg(THEME.text_dim),
                ),
            ]),
        )
    };

    let search_widget = Paragraph::new(search_content).block(
        Block::default()
            .title(search_title)
            .title_style(
                Style::default()
                    .fg(search_border_color)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(search_border_color)),
    );
    f.render_widget(search_widget, chunks[0]);

    if app.branches.is_empty() {
        // Empty state when no branches exist
        let empty_lines = vec![
            Line::from(""),
            Line::from(Span::styled(
                "  No installation releases available in repository!  ",
                THEME.title_rose(),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Could not find any git branch containing a source workspace.",
                Style::default().fg(THEME.text_bright),
            )),
            Line::from(Span::styled(
                "Branch 'main' only hosts installer tools and documentation, with no desktop source code.",
                Style::default().fg(THEME.text_dim),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Hint: Check the remote connection and ensure a source branch is available.",
                Style::default().fg(THEME.amber),
            )),
        ];

        let empty_widget = Paragraph::new(empty_lines).wrap(Wrap { trim: true }).block(
            Block::default()
                .title(" 2. Source Branch — No Installable Releases Found ")
                .title_style(THEME.title_rose())
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(THEME.rose)),
        );
        f.render_widget(empty_widget, chunks[1]);

        let prompt_box = Paragraph::new(vec![
            Line::from(Span::styled(
                "Status: Cannot proceed without an installable source branch.",
                Style::default().fg(THEME.rose).add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                format!("Repository: {}", app.workspace_root.display()),
                Style::default().fg(THEME.text_dim),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled(" [q] ", THEME.key_badge_red()),
                Span::styled(" Quit installer    ", Style::default().fg(THEME.text_dim)),
                Span::styled(" [← / p] ", THEME.key_badge_amber()),
                Span::styled(" Back to overview", Style::default().fg(THEME.text_dim)),
            ]),
        ])
        .block(
            Block::default()
                .title(" Installation Notice ")
                .title_style(THEME.title_rose())
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(THEME.rose)),
        );
        f.render_widget(prompt_box, chunks[2]);
        return;
    }

    let filtered = app.filtered_branches();
    if filtered.is_empty() {
        let no_match = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled(
                format!("  No branches match filter: '{}'  ", app.branch_search_query),
                Style::default().fg(THEME.amber).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Press [Esc] or [Backspace] to clear or adjust your search filter.",
                Style::default().fg(THEME.text_dim),
            )),
        ])
        .block(
            Block::default()
                .title(" 2. Source Branch Selection ")
                .title_style(THEME.title_cyan())
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(THEME.amber)),
        );
        f.render_widget(no_match, chunks[1]);
    } else {
        let items: Vec<ListItem> = filtered
            .iter()
            .enumerate()
            .map(|(display_idx, (_orig_idx, b))| {
                let is_recommended = b.is_current;
                let is_selected = app.selected_branch == b.name;
                branch_row(
                    display_idx,
                    &app.branch_cursor,
                    &b.name,
                    b.is_current,
                    b.has_remote,
                    is_recommended,
                    is_selected,
                )
            })
            .collect();

        let count_info = if app.branch_search_query.is_empty() {
            format!("{} available", app.branches.len())
        } else {
            format!("{}/{} matches", filtered.len(), app.branches.len())
        };

        let list_title = format!(
            " 2. Source Branch Selection ({count_info}) [↑/↓: Move | Space: Select | Enter: Switch Branch] "
        );

        let list = List::new(items).block(
            Block::default()
                .title(list_title)
                .title_style(THEME.title_cyan())
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(THEME.cyan)),
        );
        f.render_widget(list, chunks[1]);
    }

    let prompt_box = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Target Branch : ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                format!("'{}'", app.selected_branch),
                Style::default().fg(THEME.cyan).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " — Pulled into branches/ to keep main branch clean.",
                Style::default().fg(THEME.mint),
            ),
        ]),
        Line::from(vec![
            Span::styled("Repository    : ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                format!(
                    "{} ({} branches available)",
                    app.workspace_root.display(),
                    app.branches.len()
                ),
                Style::default().fg(THEME.text_bright),
            ),
        ]),
        Line::from(vec![
            Span::styled("Recommendation: ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                "The current source branch is highlighted when available.",
                Style::default().fg(THEME.amber),
            ),
        ]),
        Line::from(vec![
            Span::styled(" [/] ", THEME.key_badge_cyan()),
            Span::styled(" Search    ", Style::default().fg(THEME.text_dim)),
            Span::styled(" [Space] ", THEME.key_badge_green()),
            Span::styled(" Select    ", Style::default().fg(THEME.text_dim)),
            Span::styled(" [Enter] ", THEME.key_badge_cyan()),
            Span::styled(
                " Confirm & Switch Branch    ",
                Style::default().fg(THEME.text_dim),
            ),
            Span::styled(" [←] ", THEME.key_badge_amber()),
            Span::styled(" Back", Style::default().fg(THEME.text_dim)),
        ]),
    ])
    .block(
        Block::default()
            .title(" Target Branch Information ")
            .title_style(THEME.title_cyan())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(THEME.border_normal)),
    );
    f.render_widget(prompt_box, chunks[2]);
}

fn branch_row<'a>(
    row_idx: usize,
    cursor: &usize,
    name: &'a str,
    is_current: bool,
    has_remote: bool,
    is_recommended: bool,
    is_selected: bool,
) -> ListItem<'a> {
    let is_cursor = row_idx == *cursor;

    let radio = if is_selected {
        Span::styled(
            "(●) ",
            Style::default().fg(THEME.mint).add_modifier(Modifier::BOLD),
        )
    } else {
        Span::styled("( ) ", Style::default().fg(THEME.text_muted))
    };

    let title_style = if is_cursor {
        Style::default().fg(THEME.cyan).add_modifier(Modifier::BOLD)
    } else if is_selected {
        Style::default()
            .fg(THEME.text_bright)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(THEME.text_body)
    };

    let mut spans = vec![radio, Span::styled(name, title_style)];

    if is_recommended {
        spans.push(Span::raw(" "));
        spans.push(Span::styled(
            " [Recommended] ",
            Style::default()
                .fg(THEME.amber)
                .bg(THEME.bg_badge)
                .add_modifier(Modifier::BOLD),
        ));
    }

    if is_current {
        spans.push(Span::raw(" "));
        spans.push(Span::styled(
            " [local] ",
            Style::default().fg(THEME.purple).bg(THEME.bg_badge),
        ));
    }

    if has_remote {
        spans.push(Span::raw(" "));
        spans.push(Span::styled(
            " [remote] ",
            Style::default().fg(THEME.text_dim).bg(THEME.bg_badge),
        ));
    }

    if is_selected {
        spans.push(Span::raw(" "));
        spans.push(Span::styled(
            " [SELECTED] ",
            Style::default()
                .fg(THEME.mint)
                .bg(THEME.bg_badge)
                .add_modifier(Modifier::BOLD),
        ));
    } else if is_cursor {
        spans.push(Span::raw(" "));
        spans.push(Span::styled(
            " [Space: Select] ",
            Style::default()
                .fg(THEME.cyan)
                .bg(THEME.bg_badge)
                .add_modifier(Modifier::BOLD),
        ));
    }

    let bg_style = if is_cursor {
        Style::default().bg(THEME.bg_cursor)
    } else if is_selected {
        Style::default().bg(THEME.bg_selected)
    } else {
        Style::default()
    };

    ListItem::new(Line::from(spans)).style(bg_style)
}
