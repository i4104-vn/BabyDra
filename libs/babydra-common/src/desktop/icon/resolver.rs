//! System icon resolving logic and asset generation.

use gdk4::Texture;
use gdk_pixbuf::Pixbuf;

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
    let logo_dir = crate::desktop::config::get_babydra_config_dir();
    let logo_path = logo_dir.join("logo.png");
    if !logo_path.exists() {
        let _ = std::fs::create_dir_all(&logo_dir);
        const PNG_BYTES: &[u8] = include_bytes!("../logo.png");
        let _ = std::fs::write(&logo_path, PNG_BYTES);
    }
    logo_path
}

pub fn get_system_or_file_icon(icon_path_or_name: &str, default_fallback: &str) -> gtk4::Image {
    if icon_path_or_name.is_empty() {
        return gtk4::Image::from_icon_name(default_fallback);
    }
    
    if icon_path_or_name.starts_with('/') {
        return gtk4::Image::from_file(icon_path_or_name);
    }
    
    let mut clean_name = icon_path_or_name.to_string();
    for ext in &[".png", ".svg", ".xpm", ".jpg", ".jpeg", ".gif"] {
        if clean_name.to_lowercase().ends_with(ext) {
            clean_name = clean_name[..clean_name.len() - ext.len()].to_string();
            break;
        }
    }

    let display = gdk4::Display::default();
    let has_icon = if let Some(disp) = display.as_ref() {
        let theme = gtk4::IconTheme::for_display(disp);
        theme.has_icon(&clean_name)
    } else {
        false
    };

    if has_icon {
        gtk4::Image::from_icon_name(&clean_name)
    } else {
        let lower_name = clean_name.to_lowercase();
        let apps = crate::desktop::apps::find_desktop_apps();
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
            gtk4::Image::from_icon_name(&clean_resolved)
        } else if let Some(ref disp) = display {
            let theme = gtk4::IconTheme::for_display(disp);
            if theme.has_icon(default_fallback) {
                gtk4::Image::from_icon_name(default_fallback)
            } else {
                gtk4::Image::from_icon_name("image-missing")
            }
        } else {
            gtk4::Image::from_icon_name("image-missing")
        }
    }
}
