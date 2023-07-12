use std::{env, fs};
use std::process::ExitCode;

use file_format::{FileFormat, Kind};

use args::Args;
use args::Parser;
use cleanup::{display_error, display_warning, move_file_to_dir};

mod args;

fn main() -> ExitCode {
    let args = Args::parse();

    let files = if let Some(dir) = args.dir {
        dir
    } else if let Ok(files) = env::current_dir() {
        files.to_str().unwrap().to_string()
    } else {
        display_error("выбранная директория недоступна или не существует");
        return ExitCode::from(1);
    };

    for file in fs::read_dir(files).unwrap() {
        let file = file.unwrap();

        if file.metadata().unwrap().is_dir() {
            continue;
        }

        let file_format = FileFormat::from_file(file.file_name());

        if let Err(_) = file_format {
            display_warning(format!("ошибка при получении формата файла \"{}\", файл будет пропущен.", file.file_name().to_string_lossy()).as_str());
            continue;
        }

        let file_format = file_format.unwrap();

        let destination = match file_format.kind() {
            Kind::Application | Kind::Geospatial | Kind::Rom => {
                "Другое"
            }
            Kind::Archive => {
                "Архивы"
            }
            Kind::Audio => {
                "Аудиозаписи"
            }
            Kind::Book => {
                "Книги"
            }
            Kind::Certificate => {
                "Сертификаты"
            }
            Kind::Compression => {
                "Сжатые файлы"
            }
            Kind::Disk => {
                "Образы дисков"
            }
            Kind::Document => {
                "Документы"
            }
            Kind::Executable => {
                "Исполняемые файлы"
            }
            Kind::Font => {
                "Шрифты"
            }
            Kind::Image => {
                "Изображения"
            }
            Kind::Model => {
                "3D-модели и чертежи"
            }
            Kind::Package => {
                "Пакеты"
            }
            Kind::Playlist => {
                "Плейлисты"
            }
            Kind::Subtitle => {
                "Субтитры"
            }
            Kind::Text => {
                "Текстовые файлы"
            }
            Kind::Video => {
                "Видео"
            }
        };

        let result = move_file_to_dir(&mut file.path(), destination);

        if let Err(e) = result {
            display_error(format!("Ошибка при перемещении файла \"{}\" ({})", file.file_name().to_string_lossy(), e).as_str())
        }
    }

    ExitCode::SUCCESS
}