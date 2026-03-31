use anyhow::Result;
use walkdir::WalkDir;

use crate::models::safety::SafetyLevel;
use crate::models::scan::{ScanCategory, ScanItem};

use super::{calculate_dir_size_async, get_last_modified, hash_id, is_hidden, project_search_roots};

/// Scan for node_modules directories in common project locations.
pub async fn scan_node_modules() -> Result<Vec<ScanItem>> {
    let search_roots = project_search_roots();
    let mut items = Vec::new();

    for root in search_roots {
        let root_clone = root.clone();
        let found: Vec<_> = tokio::task::spawn_blocking(move || {
            let mut paths = Vec::new();
            for entry in WalkDir::new(&root_clone)
                .max_depth(5)
                .into_iter()
                .filter_entry(|e| {
                    let name = e.file_name().to_string_lossy();
                    !is_hidden(e) && name != ".git" && name != "node_modules"
                })
            {
                let entry = match entry {
                    Ok(e) => e,
                    Err(_) => continue,
                };
                if entry.file_name() == "node_modules" && entry.file_type().is_dir() {
                    // Fingerprint: confirm parent has package.json
                    if let Some(parent) = entry.path().parent() {
                        if parent.join("package.json").exists() {
                            paths.push((
                                entry.path().to_path_buf(),
                                parent.file_name().unwrap_or_default().to_string_lossy().to_string(),
                            ));
                        }
                    }
                }
            }
            paths
        })
        .await?;

        for (path, project_name) in found {
            let size = calculate_dir_size_async(&path).await?;
            if size == 0 {
                continue;
            }
            let path_str = path.to_string_lossy().to_string();
            items.push(ScanItem {
                id: hash_id(&path_str, "node_dependencies"),
                path: path_str,
                display_name: format!("{}/node_modules", project_name),
                description: "Coding project dependencies - re-downloaded when you run npm install"
                    .to_string(),
                size_bytes: size,
                safety: SafetyLevel::Green,
                category: ScanCategory::NodeDependencies,
                last_modified: get_last_modified(&path),
            });
        }
    }

    // Also check for node_modules that are direct children in the walk
    // The filter_entry above skips descending into node_modules
    // We need a second pass that specifically looks for them
    Ok(items)
}

/// Scan for npm, yarn, and pnpm caches.
pub async fn scan_node_caches() -> Result<Vec<ScanItem>> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("No home directory"))?;
    let mut items = Vec::new();

    let cache_locations = [
        (home.join(".npm/_cacache"), "npm cache", "npm download cache - cleared safely with npm cache clean"),
        (home.join(".yarn/cache"), "Yarn cache", "Yarn package cache - re-downloaded when needed"),
        (home.join("Library/Caches/Yarn"), "Yarn cache (Library)", "Yarn package cache in Library - re-downloaded when needed"),
        (home.join("Library/pnpm/store"), "pnpm store", "pnpm content-addressable store - rebuilt with pnpm store prune"),
        (home.join(".pnpm-store"), "pnpm store", "pnpm content-addressable store - rebuilt with pnpm store prune"),
    ];

    for (path, name, description) in cache_locations {
        if !path.exists() {
            continue;
        }
        let size = calculate_dir_size_async(&path).await?;
        if size == 0 {
            continue;
        }
        let path_str = path.to_string_lossy().to_string();
        items.push(ScanItem {
            id: hash_id(&path_str, "node_caches"),
            path: path_str,
            display_name: name.to_string(),
            description: description.to_string(),
            size_bytes: size,
            safety: SafetyLevel::Green,
            category: ScanCategory::NodeCaches,
            last_modified: get_last_modified(&path),
        });
    }

    Ok(items)
}
