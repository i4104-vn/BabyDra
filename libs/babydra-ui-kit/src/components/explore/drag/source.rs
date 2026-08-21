use babydra_core::load_cropped_square;
use gtk4::gdk::FileList;
use gtk4::prelude::*;
use std::cell::Cell;
use std::path::PathBuf;
use std::rc::Rc;

/// Creates a DragSource. `get_targets` is a closure called at drag-begin time to
/// collect the paths that should be dragged. Using a closure lets callers snapshot
/// selection state before GTK4's FlowBox/ListBox automatically deselects items.
pub fn create_drag_source(
    preview_path: &PathBuf,
    icon_name: &str,
    is_dragging: Rc<Cell<bool>>,
    get_targets: impl Fn() -> Vec<PathBuf> + 'static,
) -> gtk4::DragSource {
    let drag_source = gtk4::DragSource::new();
    drag_source.set_actions(
        gtk4::gdk::DragAction::COPY
            | gtk4::gdk::DragAction::MOVE
            | gtk4::gdk::DragAction::ASK,
    );

    let is_drag_begin = is_dragging.clone();
    drag_source.connect_drag_begin(move |_, _| {
        is_drag_begin.set(true);
    });

    let is_drag_end = is_dragging.clone();
    drag_source.connect_drag_end(move |_, _, _| {
        is_drag_end.set(false);
    });

    drag_source.connect_prepare(move |_, _, _| {
        let targets = get_targets();

        if targets.is_empty() {
            return None;
        }

        let gio_files: Vec<gtk4::gio::File> = targets
            .iter()
            .map(|p| gtk4::gio::File::for_path(p))
            .collect();

        // 1. GdkFileList provider for GTK4 apps
        let file_list = FileList::from_array(&gio_files);
        let file_provider = gtk4::gdk::ContentProvider::for_value(&file_list.to_value());

        // 2. RFC 2483 text/uri-list for web browsers, electron apps (Discord, Telegram, VS Code), and external apps
        let mut uri_list = String::new();
        for f in &gio_files {
            uri_list.push_str(&f.uri());
            uri_list.push_str("\r\n");
        }
        let uri_bytes = glib::Bytes::from(uri_list.as_bytes());
        let uri_provider = gtk4::gdk::ContentProvider::for_bytes("text/uri-list", &uri_bytes);

        // 3. x-special/gnome-copied-files for Linux desktop file managers
        let mut gnome_content = "copy\n".to_string();
        for f in &gio_files {
            gnome_content.push_str(&f.uri());
            gnome_content.push('\n');
        }
        let gnome_bytes = glib::Bytes::from(gnome_content.as_bytes());
        let gnome_provider =
            gtk4::gdk::ContentProvider::for_bytes("x-special/gnome-copied-files", &gnome_bytes);

        // 4. Plain text for terminals and editors
        let text_plain = targets
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join("\n");
        let text_bytes = glib::Bytes::from(text_plain.as_bytes());
        let text_provider =
            gtk4::gdk::ContentProvider::for_bytes("text/plain;charset=utf-8", &text_bytes);

        let union_provider = gtk4::gdk::ContentProvider::new_union(&[
            file_provider,
            uri_provider,
            gnome_provider,
            text_provider,
        ]);

        Some(union_provider)
    });

    let has_preview = if let Some(ext) = preview_path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        matches!(
            ext_str.as_str(),
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "svg"
        )
    } else {
        false
    };

    if has_preview {
        if let Ok(pixbuf) = load_cropped_square(preview_path, 85) {
            let texture = gtk4::gdk::Texture::for_pixbuf(&pixbuf);
            drag_source.set_icon(Some(&texture), 42, 42);
            return drag_source;
        }
    }

    let display = gtk4::gdk::Display::default().unwrap();
    let icon_theme = gtk4::IconTheme::for_display(&display);
    let paintable = icon_theme.lookup_icon(
        icon_name,
        &[],
        48,
        1,
        gtk4::TextDirection::Ltr,
        gtk4::IconLookupFlags::empty(),
    );
    drag_source.set_icon(Some(&paintable), 24, 24);

    drag_source
}
