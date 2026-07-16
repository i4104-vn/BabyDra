//! System icon resolving logic and asset generation.

use gdk4::Texture;
use gdk_pixbuf::Pixbuf;
use std::collections::HashMap;
use std::sync::OnceLock;
use std::path::PathBuf;
use gio::prelude::*;

pub fn get_icon_from_svg(svg_content: &str, size: i32) -> gtk4::Image {
    let bytes = glib::Bytes::from(svg_content.as_bytes());
    let stream = gio::MemoryInputStream::from_bytes(&bytes);
    
    let pixbuf = Pixbuf::from_stream_at_scale(
        &stream,
        size,
        size,
        true,
        gio::Cancellable::NONE
    );

    match pixbuf {
        Ok(pb) => {
            let texture = Texture::for_pixbuf(&pb);
            let img = gtk4::Image::from_paintable(Some(&texture));
            img.set_pixel_size(size);
            img
        }
        Err(_) => {
            gtk4::Image::from_icon_name("image-missing")
        }
    }
}

pub fn get_logo_png(size: i32) -> gtk4::Image {
    const PNG_BYTES: &[u8] = include_bytes!("../logo.png");
    let bytes = glib::Bytes::from(PNG_BYTES);
    let stream = gio::MemoryInputStream::from_bytes(&bytes);
    
    let pixbuf = Pixbuf::from_stream_at_scale(
        &stream,
        size,
        size,
        true,
        gio::Cancellable::NONE
    );

    match pixbuf {
        Ok(pb) => {
            let texture = Texture::for_pixbuf(&pb);
            let img = gtk4::Image::from_paintable(Some(&texture));
            img.set_pixel_size(size);
            img
        }
        Err(_) => {
            gtk4::Image::from_icon_name("image-missing")
        }
    }
}

pub fn get_logo_path() -> std::path::PathBuf {
    std::path::PathBuf::from("/usr/share/babydra/logo.png")
}

static ICON_PATH_CACHE: OnceLock<HashMap<String, PathBuf>> = OnceLock::new();

fn get_theme_dirs(theme_name: &str) -> Vec<PathBuf> {
    let mut resolved_themes = Vec::new();
    let mut themes_to_resolve = vec![theme_name.to_string()];
    
    let home = glib::home_dir();
    let local_icons = home.join(".local/share/icons");
    let system_icons = PathBuf::from("/usr/share/icons");

    let mut visited = std::collections::HashSet::new();

    while let Some(theme) = themes_to_resolve.pop() {
        if theme.is_empty() || visited.contains(&theme) {
            continue;
        }
        visited.insert(theme.clone());

        let local_theme_dir = local_icons.join(&theme);
        let system_theme_dir = system_icons.join(&theme);

        let mut found_dir = None;
        if local_theme_dir.exists() {
            found_dir = Some(local_theme_dir);
        } else if system_theme_dir.exists() {
            found_dir = Some(system_theme_dir);
        }

        if let Some(dir) = found_dir {
            resolved_themes.push(dir.clone());

            let index_path = dir.join("index.theme");
            if index_path.exists() {
                if let Ok(file) = std::fs::File::open(index_path) {
                    use std::io::{BufRead, BufReader};
                    let reader = BufReader::new(file);
                    for line in reader.lines().flatten() {
                        let line = line.trim();
                        if line.starts_with("Inherits=") {
                            let inherits_str = line["Inherits=".len()..].trim();
                            for inherited in inherits_str.split(',') {
                                let inherited = inherited.trim().to_string();
                                if !inherited.is_empty() && !visited.contains(&inherited) {
                                    themes_to_resolve.push(inherited);
                                }
                            }
                            break;
                        }
                    }
                }
            }
        }
    }

    resolved_themes
}

fn populate_theme_cache(theme_name: &str) -> HashMap<String, PathBuf> {
    let mut cache = HashMap::new();
    let mut search_dirs = get_theme_dirs(theme_name);
    
    search_dirs.push(PathBuf::from("/usr/share/pixmaps"));

    for base_dir in search_dirs {
        if !base_dir.exists() {
            continue;
        }
        let mut dirs_to_visit = vec![base_dir];
        while let Some(dir) = dirs_to_visit.pop() {
            if let Ok(entries) = std::fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        let path_str = path.to_string_lossy();
                        if path_str.contains("symbolic") {
                            continue;
                        }
                        dirs_to_visit.push(path);
                    } else if path.is_file() {
                        if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                                if ext == "svg" || ext == "png" {
                                    cache.entry(stem.to_string()).or_insert(path);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    cache
}

pub fn get_resolved_icon_path(icon_name: &str) -> Option<PathBuf> {
    let cache = ICON_PATH_CACHE.get_or_init(|| {
        let gsettings = gio::Settings::new("org.gnome.desktop.interface");
        let theme_name = gsettings.string("icon-theme");
        let theme_name = theme_name.trim().to_string();
        populate_theme_cache(&theme_name)
    });
    cache.get(icon_name).cloned()
}

pub fn set_system_or_file_icon(img: &gtk4::Image, icon_path_or_name: &str, default_fallback: &str) {
    if icon_path_or_name.is_empty() {
        img.set_icon_name(Some(default_fallback));
        return;
    }
    
    if icon_path_or_name.starts_with('/') {
        img.set_from_file(Some(icon_path_or_name));
        return;
    }
    
    let mut clean_name = icon_path_or_name.to_string();
    for ext in &[".png", ".svg", ".xpm", ".jpg", ".jpeg", ".gif"] {
        if clean_name.to_lowercase().ends_with(ext) {
            clean_name = clean_name[..clean_name.len() - ext.len()].to_string();
            break;
        }
    }

    if let Some(resolved_path) = get_resolved_icon_path(&clean_name) {
        img.set_from_file(Some(resolved_path.to_string_lossy().as_ref()));
        return;
    }

    let display = gdk4::Display::default();
    let has_icon = if let Some(disp) = display.as_ref() {
        let theme = gtk4::IconTheme::for_display(disp);
        theme.has_icon(&clean_name)
    } else {
        false
    };

    if has_icon {
        img.set_icon_name(Some(&clean_name));
    } else {
        let lower_name = clean_name.to_lowercase();
        let apps = babydra_common::services::apps::find_desktop_apps();
        let mut resolved_icon = None;
        for app in apps {
            if app.name.to_lowercase() == lower_name {
                if let Some(ref app_icon) = app.icon {
                    resolved_icon = Some(app_icon.clone());
                }
                break;
            }
        }
        
        if let Some(icon_name) = resolved_icon {
            let mut clean_resolved = icon_name;
            for ext in &[".png", ".svg", ".xpm", ".jpg", ".jpeg", ".gif"] {
                if clean_resolved.to_lowercase().ends_with(ext) {
                    clean_resolved = clean_resolved[..clean_resolved.len() - ext.len()].to_string();
                    break;
                }
            }
            img.set_icon_name(Some(&clean_resolved));
        } else if let Some(ref disp) = display {
            let theme = gtk4::IconTheme::for_display(disp);
            if theme.has_icon(default_fallback) {
                img.set_icon_name(Some(default_fallback));
            } else {
                img.set_icon_name(Some("image-missing"));
            }
        } else {
            img.set_icon_name(Some("image-missing"));
        }
    }
}

pub fn get_system_or_file_icon(icon_path_or_name: &str, default_fallback: &str) -> gtk4::Image {
    let img = gtk4::Image::new();
    set_system_or_file_icon(&img, icon_path_or_name, default_fallback);
    img
}
