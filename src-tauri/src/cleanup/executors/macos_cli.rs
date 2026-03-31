use anyhow::Result;
use std::process::Command;

/// Empty the Trash.
pub async fn empty_trash() -> Result<()> {
    // Use AppleScript to empty trash safely via Finder
    let output = tokio::task::spawn_blocking(|| {
        Command::new("osascript")
            .args(["-e", "tell application \"Finder\" to empty trash"])
            .output()
    })
    .await??;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to empty trash: {}", stderr);
    }
    Ok(())
}

/// Delete old files in Downloads (older than 90 days).
pub async fn delete_old_downloads() -> Result<()> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("No home directory"))?;
    let downloads_dir = home.join("Downloads");
    let ninety_days_ago = chrono::Utc::now().timestamp() - (90 * 24 * 60 * 60);

    let old_files: Vec<std::path::PathBuf> = tokio::task::spawn_blocking({
        let d = downloads_dir.clone();
        move || -> Vec<std::path::PathBuf> {
            std::fs::read_dir(&d)
                .ok()
                .map(|rd| {
                    rd.filter_map(|e| e.ok())
                        .filter(|e| {
                            e.metadata()
                                .ok()
                                .and_then(|m| m.modified().ok())
                                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                                .map(|d| (d.as_secs() as i64) < ninety_days_ago)
                                .unwrap_or(false)
                        })
                        .map(|e| e.path())
                        .collect()
                })
                .unwrap_or_default()
        }
    })
    .await?;

    for path in old_files {
        if path.is_dir() {
            tokio::fs::remove_dir_all(&path).await.ok();
        } else {
            tokio::fs::remove_file(&path).await.ok();
        }
    }

    Ok(())
}

/// Delete Time Machine local snapshots.
pub async fn delete_time_machine_snapshots() -> Result<()> {
    let output = tokio::task::spawn_blocking(|| {
        Command::new("tmutil")
            .args(["deletelocalsnapshots", "/"])
            .output()
    })
    .await??;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to delete TM snapshots: {}", stderr);
    }
    Ok(())
}
