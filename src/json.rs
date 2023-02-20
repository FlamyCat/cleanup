use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use serde_json::json;
use cleanup::{display_error, display_warning};

pub fn create_default_ruleset() -> std::io::Result<()> {
    let ruleset = json!({
        "Прочее": [
            "",
            "txt"
        ],
        "Документы": [
            "docx",
            "xlsx",
            "pptx",
            "doc",
            "xls",
            "ppt",
            "odt",
            "pdf"
        ],
        "Музыка": [
            "mp3",
            "wav",
            "flac",
            "ogg"
        ],
        "Исходный код": [
            "c",
            "cpp",
            "cs",
            "rs",
            "py",
            "js",
            "html",
            "css"
        ],
        "Сценарии командной строки": [
            "sh",
            "sh1"
        ],
        "Видео": [
            "mp4",
            "mov",
            "webm",
            "avi",
            "mpg",
            "mpeg",
            "m4v"
        ],
        "Архивы": [
            "zip",
            "xz",
            "gz",
            "7z",
            "rar"
        ],
        "Изображения": [
            "jpg",
            "jpeg",
            "png",
            "heic",
            "tga",
            "dds",
            "gif"
        ],
        "Образы дисков": [
            "iso",
            "dmg"
        ]
    });

    let mut ruleset_file = File::create("ruleset.json")?;

    let _ = &ruleset_file.write_all(ruleset.to_string().as_ref());

    Ok(())
}

pub fn get_ruleset() -> Option<String> {
    if !Path::new("./ruleset.json").exists() {
        display_warning("отсутствует файл правил. Создается файл с набором правил по умолчанию.");
        match create_default_ruleset() {
            Ok(value) => {value}
            Err(e) => {
                display_error(&format!("не удалось создать файл правил ({})", e.to_string()));
                return None
            }
        }
    }

    let mut ruleset_file = match File::open("ruleset.json") {
        Ok(value) => {value}
        Err(e) => {
            display_error(&format!("не удалось открыть файл правил ruleset.json ({}).", e.to_string()));
            return None
        }
    };

    let mut ruleset: String = "".to_string();
    let result = ruleset_file.read_to_string(&mut ruleset);

    if let Err(e) = result {
        display_error(&format!("не удалось прочитать файл правил ruleset.json ({})", e.to_string()));
        None
    } else {
        Some(ruleset)
    }
}