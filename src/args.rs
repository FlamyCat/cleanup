pub use clap::Parser;

/// Сортировка файлов в папке
#[derive(Parser)]
pub(crate) struct Args {
    /// Директория для очистки
    #[arg(short, long)]
    pub dir: Option<String>
}
