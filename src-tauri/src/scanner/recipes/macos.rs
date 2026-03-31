use anyhow::Result;
use std::process::Command;

use crate::models::safety::SafetyLevel;
use crate::models::scan::{ScanCategory, ScanItem};

use super::{calculate_dir_size_async, get_last_modified, hash_id};

/// Scan for Xcode iOS/watchOS/tvOS device support files.
pub async fn scan_xcode_device_support() -> Result<Vec<ScanItem>> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("No home directory"))?;
    let mut items = Vec::new();

    let support_dirs = [
        home.join("Library/Developer/Xcode/iOS DeviceSupport"),
        home.join("Library/Developer/Xcode/watchOS DeviceSupport"),
        home.join("Library/Developer/Xcode/tvOS DeviceSupport"),
    ];

    for dir in support_dirs {
        if !dir.exists() {
            continue;
        }
        let entries: Vec<_> = tokio::task::spawn_blocking({
            let d = dir.clone();
            move || -> Vec<(std::path::PathBuf, String)> {
                std::fs::read_dir(&d)
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

        for (path, version) in entries {
            let size = calculate_dir_size_async(&path).await?;
            if size == 0 {
                continue;
            }
            let path_str = path.to_string_lossy().to_string();
            items.push(ScanItem {
                id: hash_id(&path_str, "xcode_device_support"),
                path: path_str,
                display_name: format!("Device support {}", version),
                description: "Debug symbols for a connected iOS device - re-downloaded when you reconnect".to_string(),
                size_bytes: size,
                safety: SafetyLevel::Green,
                category: ScanCategory::XcodeDeviceSupport,
                last_modified: get_last_modified(&path),
            });
        }
    }

    Ok(items)
}

/// Scan for Xcode archives.
pub async fn scan_xcode_archives() -> Result<Vec<ScanItem>> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("No home directory"))?;
    let archives_dir = home.join("Library/Developer/Xcode/Archives");

    if !archives_dir.exists() {
        return Ok(vec![]);
    }

    let size = calculate_dir_size_async(&archives_dir).await?;
    if size == 0 {
        return Ok(vec![]);
    }

    let path_str = archives_dir.to_string_lossy().to_string();
    Ok(vec![ScanItem {
        id: hash_id(&path_str, "xcode_archives"),
        path: path_str,
        display_name: "Xcode Archives".to_string(),
        description: "Archived app builds - review before removing if you need to submit older versions".to_string(),
        size_bytes: size,
        safety: SafetyLevel::Yellow,
        category: ScanCategory::XcodeArchives,
        last_modified: get_last_modified(&archives_dir),
    }])
}

/// Scan for browser caches (Chrome, Safari, Firefox, Arc, Brave, Edge).
pub async fn scan_browser_caches() -> Result<Vec<ScanItem>> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("No home directory"))?;
    let mut items = Vec::new();

    let browsers = [
        (home.join("Library/Caches/Google/Chrome"), "Google Chrome cache"),
        (home.join("Library/Caches/com.apple.Safari"), "Safari cache"),
        (home.join("Library/Caches/Firefox/Profiles"), "Firefox cache"),
        (home.join("Library/Caches/com.operasoftware.Opera"), "Opera cache"),
        (home.join("Library/Caches/Arc"), "Arc cache"),
        (home.join("Library/Caches/com.brave.Browser"), "Brave cache"),
        (home.join("Library/Caches/com.microsoft.edgemac"), "Edge cache"),
        (home.join("Library/Caches/com.vivaldi.Vivaldi"), "Vivaldi cache"),
    ];

    for (path, name) in browsers {
        if !path.exists() {
            continue;
        }
        let size = calculate_dir_size_async(&path).await?;
        if size < 50_000_000 {
            continue;
        }
        let path_str = path.to_string_lossy().to_string();
        items.push(ScanItem {
            id: hash_id(&path_str, "browser_caches"),
            path: path_str,
            display_name: name.to_string(),
            description: "Cached web content - rebuilt automatically as you browse".to_string(),
            size_bytes: size,
            safety: SafetyLevel::Green,
            category: ScanCategory::BrowserCaches,
            last_modified: get_last_modified(&path),
        });
    }

    Ok(items)
}

