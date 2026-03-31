use anyhow::Result;

use crate::models::safety::SafetyLevel;
use crate::models::scan::{ScanCategory, ScanItem};

use super::{calculate_dir_size_async, get_last_modified, hash_id};

/// Scan for Gradle build caches.
pub async fn scan_gradle_cache() -> Result<Vec<ScanItem>> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("No home directory"))?;
    let mut items = Vec::new();

    let cache_dirs = [
        (home.join(".gradle/caches"), "Gradle build cache"),
        (home.join(".gradle/wrapper/dists"), "Gradle wrapper distributions"),
    ];

    for (path, name) in cache_dirs {
        if !path.exists() {
            continue;
        }
        let size = calculate_dir_size_async(&path).await?;
        if size == 0 {
            continue;
        }
        let path_str = path.to_string_lossy().to_string();
        items.push(ScanItem {
            id: hash_id(&path_str, "gradle_cache"),
            path: path_str,
            display_name: name.to_string(),
            description: "Cached dependencies for Android/Java projects. Re-downloaded on next build."
                .to_string(),
            size_bytes: size,
            safety: SafetyLevel::Green,
            category: ScanCategory::GradleCache,
            last_modified: get_last_modified(&path),
        });
    }

    Ok(items)
}
