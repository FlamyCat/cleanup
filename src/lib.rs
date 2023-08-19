use std::{fs, io};
use std::io::ErrorKind;
use std::path::PathBuf;

use ansi_term::Color::*;

/// Выводит сообщение об ошибке.
///
/// # Аргументы
///
/// * `message`: сообщение об ошибке
///
pub fn display_error(message: &str) {
    println!("{}: {}", Red.paint("Ошибка"), message);
}

/// Выводит предупреждение.
///
/// # Аргументы
///
/// * `message`: сообщение
///
pub fn display_warning(message: &str) {
    println!("{}: {}", Yellow.paint("Предупреждение"), message);
}

/// Создает новую директорию по заданному пути.
///
/// # Аргументы
///
/// * `path`: путь
///
/// # Ошибки
///
/// Возвращает ошибку, если у пользователя нет прав на создание директории или операция была прервана.
///
fn create_dir(path: &PathBuf) -> io::Result<()> {
    let result = fs::create_dir(path);

    if let Err(e) = &result {
        match e.kind() {
            ErrorKind::PermissionDenied | ErrorKind::Interrupted => {
                return result;
            }
            _ => {}
        }
    }

    Ok(())
}

/// Перемещает файл в указанную директорию. Если такая не существует, функция ее создает.
///
/// # Аргументы
///
/// * `path_to_dir`: полный путь до новой родительской папки
/// * `path_to_file`: полный путь до файла
///
pub fn move_file_to_dir(path_to_dir: &mut PathBuf, path_to_file: PathBuf) -> io::Result<()> {
    if !path_to_dir.exists() {
        create_dir(path_to_dir)?;
    }

    let file_name = path_to_file.file_name().unwrap();
    path_to_dir.push(file_name);

    fs::rename(path_to_file, path_to_dir)?;

    Ok(())
}
