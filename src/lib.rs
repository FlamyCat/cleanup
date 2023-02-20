use std::path::{Path, PathBuf};
use std::fs::{DirEntry, metadata, ReadDir};
use ansi_term::Color::*;
use std::{env, fs, process};

pub fn path_is_file(path: &PathBuf) -> bool {
    metadata(&path.display().to_string()).unwrap().is_file()
}

pub fn display_error(message: &str) {
    println!("{}: {}", Red.paint("Ошибка"), message);
}

pub fn display_warning(message: &str) {
    println!("{}: {}", Yellow.paint("Предупреждение"), message);
}

pub fn enumerate_files() -> ReadDir {
    let files = match fs::read_dir(env::current_dir().unwrap()) {
        Ok(value) => { value }
        Err(_) => {
            display_error(&format!("не удалось перечислить файлы в текущей папке"));
            process::exit(1);
        }
    };
    files
}

pub fn move_file(path: &&DirEntry, new_directory: &str) {
    create_directory(&new_directory);

    let destination = get_destination(path, new_directory);
    if let Err(e) = fs::rename(&path.file_name(), &destination) {
        display_error(&format!("не удалось переместить файл \"{}\" в папку \"{}\" ({})", path.file_name().to_string_lossy(), new_directory, e.to_string()))
    }
}

fn create_directory(directory: &&str) {
    let path_to_new_folder = format!("{}/{}", env::current_dir().unwrap().to_str().unwrap(), directory);
    if !Path::new(&path_to_new_folder).exists() {
        if let Err(e) = fs::create_dir(&path_to_new_folder) {
            display_error(&format!("не удалось создать папку \"{}\" ({})", directory, e.to_string()));
        }
    }
}

fn get_destination(path_to_file: &DirEntry, new_folder: &str) -> String {
    format!("{}/{}/{}", env::current_dir().unwrap().display(), new_folder, path_to_file.file_name().to_string_lossy())
}
