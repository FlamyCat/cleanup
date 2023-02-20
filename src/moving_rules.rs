use std::fs::DirEntry;

pub fn move_music(path: &DirEntry) {
    cleanup::move_file(&path, "Музыка");
}

pub fn move_source_code(path: &DirEntry) {
    cleanup::move_file(&path, "Исходный код");
}

pub fn move_document(path: &DirEntry) {
    cleanup::move_file(&path, "Документы");
}

pub fn move_disk_image(path: &DirEntry) {
    cleanup::move_file(&path, "Образы дисков");
}

pub fn move_shell_script(path: &DirEntry) {
    cleanup::move_file(&path, "Сценарии командной строки");
}

pub fn move_video(path: &DirEntry) {
    cleanup::move_file(&path, "Видео");
}

pub fn move_archive(path: &DirEntry) {
    cleanup::move_file(&path, "Архивы");
}

pub fn move_image(path: &DirEntry) {
    cleanup::move_file(&path, "Изображения");
}

pub fn move_misc(path: &DirEntry) {
    cleanup::move_file(&path, "Прочее");
}
