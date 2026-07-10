use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, ScrolledWindow, Align};
use std::path::{Path, PathBuf};
use std::fs;
use std::cell::RefCell;

pub struct PreviewPanel {
    container: ScrolledWindow,
    lbl_content: Label,
    lbl_status: Label,
    current_file: RefCell<Option<PathBuf>>,
    watcher: RefCell<Option<babydra_common::FileWatcher>>,
}

impl PreviewPanel {
    pub fn new() -> Self {
        let container = ScrolledWindow::new();
        container.set_css_classes(&["preview-panel"]);
        container.set_hexpand(true);
        container.set_vexpand(true);

        let vbox = Box::new(Orientation::Vertical, 6);
        vbox.set_hexpand(true);
        vbox.set_vexpand(true);
        container.set_child(Some(&vbox));

        let lbl_status = Label::new(Some("Select a file to preview"));
        lbl_status.set_css_classes(&["dim-label"]);
        lbl_status.set_margin_bottom(6);
        vbox.append(&lbl_status);

        // Scrolled inner container for the label text content
        let scroll_content = ScrolledWindow::new();
        scroll_content.set_hexpand(true);
        scroll_content.set_vexpand(true);
        vbox.append(&scroll_content);

        let lbl_content = Label::builder()
            .use_markup(true)
            .wrap(true)
            .halign(Align::Start)
            .valign(Align::Start)
            .selectable(true)
            .margin_top(6)
            .margin_bottom(6)
            .margin_start(6)
            .margin_end(6)
            .build();
        scroll_content.set_child(Some(&lbl_content));

        Self {
            container,
            lbl_content,
            lbl_status,
            current_file: RefCell::new(None),
            watcher: RefCell::new(None),
        }
    }

    pub fn widget(&self) -> &ScrolledWindow {
        &self.container
    }

    pub fn clear(&self) {
        self.lbl_status.set_text("Select a file to preview");
        self.lbl_content.set_text("");
        self.current_file.replace(None);
        self.watcher.replace(None);
    }

    pub fn show_preview(&self, path: &Path) {
        let filename = path.file_name().unwrap_or_default().to_string_lossy();
        self.lbl_status.set_text(&format!("Previewing: {}", filename));
        self.current_file.replace(Some(path.to_path_buf()));

        // Read file contents (limit to 1MB to avoid freezing)
        if let Ok(metadata) = fs::metadata(path) {
            if metadata.len() > 1024 * 1024 {
                self.lbl_content.set_text("[File is too large to preview (> 1MB)]");
                self.watcher.replace(None);
                return;
            }
        }

        if let Ok(content) = fs::read_to_string(path) {
            let is_markdown = path.extension().and_then(|e| e.to_str()) == Some("md");
            if is_markdown {
                let parsed = render_markdown_to_pango(&content);
                self.lbl_content.set_markup(&parsed);
            } else {
                let escaped = glib::markup_escape_text(&content);
                self.lbl_content.set_markup(&escaped);
            }
        } else {
            self.lbl_content.set_text("[Failed to load file contents / Binary file]");
        }

        // Setup unbounded channel for thread-safe UI updates
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
        
        let label_clone = self.lbl_content.clone();
        let path_p = path.to_path_buf();
        glib::spawn_future_local(async move {
            while let Some(new_content) = rx.recv().await {
                let is_md = path_p.extension().and_then(|e| e.to_str()) == Some("md");
                if is_md {
                    let parsed = render_markdown_to_pango(&new_content);
                    label_clone.set_markup(&parsed);
                } else {
                    let escaped = glib::markup_escape_text(&new_content);
                    label_clone.set_markup(&escaped);
                }
            }
        });

        // Setup FileWatcher for Hot Reload
        let path_clone = path.to_path_buf();
        let watcher_res = babydra_common::FileWatcher::new(path_clone.clone(), move |_event| {
            if let Ok(new_content) = fs::read_to_string(&path_clone) {
                let _ = tx.send(new_content);
            }
        });

        if let Ok(watcher) = watcher_res {
            self.watcher.replace(Some(watcher));
        } else {
            self.watcher.replace(None);
        }
    }
}

fn render_markdown_to_pango(markdown: &str) -> String {
    let options = pulldown_cmark::Options::empty();
    let parser = pulldown_cmark::Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    // Simple parser translating basic HTML tags to Pango markup tags
    html_output
        .replace("<h1>", "\n<b><span size=\"xx-large\">")
        .replace("</h1>", "</span></b>\n")
        .replace("<h2>", "\n<b><span size=\"x-large\">")
        .replace("</h2>", "</span></b>\n")
        .replace("<h3>", "\n<b><span size=\"large\">")
        .replace("</h3>", "</span></b>\n")
        .replace("<p>", "")
        .replace("</p>", "\n")
        .replace("<strong>", "<b>")
        .replace("</strong>", "</b>")
        .replace("<em>", "<i>")
        .replace("</em>", "</i>")
        .replace("<pre><code>", "\n<span face=\"monospace\" background=\"#2e2e2e\">")
        .replace("</code></pre>", "</span>\n")
        .replace("<code>", "<span face=\"monospace\" background=\"#2e2e2e\"> ")
        .replace("</code>", " </span>")
        .replace("<ul>", "")
        .replace("</ul>", "")
        .replace("<li>", " • ")
        .replace("</li>", "\n")
}
