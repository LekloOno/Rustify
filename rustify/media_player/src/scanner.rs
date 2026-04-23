use std::fs::File;
use std::path::Path;
use walkdir::WalkDir;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::probe::Hint;
use symphonia::core::meta::{MetadataOptions, MetadataRevision};

#[derive(Debug, Clone)]
pub struct TrackInfo {
    pub path: String,
    pub title: Option<String>,
    pub artist: Option<String>,
}

pub fn scan_directory(dir_path: &str) -> Vec<TrackInfo> {
    let mut tracks = Vec::new();

    for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if ext == "mp3" || ext == "flac" || ext == "wav" {
                    if let Some(info) = extract_metadata(path) {
                        tracks.push(info);
                    }
                }
            }
        }
    }
    tracks
}

fn extract_metadata(path: &Path) -> Option<TrackInfo> {
    let file = File::open(path).ok()?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        hint.with_extension(ext);
    }
    
    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &Default::default(), &MetadataOptions::default())
        .ok()?;

    let mut format = probed.format;
    let mut title = None;
    let mut artist = None;

    if let Some(metadata) = format.metadata().current() {
        for tag in metadata.tags() {
            if tag.std_key == Some(symphonia::core::meta::StandardTagKey::TrackTitle) {
                title = Some(tag.value.to_string());
            }
            if tag.std_key == Some(symphonia::core::meta::StandardTagKey::Artist) {
                artist = Some(tag.value.to_string());
            }
        }
    }

    Some (TrackInfo {
        path: path.to_string_lossy().to_string(),
        title,
        artist,
    })
}