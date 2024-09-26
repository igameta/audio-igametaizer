use std::{io::Write, path::Path, path::PathBuf, fs};

use clap::Parser;
use serde::Serialize;
use serde_json::json;
use mp3_metadata::AudioTag;

#[derive(Debug, Parser)]
#[clap(author, about, version)]
// Audio Metadata Parser into JSON
struct Args {
    /// input file path
    #[arg(required(true))]
    path: String,

    /// output file path
    #[arg(required(false))]
    outpath: Option<String>,

    /// output filename
    #[arg(short, long, default_value = "output.json")]
    filename: String,
}

#[derive(Serialize)]
struct Metadata {
    album: String,  // Album
    title: String,  // TrackTitle
    artist: String, // Artist
    time: u32,
    rating: u32, // Rating
}

fn main() {
    let args = Args::parse();
    let mut parsed_tag: Vec<Metadata> = Vec::new();

    // ファイルパスを格納
    let dir = fs::read_dir(&args.path).unwrap();
    let mut file_path: Vec<PathBuf> = Vec::new();
    for item in dir {
        file_path.push(item.unwrap().path());
    }

    for file in file_path {}

    // jsonに書き出し
    match out_json(parsed_tag) {
        Ok(_) => println!("Success"),
        Err(e) => println!("Error: {}", e),
    }
}

fn get_tags(file_path: PathBuf) -> Metadata {
    let file = std::fs::File::open(file_path).expect("file not found");
    let mut data = Metadata {
        album: "".to_string(),
        title: "".to_string(),
        artist: "".to_string(),
        time: 0,
        rating: 0,
    };

    match mp3_metadata::read_from_file(file) {
        Ok(meta) => {
            let tag = meta.tag.unwrap();
        },
        Err(e) => println!("Error: {}", e),
    }

}

fn out_json(target: Vec<Metadata>) -> std::io::Result<()> {
    let serialized = serde_json::to_string(&target).unwrap();
    let mut file = std::fs::File::create("output.json")?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}
