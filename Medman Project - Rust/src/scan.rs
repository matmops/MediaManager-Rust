//use crate::musicfile::JpegFile;
use id3::Tag;
//use immeta::*;
use std::fs;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

use crate::musicfile::MusicFile;

pub const SUPPORTED_EXTENSIONS: [&str; 3] = ["mp3","jpeg","png"];
pub const SUPPORTED_IMG: [&str; 2] = ["jpeg","png"];
pub const SUPPORTED_MP3: [&str; 1] = ["mp3"];


/// This function in used to check if the directory is supported.
/// The parameter is an entry most likely to be a directory entry.
/// It returns a bool (true,false).
pub fn is_supported(entry: &DirEntry) -> bool {
    entry.path().is_file()
        && SUPPORTED_EXTENSIONS.contains(&entry.path().extension().unwrap().to_str().unwrap())
}

pub fn is_jpeg(entry: &DirEntry) -> bool {
    entry.path().is_file()
        && SUPPORTED_IMG.contains(&entry.path().extension().unwrap().to_str().unwrap())
}

pub fn is_mp3(entry: &DirEntry) -> bool {
    entry.path().is_file()
        && SUPPORTED_MP3.contains(&entry.path().extension().unwrap().to_str().unwrap())
}


/// This function scans a path that is given as a parameter.
/// It analyses all the metadata in the entered path.
/// This function is recursive. It returns a MusicFile with all the metadata.
pub fn scan(path: &Path) -> Vec<MusicFile> {
    let mut mediafiles: Vec<MusicFile> = Vec::new();
    let walker = WalkDir::new(path).into_iter();
    for entry in walker {
        let entry = entry.unwrap();
        if is_mp3(&entry) {
            if let Ok(meta) = fs::metadata(&entry.path()) {
                let tag = Tag::read_from_path(&entry.path()).expect("cannot read from path");
                let date_created = meta.created().unwrap();
                let date_modified = meta.modified().unwrap();

                let title = tag.title().unwrap_or("(no title)");
                let album = tag.album().unwrap_or("(no album)");
                let artist = tag.artist().unwrap_or("(no artist)");
                let year = tag
                    .year()
                    .map(|year| year.to_string())
                    .unwrap_or_else(|| "(no year)".to_string());

                mediafiles.push(MusicFile::new(
                    entry.path(),
                    date_created,
                    date_modified,
                    title.to_string(),
                    album.to_string(),
                    artist.to_string(),
                    year.to_string(),
                ));
            }
        }

/*        if is_Jpeg(&entry) {
            let mut mediafiles: Vec<JpegFile> = Vec::new();
            if let Ok(meta) = fs::metadata(&entry.path()) {
                let load = load_from_file(&entry.path()).expect("cannot read from path");
                let date_created = meta.created().unwrap();
                let date_modified = meta.modified().unwrap();
                let file_name = Path::new(&entry.path()).file_name().to_string();
                let dimensions = load.dimensions();
                

                mediafiles.push(JpegFile::new(
                    entry.path(),
                    file_name,
                    date_created,
                    date_modified,
                    dimensions,
                );
            }
        } */
    } 
    let json:String = serde_json::to_string_pretty(&mediafiles).unwrap();
    
    fs::write("scan.json", &json).expect("Unable to write file");
    mediafiles
}

/*
#[cfg(test)]
mod tests {
    #[test]
    fn scan() { <body> } */
