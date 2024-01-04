use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::crud::blog::Blog;
use regex::Regex;
use time::macros::date;

fn main() {
    let metadata_vec: Vec<std::path::PathBuf> = Vec::new();
    for element in std::path::Path::new(r"articles").read_dir().unwrap() {
        let path = element.unwrap().path();
        if let Some(extension) = path.extension() {
            if extension == "md" {
                // mdvec.push(path);
                let metadata = get_metadata(&path);
                print!("{:?}", metadata)
            }
        }
    }

    println!("{:?}", metadata_vec)
}

#[derive(Debug)]
pub struct Metadata {
    id: Option<i16>,
    title: Option<String>,
    date: Option<String>,
}

fn get_metadata(path: &PathBuf) -> Metadata {
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut metadata = Metadata {
        id: None,
        title: None,
        date: None,
    };

    let id_regex = Regex::new(r"% id: (.*)").unwrap();
    let title_regex = Regex::new(r"% title: (.*)").unwrap();
    let date_regex = Regex::new(r"% date: (.*)").unwrap();

    for line in reader.lines() {
        if let Ok(line_content) = line {
            if let Some(captures) = id_regex.captures(&line_content) {
                metadata.id = captures.get(1).map(|m| m.as_str().parse::<i16>().unwrap());
            }
            if let Some(captures) = title_regex.captures(&line_content) {
                metadata.title = captures.get(1).map(|m| m.as_str().to_string());
            }
            if let Some(captures) = date_regex.captures(&line_content) {
                metadata.date = captures.get(1).map(|m| m.as_str().to_string());
            }
        }
    }

    Blog::create_blog(
        metadata.id,
        metadata.title,
        String::from("summary"),
        String::from("body"),
        String::from("summary"),
        date!(2022 - 01 - 01),
        vec![String::from("tag1"), String::from("tag2")],
    );

    metadata
}
