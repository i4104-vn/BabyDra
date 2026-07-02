//! Controller logic coordinating real-time app search queries, web redirect options,
//! and spawning background indexing worker threads for file queries.

use gtk4::prelude::*;
use std::process::Command;
use super::file_search::{search_files, create_file_row};

mod render;

/// Populates the right-hand container with query results.
/// Displays a web browser search button and launches a background thread to match files.
pub fn populate_search_results(
    right_scroll: &gtk4::ScrolledWindow,
    query: &str,
    window: &gtk4::ApplicationWindow,
) {
    let query_trimmed = query.trim().to_string();

    if query_trimmed.is_empty() {
        let right_content = render::build_welcome_layout();
        right_scroll.set_child(Some(&right_content));
    } else {
        let (right_content, files_placeholder) = render::build_results_layout();
        let (browser_btn, _) = render::build_browser_search_button(&query_trimmed);

        let query_for_browser = query_trimmed.clone();
        let win_to_close = window.clone();
        browser_btn.connect_clicked(move |_| {
            let search_query = query_for_browser.replace(' ', "+");
            let url = format!("https://www.google.com/search?q={}", search_query);
            println!("Searching Google: {}", url);
            if let Err(e) = Command::new("xdg-open").arg(&url).spawn() {
                eprintln!("Failed to launch browser: {}", e);
            }
            win_to_close.close();
        });

        right_content.append(&browser_btn);
        right_scroll.set_child(Some(&right_content));

        let query_for_search = query_trimmed.clone();
        let (sender, receiver) = std::sync::mpsc::channel::<Vec<std::path::PathBuf>>();

        std::thread::spawn(move || {
            let results = search_files(&query_for_search);
            let _ = sender.send(results);
        });

        let files_placeholder_clone = files_placeholder.clone();
        let win_clone = window.clone();
        gtk4::glib::idle_add_local(move || {
            match receiver.try_recv() {
                Ok(matched_files) => {
                    if !matched_files.is_empty() {
                        let files_title = render::build_files_title();
                        files_placeholder_clone.append(&files_title);

                        for file_path in &matched_files {
                            let btn = create_file_row(file_path, &win_clone);
                            files_placeholder_clone.append(&btn);
                        }
                    }
                    gtk4::glib::ControlFlow::Break
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {
                    gtk4::glib::ControlFlow::Continue
                }
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    gtk4::glib::ControlFlow::Break
                }
            }
        });
    }
}

