use std::path::Path;
use tokio::fs::read_dir;

#[derive(Debug)]
pub(crate) enum Language {
    Go,
    Javascript,
    Rust,
}

impl Language {
    pub(crate) async fn infer_from_dir(dir: &Path) -> Self {
        let mut iter = read_dir(dir).await.unwrap();
        let mut language = None;
        while let Some(file) = iter.next_entry().await.unwrap() {
            match file.file_name().to_str().unwrap() {
                "Cargo.toml" => {
                    language = Some(Language::Rust);
                    break;
                }
                "main.go" => {
                    language = Some(Language::Go);
                    break;
                }
                "main.js" => {
                    language = Some(Language::Javascript);
                    break;
                }
                _ => continue,
            }
        }
        language.unwrap()
    }
}
