use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::models::{InstallState, WizardStep};
use crate::ui::THEME;

pub fn draw_footer_shortcuts(f: &mut Frame, app: &App, area: Rect) {
    if area.height < 1 || area.width < 20 {
        return;
    }

    let (step_shortcuts, title) = match app.current_step {
        WizardStep::Welcome => (
            vec![
                ("Enter / →", "Get Started", THEME.mint),
                ("Tab / n", "Next Step", THEME.cyan),
                ("1-6", "Jump Step", THEME.purple),
            ],
            " Shortcuts (Welcome) ",
        ),
        WizardStep::SourceBranch => (
            vec![
                ("Space", "Select", THEME.mint),
                ("Enter", "Switch & Next", THEME.cyan),
                ("↑ / ↓", "Move", THEME.blue),
                ("← / →", "Step", THEME.amber),
            ],
            " Shortcuts (Branch) ",
        ),
        WizardStep::Binaries => (
            vec![
                ("Space", "Toggle", THEME.mint),
                ("a", "Toggle All", THEME.cyan),
                ("Enter / →", "Next Step", THEME.mint),
                ("←", "Back", THEME.amber),
                ("s", "Binary Path", THEME.purple),
                ("r", "Rescan", THEME.purple),
            ],
            " Shortcuts (Binaries) ",
        ),
        WizardStep::VariantSelection => (
            vec![
                ("Space", "Select", THEME.mint),
                ("Enter / →", "Apply & Next", THEME.cyan),
                ("↑ / ↓", "Move", THEME.blue),
                ("←", "Back", THEME.amber),
            ],
            " Shortcuts (Themes) ",
        ),
        WizardStep::ExecuteInstall => (
            if app.show_confirm_dialog {
                vec![
                    ("Enter", "Start Installation", THEME.mint),
                    ("Esc / ←", "Back", THEME.amber),
                ]
            } else if app.install_state == InstallState::Installing {
                vec![
                    ("↑ / ↓", "Scroll", THEME.blue),
                    ("PgUp/Dn", "Fast", THEME.blue),
                    ("c", "Clear", THEME.amber),
                    ("g / G", "Top/End", THEME.purple),
                ]
            } else {
                vec![
                    ("Enter / i", "Start Install", THEME.mint),
                    ("← / p", "Back to Setup", THEME.amber),
                ]
            },
            " Shortcuts (Install) ",
        ),
        WizardStep::Summary => (
            vec![
                ("Enter / q", "Finish & Exit", THEME.mint),
                ("←", "Back", THEME.amber),
                ("1-5", "Review Step", THEME.purple),
            ],
            " Shortcuts (Summary) ",
        ),
    };

    if area.width < 75 {
        // Compact single-block footer
        let mut spans = Vec::new();
        spans.push(Span::raw(" "));
        for (i, (key, desc, col)) in step_shortcuts.into_iter().enumerate() {
            if i > 0 {
                spans.push(Span::styled(" ", Style::default()));
            }
            spans.push(Span::styled(
                format!(" {} ", key),
                Style::default()
                    .fg(col)
                    .bg(THEME.bg_badge)
                    .add_modifier(Modifier::BOLD),
            ));
            spans.push(Span::styled(
                format!(" {} ", desc),
                Style::default().fg(THEME.text_body),
            ));
        }

        let p = Paragraph::new(Line::from(spans)).block(
            Block::default()
                .title(title)
                .title_style(THEME.title_cyan())
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(THEME.cyan)),
        );
        f.render_widget(p, area);
    } else {
        // Dual block footer (Context Shortcuts on Left, Global on Right)
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(35), Constraint::Length(34)])
            .split(area);

        let mut left_spans = Vec::new();
        left_spans.push(Span::raw(" "));
        for (i, (key, desc, col)) in step_shortcuts.into_iter().enumerate() {
            if i > 0 {
                left_spans.push(Span::styled(" ", Style::default()));
            }
            left_spans.push(Span::styled(
                format!(" {} ", key),
                Style::default()
                    .fg(col)
                    .bg(THEME.bg_badge)
                    .add_modifier(Modifier::BOLD),
            ));
            left_spans.push(Span::styled(
                format!(" {} ", desc),
                Style::default().fg(THEME.text_body),
            ));
        }

        let left_para = Paragraph::new(Line::from(left_spans)).block(
            Block::default()
                .title(title)
                .title_style(THEME.title_cyan())
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(THEME.cyan)),
        );
        f.render_widget(left_para, chunks[0]);

        let right_spans = vec![
            Span::styled(
                " ? ",
                Style::default()
                    .fg(THEME.purple)
                    .bg(THEME.bg_badge)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Help ", Style::default().fg(THEME.text_body)),
            Span::styled("│ ", Style::default().fg(THEME.border_normal)),
            Span::styled(
                " q ",
                Style::default()
                    .fg(THEME.rose)
                    .bg(THEME.bg_badge)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Quit ", Style::default().fg(THEME.text_body)),
            Span::styled("│ ", Style::default().fg(THEME.border_normal)),
            Span::styled(
                " 1-6 ",
                Style::default()
                    .fg(THEME.amber)
                    .bg(THEME.bg_badge)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Jump ", Style::default().fg(THEME.text_body)),
        ];

        let right_para = Paragraph::new(Line::from(right_spans))
            .alignment(Alignment::Right)
            .block(
                Block::default()
                    .title(" Global ")
                    .title_style(THEME.title_amber())
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(THEME.border_normal)),
            );
        f.render_widget(right_para, chunks[1]);
    }
}