/// Scan for system and application logs.
pub async fn scan_system_logs() -> Result<Vec<ScanItem>> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("No home directory"))?;
    let mut items = Vec::new();

    let log_dirs = [
        (home.join("Library/Logs"), "Application logs"),
        (home.join("Library/Logs/DiagnosticReports"), "Diagnostic reports"),
    ];

    for (path, name) in log_dirs {
        if !path.exists() {
            continue;
        }
        let size = calculate_dir_size_async(&path).await?;
        if size < 50_000_000 {
            continue;
        }
        let path_str = path.to_string_lossy().to_string();
        items.push(ScanItem {
            id: hash_id(&path_str, "system_logs"),
            path: path_str,
            display_name: name.to_string(),
            description: "Log files from apps and the system - safe to remove, new logs created as needed".to_string(),
            size_bytes: size,
            safety: SafetyLevel::Green,
            category: ScanCategory::SystemLogs,
            last_modified: get_last_modified(&path),
        });
    }

    Ok(items)
}

/// Scan for iOS device backups.
pub async fn scan_ios_backups() -> Result<Vec<ScanItem>> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("No home directory"))?;
    let backups_dir = home.join("Library/Application Support/MobileSync/Backup");

    if !backups_dir.exists() {
        return Ok(vec![]);
    }

    let mut items = Vec::new();

    let entries: Vec<_> = tokio::task::spawn_blocking({
        let d = backups_dir.clone();
        move || -> Vec<(std::path::PathBuf, String)> {
            std::fs::read_dir(&d)
                .ok()
                .map(|rd| {
                    rd.filter_map(|e| e.ok())
                        .filter(|e| e.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
                        .map(|e| {
                            let name = e.file_name().to_string_lossy().to_string();
                            // Backup dirs are UUIDs, shorten for display
                            let short = if name.len() > 8 {
                                format!("{}...", &name[..8])
                            } else {
                                name.clone()
                            };
                            (e.path(), short)
                        })
                        .collect()
                })
                .unwrap_or_default()
        }
    })
    .await?;

    for (path, short_id) in entries {
        let size = calculate_dir_size_async(&path).await?;
        if size == 0 {
            continue;
        }
        let path_str = path.to_string_lossy().to_string();
        items.push(ScanItem {
            id: hash_id(&path_str, "ios_backups"),
            path: path_str,
            display_name: format!("iOS backup ({})", short_id),
            description: "Local iPhone/iPad backup - check you have iCloud backup before removing".to_string(),
            size_bytes: size,
            safety: SafetyLevel::Yellow,
            category: ScanCategory::IosBackups,
            last_modified: get_last_modified(&path),
        });
    }

    Ok(items)
}

/// Scan the Trash.
pub async fn scan_trash() -> Result<Vec<ScanItem>> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("No home directory"))?;
    let trash_dir = home.join(".Trash");

    if !trash_dir.exists() {
        return Ok(vec![]);
    }

    let size = calculate_dir_size_async(&trash_dir).await?;
    if size < 10_000_000 {
        return Ok(vec![]);
    }

    let path_str = trash_dir.to_string_lossy().to_string();
    Ok(vec![ScanItem {
        id: hash_id(&path_str, "trash"),
        path: path_str,
        display_name: "Trash".to_string(),
        description: "Files you've already deleted but haven't emptied yet".to_string(),
        size_bytes: size,
        safety: SafetyLevel::Yellow,
        category: ScanCategory::Trash,
        last_modified: get_last_modified(&trash_dir),
    }])
}

