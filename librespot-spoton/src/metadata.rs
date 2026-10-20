//! Track details supplied to LMS without a separate Web API request.
use librespot_metadata::audio::{AudioItem, UniqueFields};
use serde_json::json;

/// Convert already-loaded music metadata to the subset of the Web API track
/// shape used by LMS. No network requests are made; unsupported item types
/// retain the existing fallback path.
pub(crate) fn track_metadata(item: &AudioItem) -> Option<serde_json::Value> {
    let UniqueFields::Track { artists, album, album_artists, .. } = &item.unique_fields else {
        return None;
    };
    let mut names: Vec<&str> = Vec::new();
    let mut artist_details = Vec::new();
    for artist in artists.iter() {
        if !names.contains(&artist.name.as_str()) {
            names.push(&artist.name);
            artist_details.push(json!({"name": artist.name, "id": artist.id.to_id()}));
        }
    }
    if names.is_empty() {
        artist_details.extend(album_artists.iter().map(|name| json!({"name": name})));
    }
    Some(json!({
        "uri": item.track_id.to_uri(),
        "name": item.name,
        "duration_ms": item.duration_ms,
        "artists": artist_details,
        "album": {
            "name": album,
            "images": item.covers.iter().map(|cover| json!({
                "url": cover.url, "width": cover.width, "height": cover.height,
            })).collect::<Vec<_>>(),
        },
    }))
}


