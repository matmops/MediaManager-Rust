use medman::cli::CliArguments;
use medman::cli::Command;
use medman::error::ComError;
use medman::scan::scan;
use medman::search::search;
use medman::write2::{write2md, write_2_playlist};
use std::fs;
use std::fs::OpenOptions;
use std::io;
extern crate serde;
extern crate serde_json;

fn main() -> Result<(), ComError> {
    let args = CliArguments::new();
    //println!("{:?}", args);
    loop {
        match &args.action {
            Command::Scan => {
                let music_files = scan(args.path());
                // Serialization
                let json: String = serde_json::to_string_pretty(&music_files).unwrap();
                fs::write("scan.json", &json).expect("Unable to write file");
                let file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open("scan.json");
                match file {
                    Ok(file) => file,
                    Err(_) => panic!("cannot create or open data file"),
                };
                println!("Check scan file!");
                break;
            }
            Command::Search => {
                println!("\nkeyword : ");

                let mut keyword = String::new();
                let mut format = String::new();
                io::stdin()
                    .read_line(&mut keyword)
                    .expect("Failed to read line");
                println!("\nsearch by : ");
                io::stdin()
                    .read_line(&mut format)
                    .expect("Failed to read line");
                let m = search(keyword.trim().to_string(), format.trim().to_string());
                let j = write2md(m);
                if j == false {
                    println!("Cannot find media");
                    break;
                }
                println!("Scan success, check search file!");
                break;
                //println!("{:?}", m);
            }
            Command::Write2playlist => {
                println!("\n Name of playlist:");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");
                write_2_playlist(args.path(), name.trim().to_string());
                break;
            }
        }
    }

    //println!("{:?}", ds_music_files);
    Ok(())
}