/// Scan for old files in Downloads (older than 90 days).
pub async fn scan_old_downloads() -> Result<Vec<ScanItem>> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("No home directory"))?;
    let downloads_dir = home.join("Downloads");

    if !downloads_dir.exists() {
        return Ok(vec![]);
    }

    let ninety_days_ago = chrono::Utc::now().timestamp() - (90 * 24 * 60 * 60);

    let old_files: Vec<(std::path::PathBuf, u64)> = tokio::task::spawn_blocking({
        let d = downloads_dir.clone();
        move || -> Vec<(std::path::PathBuf, u64)> {
            std::fs::read_dir(&d)
                .ok()
                .map(|rd| {
                    rd.filter_map(|e| e.ok())
                        .filter_map(|e| {
                            let meta = e.metadata().ok()?;
                            let modified = meta.modified().ok()?;
                            let ts = modified
                                .duration_since(std::time::UNIX_EPOCH)
                                .ok()?
                                .as_secs() as i64;
                            if ts < ninety_days_ago {
                                let size = if meta.is_dir() {
                                    super::calculate_dir_size(&e.path())
                                } else {
                                    meta.len()
                                };
                                Some((e.path(), size))
                            } else {
                                None
                            }
                        })
                        .collect()
                })
                .unwrap_or_default()
        }
    })
    .await?;

    let total_size: u64 = old_files.iter().map(|(_, s)| s).sum();
    if total_size < 50_000_000 {
        return Ok(vec![]);
    }

    let path_str = downloads_dir.to_string_lossy().to_string();
    Ok(vec![ScanItem {
        id: hash_id(&path_str, "old_downloads"),
        path: "downloads://old-files".to_string(), // Special path — cleanup handled differently
        display_name: format!("Old downloads ({} files)", old_files.len()),
        description: "Files in Downloads older than 90 days".to_string(),
        size_bytes: total_size,
        safety: SafetyLevel::Yellow,
        category: ScanCategory::OldDownloads,
        last_modified: None,
    }])
}

/// Scan for Time Machine local snapshots.
pub async fn scan_time_machine_snapshots() -> Result<Vec<ScanItem>> {
    let output = tokio::task::spawn_blocking(|| {
        Command::new("tmutil")
            .args(["listlocalsnapshots", "/"])
            .output()
    })
    .await??;

    if !output.status.success() {
        return Ok(vec![]);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let snapshot_count = stdout.lines().filter(|l| l.contains("com.apple.TimeMachine")).count();

    if snapshot_count == 0 {
        return Ok(vec![]);
    }

    // Estimate size: get actual usage from tmutil
    let size_output = tokio::task::spawn_blocking(|| {
        Command::new("tmutil")
            .args(["listlocalsnapshots", "/", "-purgeable"])
            .output()
    })
    .await??;

    // Parse purgeable size if available, otherwise estimate
    let size_str = String::from_utf8_lossy(&size_output.stdout);
    let estimated_size = parse_purgeable_size(&size_str)
        .unwrap_or((snapshot_count as u64) * 1_000_000_000); // Rough 1GB per snapshot estimate

    if estimated_size == 0 {
        return Ok(vec![]);
    }

    Ok(vec![ScanItem {
        id: hash_id("timemachine://snapshots", "time_machine_snapshots"),
        path: "timemachine://snapshots".to_string(),
        display_name: format!("Time Machine snapshots ({})", snapshot_count),
        description: "Local Time Machine snapshots - new ones are created automatically".to_string(),
        size_bytes: estimated_size,
        safety: SafetyLevel::Yellow,
        category: ScanCategory::TimeMachineSnapshots,
        last_modified: None,
    }])
}

fn parse_purgeable_size(output: &str) -> Option<u64> {
    // Try to find a line with purgeable size info
    for line in output.lines() {
        let line = line.trim().to_lowercase();
        if line.contains("purgeable") || line.contains("size") {
            // Try to extract a number
            for word in line.split_whitespace() {
                if let Ok(bytes) = word.parse::<u64>() {
                    return Some(bytes);
                }
                // Handle GB/MB suffixes
                if word.ends_with("gb") {
                    if let Ok(n) = word[..word.len()-2].parse::<f64>() {
                        return Some((n * 1_000_000_000.0) as u64);
                    }
                }
            }
        }
    }
    None
}
