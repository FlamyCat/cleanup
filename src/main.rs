use std::{env, fs, process};
use std::fs::ReadDir;

use cleanup::{display_error, path_is_file};


mod args;

fn main() {
    let files = enumerate_files();

    for file in files {
        let path = file.unwrap().path();

        if path_is_file(&path) {
            match &path.extension() {
                None => {
                    display_error(&format!("файл \"{}\" не имеет расширения.",
                                            path.file_name()
                                                .unwrap()
                                                .to_string_lossy()
                    ))
                }

                Some(extension) => {
                    println!("{}", extension.to_string_lossy());
                }
            }
        }
    }
}

fn enumerate_files() -> ReadDir {
    let files = match fs::read_dir(env::current_dir().unwrap()) {
        Ok(value) => { value }
        Err(_) => {
            display_error(&format!("не удалось перечислить файлы в текущей папке"));
            process::exit(1);
        }
    };
    files
}
