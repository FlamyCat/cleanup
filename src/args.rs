pub use clap::Parser;

/// Сортировка файлов в папке
#[derive(Parser)]
pub(crate) struct Args {
    /// Директория для очистки
    #[arg(long)]
    pub dir: Option<String>,

    /// Регулярное выражение, по которому осуществляется поиск файлов и папок для удаления
    #[arg(short, long)]
    pub deletion_pattern: Option<String>
}
