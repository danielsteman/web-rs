use std::ffi::OsStr;
use std::fs;
use std::path::Path;

fn main() {
    // let paths = fs::read_dir("./articles").unwrap();
    // for path in paths {
    //     for sub_path in path {
    //         println!("", &sub_path.path().extension().unwrap())
    //     }
    // }
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}
