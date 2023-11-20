use std::{fs, io};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use ansi_term::Color::*;
use regex::Regex;

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


/// Проверяет, можно ли удалить файл или директорию по заданному пути.
/// Если это можно сделать, файл или директория переносятся в корзину.
/// Если при удалении файла/директории возникает ошибка, функция ее обрабатывает.
///
/// # Аргументы
///
/// * `path_to_dir_or_file`: путь до файла или директории.
/// * `pattern`: регулярное выражение, по которому проверяется, подлежит ли файл удалению.
///
/// # Возвращаемое значение
/// Истина, если удаление прошло успешно.
///
pub fn try_to_delete_file_or_dir(path_to_dir_or_file: &Path, pattern: Regex) -> bool {
    if pattern.is_match(path_to_dir_or_file.as_os_str().to_str().unwrap().trim()) {
        let result = trash::delete(path_to_dir_or_file);
        if let Err(e) = result {
            display_error(&format!("Ошибка при удалении {}, ({})", path_to_dir_or_file.to_str().unwrap(), e));
            return false;
        }
        return true;
    }
    return false;
}
