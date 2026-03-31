use std::path::Path;
use std::process::Command;

use crate::models::safety::SafetyLevel;
use crate::models::scan::{ScanCategory, ScanItem};

/// Classify all scan items, potentially adjusting safety levels.
pub fn classify_all(items: Vec<ScanItem>) -> Vec<ScanItem> {
    items.into_iter().map(classify_item).collect()
}

fn classify_item(mut item: ScanItem) -> ScanItem {
    // For node_modules: downgrade to yellow if the project has uncommitted git changes
    if item.category == ScanCategory::NodeDependencies {
        if let Some(parent) = Path::new(&item.path).parent() {
            if has_uncommitted_changes(parent) {
                item.safety = SafetyLevel::Yellow;
                item.description = format!(
                    "{} (project has uncommitted changes - review before removing)",
                    item.description
                );
            }
        }
    }

    item
}

fn has_uncommitted_changes(project_dir: &Path) -> bool {
    Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(project_dir)
        .output()
        .map(|o| o.status.success() && !o.stdout.is_empty())
        .unwrap_or(false)
}
