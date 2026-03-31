use anyhow::Result;
use std::process::Command;

use crate::models::safety::SafetyLevel;
use crate::models::scan::{ScanCategory, ScanItem};

use super::{calculate_dir_size_async, get_last_modified, hash_id};

/// Scan ~/Library/Caches per-app subdirectories.
pub async fn scan_library_caches() -> Result<Vec<ScanItem>> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("No home directory"))?;
    let caches_dir = home.join("Library/Caches");

    if !caches_dir.exists() {
        return Ok(vec![]);
    }

    let entries: Vec<_> = tokio::task::spawn_blocking({
        let cd = caches_dir.clone();
        move || -> Vec<(std::path::PathBuf, String)> {
            std::fs::read_dir(&cd)
                .ok()
                .map(|rd| {
                    rd.filter_map(|e| e.ok())
                        .filter(|e| e.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
                        .map(|e| {
                            let name = e.file_name().to_string_lossy().to_string();
                            (e.path(), name)
                        })
                        .collect()
                })
                .unwrap_or_default()
        }
    })
    .await?;

    let mut items = Vec::new();

    for (path, name) in entries {
        let size = calculate_dir_size_async(&path).await?;
        // Only include caches over 50MB to avoid clutter
        if size < 50_000_000 {
            continue;
        }
        let path_str = path.to_string_lossy().to_string();
        // Create a friendlier name from bundle identifier
        let display_name = humanize_bundle_id(&name);
        items.push(ScanItem {
            id: hash_id(&path_str, "system_caches"),
            path: path_str,
            display_name: format!("{} cache", display_name),
            description: "Application cache - regenerated automatically by the app.".to_string(),
            size_bytes: size,
            safety: SafetyLevel::Green,
            category: ScanCategory::SystemCaches,
            last_modified: get_last_modified(&path),
        });
    }

    Ok(items)
}

/// Scan for Homebrew cache.
pub async fn scan_homebrew_cache() -> Result<Vec<ScanItem>> {
    // Try to find brew cache directory
    let cache_dir = tokio::task::spawn_blocking(|| {
        Command::new("brew")
            .args(["--cache"])
            .output()
            .ok()
            .and_then(|o| {
                if o.status.success() {
                    Some(
                        String::from_utf8_lossy(&o.stdout)
                            .trim()
                            .to_string(),
                    )
                } else {
                    None
                }
            })
    })
    .await?;

    let cache_path = match cache_dir {
        Some(p) => std::path::PathBuf::from(p),
        None => return Ok(vec![]),
    };

    if !cache_path.exists() {
        return Ok(vec![]);
    }

    let size = calculate_dir_size_async(&cache_path).await?;
    if size == 0 {
        return Ok(vec![]);
    }

    let path_str = cache_path.to_string_lossy().to_string();
    Ok(vec![ScanItem {
        id: hash_id(&path_str, "homebrew_cache"),
        path: path_str,
        display_name: "Homebrew package cache".to_string(),
        description:
            "Downloaded package files from Homebrew. No longer needed after installation."
                .to_string(),
        size_bytes: size,
        safety: SafetyLevel::Green,
        category: ScanCategory::HomebrewCache,
        last_modified: get_last_modified(&cache_path),
    }])
}

/// Convert a bundle identifier like "com.apple.Safari" to "Safari".
fn humanize_bundle_id(bundle_id: &str) -> String {
    // Take the last component of a bundle identifier
    bundle_id
        .rsplit('.')
        .next()
        .unwrap_or(bundle_id)
        .to_string()
}
