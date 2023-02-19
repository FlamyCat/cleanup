use std::path::PathBuf;
use std::fs::metadata;
use ansi_term::Color::Red;

pub fn path_is_file(path: &PathBuf) -> bool {
    metadata(&path.display().to_string()).unwrap().is_file()
}

pub fn display_error(message: &str) {
    println!("{}: {}", Red.paint("Ошибка"), message);
}
