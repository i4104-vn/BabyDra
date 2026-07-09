//! Media player status querying helpers.
//! Commands standard MPRIS active players using the `playerctl` command line utility.

use gdk_pixbuf::prelude::*;
use gtk4::prelude::*;

/// Loads local or file:// media album cover art, resizing to fit dimensions.
pub fn load_album_art(art_url: &str, size: i32) -> Option<gtk4::Widget> {
    if art_url.is_empty() {
        return None;
    }

    let local_path = if let Some(path_str) = art_url.strip_prefix("file://") {
        babydra_common::decode_uri(path_str)
    } else if art_url.starts_with('/') {
        art_url.to_string()
    } else {
        return None;
    } ;

    let pb = gdk_pixbuf::Pixbuf::from_file_at_scale(
        &local_path,
        size,
        size,
        true,
    ).ok()?;
    
    let texture = gdk4::Texture::for_pixbuf(&pb);
    let picture = gtk4::Picture::for_paintable(&texture);
    picture.set_size_request(pb.width(), pb.height());
    picture.set_content_fit(gtk4::ContentFit::Contain);
    Some(picture.upcast())
}

/// Parses and scales raw image data from memory buffers to build an album cover GTK widget.
pub fn load_album_art_from_bytes(bytes: &[u8], size: i32) -> Option<gtk4::Widget> {
    let loader = gdk_pixbuf::PixbufLoader::new();
    loader.write(bytes).ok()?;
    loader.close().ok()?;
    let pb = loader.pixbuf()?;
    
    let w = pb.width();
    let h = pb.height();
    if w <= 0 || h <= 0 {
        return None;
    }
    
    let scale_w = size as f64 / w as f64;
    let scale_h = size as f64 / h as f64;
    let scale = scale_w.min(scale_h);
    
    let dest_w = (w as f64 * scale) as i32;
    let dest_h = (h as f64 * scale) as i32;
    
    let scaled_pb = pb.scale_simple(dest_w, dest_h, gdk_pixbuf::InterpType::Bilinear)?;
    
    let texture = gdk4::Texture::for_pixbuf(&scaled_pb);
    let picture = gtk4::Picture::for_paintable(&texture);
    picture.set_size_request(dest_w, dest_h);
    picture.set_content_fit(gtk4::ContentFit::Contain);
    Some(picture.upcast())
}


