use crate::musicfile::MusicFile;
use std::fs;
use std::path::Path;
extern crate m3u;
extern crate serde_json;

/// A function that converts SystemTime into a DateTime using UNIX EPOCH format
/// Input in SystemTime and output is DateTime
/// This function is recquired to be able to search into the metadata<SystemTime>
pub fn write2md(music_files: Vec<MusicFile>) -> bool {
  if music_files.is_empty() {
    false;
  }
  let m_json: String = serde_json::to_string_pretty(&music_files).unwrap();
  // Deserialization
  let _deser_music_files: Vec<MusicFile> = serde_json::from_str(&m_json).unwrap();
  fs::write("search.md", &m_json).expect("Unable to write md file");

  true
}

/// Create a playlist with the path
pub fn write_2_playlist(path: &Path, name: String) {
  let mut playlist_name: String = name.to_owned();
  let extension: &str = ".m3u";
  playlist_name.push_str(extension);
  
  // Create a multimedia media playlist.
  let playlist = vec![m3u::path_entry(&path)];

  // Write the playlist to a file.
  {
    let mut file = std::fs::File::create(playlist_name).unwrap();
    let mut writer = m3u::Writer::new(&mut file);
    for entry in &playlist {
      writer.write_entry(entry).unwrap();
    }
  }
}
