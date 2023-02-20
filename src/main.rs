use std::fs::DirEntry;
use cleanup::{display_warning, path_is_file};


mod args;
mod moving_rules;

fn main() {
    let files = cleanup::enumerate_files();


    for file in files {
        let path = &file.unwrap();

        if path_is_file(&path.path()) {
            match &path.path().extension() {
                None => {
                    moving_rules::move_misc(&path);
                }

                Some(extension) => {
                    match extension.to_str() {
                        Some(value) => {
                            handle_file_extension(&path, value)
                        }
                        None => {
                            handle_missing_extension(&path)
                        }
                    }
                }
            }
        }
    }
}

fn handle_file_extension(path: &&DirEntry, value: &str) {
    match value {
        "txt" => moving_rules::move_misc(&path),

        "docx" | "xlsx" | "pptx"
        | "doc" | "xls" | "ppt"
        | "odt"
        | "pdf" => moving_rules::move_document(&path),

        "iso" => moving_rules::move_disk_image(&path),

        "mp3" | "wav" | "flac" | "ogg" => moving_rules::move_music(&path),

        "c" | "cpp" | "cs" | "rs" | "py" | "js" | "html" | "css" => moving_rules::move_source_code(&path),

        "sh" | "sh1" => moving_rules::move_shell_script(&path),

        "mp4" | "mov" => moving_rules::move_video(&path),

        "zip" | "xz" | "gz" | "7z" => moving_rules::move_archive(&path),

        "jpg" | "jpeg" | "png" | "heic" | "tga" | "dds" => moving_rules::move_image(&path),

        _ => {
            display_warning(&format!("файл {} имеет неизвестный тип и будет перемещен в папку \"Прочее\".", &path.file_name().to_str().unwrap()));
            moving_rules::move_misc(&path);
        }
    }
}

fn handle_missing_extension(path: &&DirEntry) {
    display_warning(&format!("не удается преобразовать расширение файла {} в UTF-8. Он будет перемещен в папку \"Прочее\".", &path.file_name().to_str().unwrap()));
    moving_rules::move_misc(&path);
}
