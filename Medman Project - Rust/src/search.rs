use crate::musicfile::MusicFile;
use chrono::{TimeZone,Utc,DateTime};
use std::fs::File;
use std::time::{SystemTime,UNIX_EPOCH};

/// A function that converts SystemTime into a DateTime using UNIX EPOCH format
/// Input in SystemTime and output is DateTime
/// This function is recquired to be able to search into the metadata<SystemTime>
pub fn system_time_to_date_time(t: SystemTime) -> DateTime<Utc> {
  let (sec, nsec) = match t.duration_since(UNIX_EPOCH) {
    Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
    Err(e) => {
      let dur = e.duration();
      let (sec, nsec) = (dur.as_secs() as i64, dur.subsec_nanos());
      if nsec == 0 {
        (-sec, 0)
      } else {
        (-sec - 1, 1_000_000_000 - nsec)
      }
    }
  };
  Utc.timestamp(sec, nsec)
}

/// A function that search in the music file catalog.
/// This function take 3 parameters.
/// It returns a specific music file. (to be modified)
pub fn search(keyword: String, format: String) -> Vec<MusicFile> {
  let mut data_result = Vec::<MusicFile>::new();

  let file = File::open("scan.json").unwrap();
  let music_files: Vec<MusicFile> = serde_json::from_reader(file).unwrap();

  for i in music_files {
    match format.trim() {
      "date_created" => {
        let datetime = DateTime::parse_from_str(&keyword, "%Y-%m-%d %H:%M:%S %z").unwrap();
        if datetime == system_time_to_date_time(i.date_created) {
          data_result.push(i);
        }
      }

      "date_modified" => {
        let datetime = DateTime::parse_from_str(&keyword, "%Y-%m-%d %H:%M:%S %z").unwrap(); //println!("please use (0000-00-00 00:00:00 UTC) format")
        if datetime == system_time_to_date_time(i.date_modified) {
          data_result.push(i);
        }
      }

      "title" => {
        if keyword == i.title {
          data_result.push(i);
        }
      }

      "album" => {
        if keyword == i.album {
          data_result.push(i);
        }
      }

      "artist" => {
        if keyword == i.artist {
          data_result.push(i);
        }
      }

      "year" => {
        if keyword == i.year {
          data_result.push(i);
        }
      }

      _ => panic!("try using this format: (title,artist,year,album,etc...)"),
    };
  }
  data_result
}

/*
#[cfg(test)]
mod tests {
    #[test]
    fn search() {
        assert_eq!("Time To Fall".to_string(),"title".to_string(), "{} {}", musicfile, ds_music_files);
    }
}
*/
