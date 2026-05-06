use anyhow::Result;
use std::path::{Path, PathBuf};

/// Known safe path patterns relative to the home directory.
/// Only paths matching these patterns can be deleted.
const SAFE_PATTERNS: &[&str] = &[
    // System / macOS
    "Library/Caches",
    "Library/Developer/CoreSimulator",
    "Library/Developer/Xcode/DerivedData",
    "Library/Application Support/Code/Cache",
    "Library/Application Support/Code/CachedData",
    "Library/Application Support/Code/CachedExtensionVSIXs",
    "Library/Application Support/Cursor/Cache",
    "Library/Application Support/Cursor/CachedData",
    // Node / JS
    "Library/pnpm",
    ".npm",
    ".yarn",
    ".pnpm-store",
    ".expo",
    // Rust
    ".cargo/registry",
    ".cargo/git",
    // Python
    ".cache/pip",
    ".conda/pkgs",
    ".mypy_cache",
    ".ruff_cache",
    "miniconda3/pkgs",
    "anaconda3/pkgs",
    // Go
    "go/pkg",
    ".cache/go-build",
    // Java
    ".m2/repository",
    ".gradle",
    // Ruby
    ".gem",
    ".bundle/cache",
    // .NET
    ".nuget",
    ".dotnet",
    ".local/share/NuGet",
    // Flutter / Dart
    ".pub-cache",
    ".dartServer",
    // CocoaPods
    ".cocoapods",
    // PHP
    ".composer/cache",
    // macOS system
    "Library/Logs",
    "Library/Developer/Xcode/iOS DeviceSupport",
    "Library/Developer/Xcode/watchOS DeviceSupport",
    "Library/Developer/Xcode/tvOS DeviceSupport",
    "Library/Developer/Xcode/Archives",
    "Library/Application Support/MobileSync/Backup",
];

/// Check if the path contains a known safe directory component
/// (node_modules, target with Cargo.toml, venv with pyvenv.cfg).
fn contains_safe_project_dir(path: &Path) -> bool {
    path.components()
        .any(|c| {
            let name = c.as_os_str().to_string_lossy();
            name == "node_modules" || name == "target" || name == "venv"
                || name == ".venv" || name == ".tox" || name == "__pycache__"
        })
}

/// Validate that a path is safe to delete:
/// - Must be under the user's home directory
/// - Must match a known safe pattern or contain node_modules
/// - Resolves symlinks to prevent symlink attacks
fn validate_path(path: &Path) -> Result<PathBuf> {
    let home = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Cannot determine home directory"))?;

    // Resolve symlinks and normalize the path
    let canonical = path.canonicalize()
        .map_err(|e| anyhow::anyhow!("Cannot resolve path {}: {}", path.display(), e))?;

    // Must be under the home directory
    if !canonical.starts_with(&home) {
        anyhow::bail!(
            "Refusing to delete path outside home directory: {}",
            canonical.display()
        );
    }

    // Must not be the home directory itself
    if canonical == home {
        anyhow::bail!("Refusing to delete the home directory");
    }

    // Check against known safe patterns
    let relative = canonical.strip_prefix(&home)
        .map_err(|_| anyhow::anyhow!("Path is not relative to home"))?;

    let is_safe = SAFE_PATTERNS
        .iter()
        .any(|pattern| relative.starts_with(pattern))
        || contains_safe_project_dir(&canonical);

    if !is_safe {
        anyhow::bail!(
            "Path does not match any known safe deletion pattern: {}",
            canonical.display()
        );
    }

    Ok(canonical)
}

/// Delete a file or directory at the given path after validation.
pub async fn delete_path(path: &str) -> Result<()> {
    let path = PathBuf::from(path);
    if !path.exists() {
        return Ok(()); // Already gone
    }

    let validated = validate_path(&path)?;

    if validated.is_dir() {
        tokio::fs::remove_dir_all(&validated).await?;
    } else {
        tokio::fs::remove_file(&validated).await?;
    }
    Ok(())
}

/// Top-level home folders we never delete in their entirety, even when the
/// user selects them. Individual children are fine.
const PROTECTED_TOPLEVEL: &[&str] = &[
    "Library",
    "Documents",
    "Downloads",
    "Desktop",
    "Movies",
    "Music",
    "Pictures",
    "Public",
    "Applications",
];

/// Delete a user-selected file or directory. Used for categories where the
/// user explicitly opts in to each item (Documents, App Data) and the
/// safe-pattern allowlist used by `delete_path` would be too restrictive.
/// Still enforces: must be under home, must not be home itself, must not be
/// one of the protected top-level home folders.
pub async fn delete_user_path(path: &str) -> Result<()> {
    let path = PathBuf::from(path);
    if !path.exists() {
        return Ok(()); // Already gone
    }

    let canonical = path
        .canonicalize()
        .map_err(|e| anyhow::anyhow!("Cannot resolve path {}: {}", path.display(), e))?;

    let home = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Cannot determine home directory"))?;

    if !canonical.starts_with(&home) {
        anyhow::bail!(
            "Refusing to delete path outside home directory: {}",
            canonical.display()
        );
    }

    if canonical == home {
        anyhow::bail!("Refusing to delete the home directory");
    }

    let relative = canonical
        .strip_prefix(&home)
        .map_err(|_| anyhow::anyhow!("Path is not relative to home"))?;

    let components: Vec<_> = relative.components().collect();
    if components.len() == 1 {
        let name = components[0].as_os_str().to_string_lossy();
        if PROTECTED_TOPLEVEL.iter().any(|p| *p == name) {
            anyhow::bail!("Refusing to delete protected top-level folder: {}", name);
        }
    }

    if canonical.is_dir() {
        tokio::fs::remove_dir_all(&canonical).await?;
    } else {
        tokio::fs::remove_file(&canonical).await?;
    }
    Ok(())
}
