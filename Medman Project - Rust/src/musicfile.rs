use immeta::Dimensions;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct MusicFile {
    pub path: PathBuf,
    pub date_created: SystemTime,
    pub date_modified: SystemTime,

    pub title: String,
    pub album: String,
    pub artist: String,
    pub year: String,
}

pub struct JpegFile {
    pub path: PathBuf,
    pub date_created: SystemTime,
    pub date_modified: SystemTime,

    pub file_name: String,
    pub dimensions: Dimensions,
}

impl MusicFile {
    pub fn new(
        path: &Path,
        date_created: SystemTime,
        date_modified: SystemTime,
        title: String,
        album: String,
        artist: String,
        year: String,
    ) -> MusicFile {
        
        MusicFile {
            path: path.to_path_buf(),
            date_created,
            date_modified,
            title,
            album,
            artist,
            year,
        }
    }
}

impl JpegFile {
    pub fn new(
        path: &Path,
        file_name : String,
        date_created: SystemTime,
        date_modified: SystemTime,
        dimensions: Dimensions,
        
    ) -> JpegFile {
        
        JpegFile {
            path: path.to_path_buf(),
            file_name,
            date_created,
            date_modified,
            dimensions,
        }
    }
}
