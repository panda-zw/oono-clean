use anyhow::Result;
use std::process::Command;

use crate::models::safety::SafetyLevel;
use crate::models::scan::{ScanCategory, ScanItem};

use super::{calculate_dir_size_async, get_last_modified, hash_id};

/// Scan for Xcode simulator devices and runtimes.
pub async fn scan_simulators() -> Result<Vec<ScanItem>> {
    // Check if xcrun is available
    let available = tokio::task::spawn_blocking(|| {
        Command::new("xcrun")
            .args(["simctl", "list", "devices", "-j"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    })
    .await?;

    if !available {
        return Ok(vec![]);
    }

    let mut items = Vec::new();

    // Check CoreSimulator devices directory
    if let Some(home) = dirs::home_dir() {
        let sim_dir = home.join("Library/Developer/CoreSimulator/Devices");
        if sim_dir.exists() {
            let size = calculate_dir_size_async(&sim_dir).await?;
            if size > 0 {
                let path_str = sim_dir.to_string_lossy().to_string();
                items.push(ScanItem {
                    id: hash_id(&path_str, "xcode_simulators"),
                    path: path_str,
                    display_name: "Xcode simulator devices".to_string(),
                    description:
                        "iOS/watchOS/tvOS simulator devices. Re-downloaded from Xcode when needed."
                            .to_string(),
                    size_bytes: size,
                    safety: SafetyLevel::Green,
                    category: ScanCategory::XcodeSimulators,
                    last_modified: get_last_modified(&sim_dir),
                });
            }
        }

        // Simulator caches
        let sim_caches = home.join("Library/Developer/CoreSimulator/Caches");
        if sim_caches.exists() {
            let size = calculate_dir_size_async(&sim_caches).await?;
            if size > 0 {
                let path_str = sim_caches.to_string_lossy().to_string();
                items.push(ScanItem {
                    id: hash_id(&path_str, "xcode_simulators"),
                    path: path_str,
                    display_name: "Xcode simulator caches".to_string(),
                    description: "Cached data for iOS simulators. Rebuilt when needed.".to_string(),
                    size_bytes: size,
                    safety: SafetyLevel::Green,
                    category: ScanCategory::XcodeSimulators,
                    last_modified: get_last_modified(&sim_caches),
                });
            }
        }
    }

    Ok(items)
}

/// Scan for Xcode derived data.
pub async fn scan_derived_data() -> Result<Vec<ScanItem>> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("No home directory"))?;
    let derived_data = home.join("Library/Developer/Xcode/DerivedData");

    if !derived_data.exists() {
        return Ok(vec![]);
    }

    let mut items = Vec::new();

    // List individual project build directories
    let entries: Vec<_> = tokio::task::spawn_blocking({
        let dd = derived_data.clone();
        move || -> Vec<(std::path::PathBuf, String)> {
            std::fs::read_dir(&dd)
                .ok()
                .map(|rd| {
                    rd.filter_map(|e| e.ok())
                        .filter(|e| e.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
                        .map(|e| {
                            let name = e.file_name().to_string_lossy().to_string();
                            // DerivedData dirs look like "ProjectName-abcdef123"
                            let display = name
                                .rsplit_once('-')
                                .map(|(n, _)| n.to_string())
                                .unwrap_or(name.clone());
                            (e.path(), display)
                        })
                        .collect()
                })
                .unwrap_or_default()
        }
    })
    .await?;

    for (path, project_name) in entries {
        let size = calculate_dir_size_async(&path).await?;
        if size == 0 {
            continue;
        }
        let path_str = path.to_string_lossy().to_string();
        items.push(ScanItem {
            id: hash_id(&path_str, "xcode_derived_data"),
            path: path_str,
            display_name: format!("{} (Xcode build data)", project_name),
            description:
                "Build artifacts from Xcode. Rebuilt automatically when you open the project."
                    .to_string(),
            size_bytes: size,
            safety: SafetyLevel::Green,
            category: ScanCategory::XcodeDerivedData,
            last_modified: get_last_modified(&path),
        });
    }

    Ok(items)
}
