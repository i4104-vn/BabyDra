use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;
use crate::ui::THEME;

/// Draws step 2: choose the git branch to install from.
pub fn draw_branch_step(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Length(8)])
        .split(area);

    let build_from_source = app.is_build_from_source();

    // Row 0: pre-built only.
    let mut items: Vec<ListItem> = Vec::with_capacity(app.branches.len() + 1);
    items.push(branch_row(
        0,
        &app.branch_cursor,
        "(none) — use pre-built binaries only",
        &[],
        build_from_source,
        app.selected_branch.is_empty(),
    ));

    for (i, b) in app.branches.iter().enumerate() {
        let row_idx = i + 1;
        let mut tags = Vec::new();
        if b.is_current {
            tags.push("current");
        }
        if b.has_remote {
            tags.push("remote");
        }
        items.push(branch_row(
            row_idx,
            &app.branch_cursor,
            &b.name,
            &tags,
            build_from_source,
            app.selected_branch == b.name,
        ));
    }

    let list = List::new(items).block(
        Block::default()
            .title(" 2. Install Source — Pick a Git Branch [↑/↓: Move | Space: Select | Enter: Switch Branch] ")
            .title_style(THEME.title_cyan())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(THEME.cyan)),
    );
    f.render_widget(list, chunks[0]);

    let (mode_color, mode_text) = if build_from_source {
        (
            THEME.amber,
            format!(
                "★ Target branch: '{}' (Will checkout, pull, and rescan workspace components on Enter).",
                app.selected_branch
            ),
        )
    } else {
        (
            THEME.mint,
            "● Target mode: Pre-built binaries only (No git checkout or cargo rebuild).".to_string(),
        )
    };

    let prompt_box = Paragraph::new(vec![
        Line::from(Span::styled(
            mode_text,
            Style::default()
                .fg(mode_color)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!(
                "Repository: {} ({} branches available)",
                app.workspace_root.display(),
                app.branches.len()
            ),
            Style::default().fg(THEME.text_dim),
        )),
        Line::from("Pressing [Enter] on a branch will display a loading modal, run git checkout & pull, then rescan variants and binaries."),
        Line::from(Span::styled(
            "Controls: [Space] Select Branch | [Enter / n] Confirm & Switch | [p] Previous Step",
            Style::default().fg(THEME.text_muted),
        )),
    ])
    .block(
        Block::default()
            .title(" Target Branch Selection Info ")
            .title_style(Style::default().fg(mode_color).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(mode_color)),
    );
    f.render_widget(prompt_box, chunks[1]);
}

fn branch_row<'a>(
    row_idx: usize,
    cursor: &usize,
    name: &'a str,
    tags: &[&str],
    build_from_source: bool,
    is_selected: bool,
) -> ListItem<'a> {
    let is_cursor = row_idx == *cursor;

    let radio = if is_selected {
        Span::styled(
            "(●) ",
            Style::default()
                .fg(THEME.mint)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        Span::styled("( ) ", Style::default().fg(THEME.text_muted))
    };

    let title_style = if is_cursor {
        Style::default()
            .fg(THEME.cyan)
            .add_modifier(Modifier::BOLD)
    } else if is_selected {
        Style::default()
            .fg(THEME.text_bright)
            .add_modifier(Modifier::BOLD)
    } else if row_idx == 0 {
        Style::default().fg(THEME.text_bright)
    } else {
        Style::default().fg(THEME.text_dim)
    };

    let mut spans = vec![radio, Span::styled(name, title_style)];
    for t in tags {
        let tag_style = match *t {
            "current" => Style::default().fg(THEME.mint).bg(THEME.bg_badge).add_modifier(Modifier::BOLD),
            _ => Style::default().fg(THEME.text_dim).bg(THEME.bg_badge),
        };
        spans.push(Span::raw(" "));
        spans.push(Span::styled(format!(" {t} "), tag_style));
    }

    if is_selected {
        if row_idx == 0 {
            spans.push(Span::raw(" "));
            spans.push(Span::styled(
                " [SELECTED: PRE-BUILT ONLY] ",
                Style::default()
                    .fg(THEME.mint)
                    .bg(THEME.bg_badge)
                    .add_modifier(Modifier::BOLD),
            ));
        } else {
            spans.push(Span::raw(" "));
            spans.push(Span::styled(
                " [★ TARGET BRANCH] ",
                Style::default()
                    .fg(THEME.amber)
                    .bg(THEME.bg_badge)
                    .add_modifier(Modifier::BOLD),
            ));
            if build_from_source {
                spans.push(Span::styled(
                    " (will checkout & rebuild)",
                    Style::default().fg(THEME.cyan),
                ));
            }
        }
    } else if is_cursor {
        spans.push(Span::raw(" "));
        spans.push(Span::styled(
            " [Space: Chọn Target] ",
            Style::default()
                .fg(THEME.mint)
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
