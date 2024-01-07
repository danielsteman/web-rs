use std::fs;

use regex::Regex;

fn main() {
    match fs::read_dir("./articles") {
        Ok(files) => {
            for file in files {
                let path = file.unwrap().path();
                let str_path = path.as_os_str();
                let content = fs::read_to_string(str_path)
                    .expect(format!("Error reading from {:?}", str_path).as_str());

                if let Some(metadata) = get_metadata(content.as_str()) {
                    println!("{:?}", metadata)
                } else {
                    println!("No metadata found in file")
                }
            }
        }
        Err(e) => eprintln!("Error reading from dir `articles`: {}", e),
    }
}

#[derive(Debug)]
struct Metadata {
    id: Option<String>,
    title: Option<String>,
    date: Option<String>,
    tags: Option<String>,
}

impl Metadata {
    fn is_complete(&self) -> bool {
        self.id.is_some() && self.title.is_some() && self.date.is_some() && self.tags.is_some()
    }
}

fn get_metadata(text: &str) -> Option<Metadata> {
    let mut metadata = Metadata {
        id: None,
        title: None,
        date: None,
        tags: None,
    };

    let re = Regex::new(r"% (\w+): (.+)").unwrap();

    for line in text.lines() {
        if let Some(capture) = re.captures(line) {
            if let (Some(key), Some(value)) = (capture.get(1), capture.get(2)) {
                match key.as_str() {
                    "id" => metadata.id = Some(value.as_str().to_string()),
                    "title" => metadata.title = Some(value.as_str().to_string()),
                    "date" => metadata.date = Some(value.as_str().to_string()),
                    "tags" => metadata.tags = Some(value.as_str().to_string()),
                    _ => {}
                }
            }
        }
    }

    if metadata.is_complete() {
        Some(metadata)
    } else {
        None
    }
}
