pub mod android;
pub mod docker;
pub mod go;
pub mod macos;
pub mod maven;
pub mod misc;
pub mod node;
pub mod python;
pub mod rust;
pub mod system;
pub mod xcode;

use anyhow::Result;
use sha2::{Digest, Sha256};
use std::path::Path;
use walkdir::WalkDir;

/// Calculate total size of a directory by walking all files.
pub fn calculate_dir_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.metadata().map(|m| m.len()).unwrap_or(0))
        .sum()
}

/// Generate a stable ID from a path and category.
pub fn hash_id(path: &str, category: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(path.as_bytes());
    hasher.update(category.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)[..16].to_string()
}

/// Get last modified timestamp for a path.
pub fn get_last_modified(path: &Path) -> Option<i64> {
    path.metadata()
        .ok()
        .and_then(|m| m.modified().ok())
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64)
}

/// Get common project root directories to search.
pub fn project_search_roots() -> Vec<std::path::PathBuf> {
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => return vec![],
    };

    ["Development", "Projects", "Code", "repos", "workspace", "Desktop", "Documents", "dev", "src"]
        .iter()
        .map(|d| home.join(d))
        .filter(|p| p.exists())
        .collect()
}

/// Check if a walkdir entry is hidden (starts with dot).
pub fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

/// Calculate size asynchronously using spawn_blocking.
pub async fn calculate_dir_size_async(path: &Path) -> Result<u64> {
    let path = path.to_owned();
    let size = tokio::task::spawn_blocking(move || calculate_dir_size(&path)).await?;
    Ok(size)
}
