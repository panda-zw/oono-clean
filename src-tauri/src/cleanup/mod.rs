pub mod executors;

use chrono::Utc;
use tauri::{AppHandle, Emitter};

use crate::db::audit_repo;
use crate::models::audit::AuditEntry;
use crate::models::cleanup::{CleanupItemResult, CleanupProgress, CleanupStatus};
use crate::models::scan::{ScanCategory, ScanItem};
use crate::AppState;

pub async fn execute_cleanup(
    items: Vec<ScanItem>,
    app_handle: AppHandle,
    state: &AppState,
) -> CleanupProgress {
    let total = items.len();
    let mut progress = CleanupProgress {
        total_items: total,
        completed_items: 0,
        current_item: None,
        bytes_freed: 0,
        status: CleanupStatus::InProgress,
        results: Vec::new(),
    };

    for item in &items {
        progress.current_item = Some(item.display_name.clone());
        let _ = app_handle.emit("cleanup:progress", &progress);

        let result = match item.category {
            ScanCategory::DockerImages => executors::docker_cli::prune_images().await,
            ScanCategory::DockerBuildCache => executors::docker_cli::prune_build_cache().await,
            ScanCategory::XcodeSimulators => executors::xcode_cli::delete_simulators().await,
            ScanCategory::HomebrewCache => executors::homebrew_cli::cleanup().await,
            ScanCategory::Trash => executors::macos_cli::empty_trash().await,
            ScanCategory::OldDownloads => executors::macos_cli::delete_old_downloads().await,
            ScanCategory::TimeMachineSnapshots => executors::macos_cli::delete_time_machine_snapshots().await,
            _ => executors::file_delete::delete_path(&item.path).await,
        };

        let (success, error) = match result {
            Ok(()) => (true, None),
            Err(e) => (false, Some(e.to_string())),
        };

        // Briefly lock DB just for the audit write
        if let Ok(db) = state.db.lock() {
            let _ = audit_repo::insert_entry(
                &db,
                &AuditEntry {
                    id: 0,
                    item_path: item.path.clone(),
                    item_display_name: item.display_name.clone(),
                    category: item.category.as_str().to_string(),
                    size_bytes: item.size_bytes,
                    deleted_at: Utc::now().timestamp(),
                    success,
                    error_message: error.clone(),
                },
            );
        }

        if success {
            progress.bytes_freed += item.size_bytes;
        }

        progress.completed_items += 1;
        progress.results.push(CleanupItemResult {
            item_id: item.id.clone(),
            path: item.path.clone(),
            size_bytes: item.size_bytes,
            success,
            error,
        });
    }

    progress.status = CleanupStatus::Completed;
    progress.current_item = None;
    let _ = app_handle.emit("cleanup:progress", &progress);
    progress
}
