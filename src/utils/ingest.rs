use std::fs;

fn main() {
    let paths = get_paths();
}

fn get_paths() {
    let paths = fs::read_dir("./articles").unwrap();
    for path in paths {
        println!("{}", path.unwrap().path().display())
    }
}
