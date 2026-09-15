//! Album artwork payload types for the media player island feature.

/// (art_url, fallback_icon_name, load_result)
pub type ArtPayload = (String, String, Result<Vec<u8>, ()>);
