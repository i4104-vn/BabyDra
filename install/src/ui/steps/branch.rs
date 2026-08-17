use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

/// Draws step 2: choose the git branch to install from.
///
/// Row 0 is always "pre-built binaries only". Rows 1..=N are the actual
/// branches discovered from the repository (local + remote). Selecting a
/// branch switches the installer into "build from source" mode: at install
/// time it will checkout the branch, pull, run `cargo build --release` and
/// then copy the freshly built binaries.
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
            .title(" 2. Install Source — Pick a Git Branch [Up/Down: Move | Space: Select | Enter: Next] ")
            .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Yellow)),
    );
    f.render_widget(list, chunks[0]);

    let (mode_color, mode_text) = if build_from_source {
        (
            Color::Cyan,
            format!(
                "Build from source: branch '{}' will be checked out, pulled and rebuilt with cargo.",
                app.selected_branch
            ),
        )
    } else {
        (
            Color::Green,
            "Pre-built mode: binaries are copied directly from the source folder — no rebuild."
                .to_string(),
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
                "Detected {} branch(es) in {}",
                app.branches.len(),
                app.workspace_root.display()
            ),
            Style::default().fg(Color::DarkGray),
        )),
        Line::from("Picking a branch runs: git fetch --prune -> git checkout <branch> -> git pull -> cargo build --release."),
        Line::from(Span::styled(
            "Press [Space] to select, [Enter/n] to continue, [p] for previous step.",
            Style::default().fg(Color::DarkGray),
        )),
    ])
    .block(
        Block::default()
            .title(" Install Mode Info ")
            .title_style(
                Style::default()
                    .fg(mode_color)
                    .add_modifier(Modifier::BOLD),
            )
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
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        Span::styled("( ) ", Style::default().fg(Color::DarkGray))
    };

    let title_style = if is_cursor {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
    } else if row_idx == 0 {
        Style::default().fg(Color::White)
    } else {
        Style::default().fg(Color::Cyan)
    };

    let mut spans = vec![radio, Span::styled(name, title_style)];
    for t in tags {
        let tag_style = match *t {
            "current" => Style::default().fg(Color::Green),
            _ => Style::default().fg(Color::DarkGray),
        };
        spans.push(Span::raw("  "));
        spans.push(Span::styled(format!("[{t}]"), tag_style));
    }
    if is_selected && build_from_source && row_idx > 0 {
        spans.push(Span::styled(
            "  will rebuild from source",
            Style::default().fg(Color::Cyan),
        ));
    }

    ListItem::new(Line::from(spans)).style(if is_cursor {
        Style::default().bg(Color::Rgb(25, 30, 48))
    } else {
        Style::default()
    })
}
