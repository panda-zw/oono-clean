use anyhow::Result;
use std::process::Command;

use crate::models::safety::SafetyLevel;
use crate::models::scan::{ScanCategory, ScanItem};

use super::hash_id;

/// Check if Docker is available and running.
fn docker_available() -> bool {
    Command::new("docker")
        .args(["info"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Parse docker system df output to get sizes.
fn parse_docker_df() -> Result<DockerDf> {
    let output = Command::new("docker")
        .args(["system", "df", "--format", "{{.Type}}\t{{.Size}}\t{{.Reclaimable}}"])
        .output()?;

    if !output.status.success() {
        anyhow::bail!("docker system df failed");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut df = DockerDf::default();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 3 {
            continue;
        }
        let reclaimable_bytes = parse_docker_size(parts[2].split_whitespace().next().unwrap_or("0"));
        match parts[0] {
            "Images" => df.images_reclaimable = reclaimable_bytes,
            "Build Cache" => df.build_cache_reclaimable = reclaimable_bytes,
            _ => {}
        }
    }

    Ok(df)
}

#[derive(Default)]
struct DockerDf {
    images_reclaimable: u64,
    build_cache_reclaimable: u64,
}

/// Parse Docker's human-readable size strings (e.g. "1.2GB", "500MB", "10kB").
fn parse_docker_size(s: &str) -> u64 {
    let s = s.trim();
    if s == "0B" || s == "0" {
        return 0;
    }

    let (num_str, unit) = if s.ends_with("GB") {
        (&s[..s.len() - 2], 1_000_000_000u64)
    } else if s.ends_with("MB") {
        (&s[..s.len() - 2], 1_000_000u64)
    } else if s.ends_with("kB") {
        (&s[..s.len() - 2], 1_000u64)
    } else if s.ends_with('B') {
        (&s[..s.len() - 1], 1u64)
    } else {
        return 0;
    };

    num_str
        .trim()
        .parse::<f64>()
        .map(|n| (n * unit as f64) as u64)
        .unwrap_or(0)
}

/// Scan for unused Docker images.
pub async fn scan_docker_images() -> Result<Vec<ScanItem>> {
    let available = tokio::task::spawn_blocking(docker_available).await?;
    if !available {
        return Ok(vec![]);
    }

    let df = tokio::task::spawn_blocking(parse_docker_df).await??;
    if df.images_reclaimable == 0 {
        return Ok(vec![]);
    }

    Ok(vec![ScanItem {
        id: hash_id("docker_images", "docker_images"),
        path: "docker://images".to_string(),
        display_name: "Docker unused images".to_string(),
        description: "Container images not used by any running container. Re-pulled when needed."
            .to_string(),
        size_bytes: df.images_reclaimable,
        safety: SafetyLevel::Green,
        category: ScanCategory::DockerImages,
        last_modified: None,
    }])
}

/// Scan for Docker build cache.
pub async fn scan_docker_build_cache() -> Result<Vec<ScanItem>> {
    let available = tokio::task::spawn_blocking(docker_available).await?;
    if !available {
        return Ok(vec![]);
    }

    let df = tokio::task::spawn_blocking(parse_docker_df).await??;
    if df.build_cache_reclaimable == 0 {
        return Ok(vec![]);
    }

    Ok(vec![ScanItem {
        id: hash_id("docker_build_cache", "docker_build_cache"),
        path: "docker://build-cache".to_string(),
        display_name: "Docker build cache".to_string(),
        description: "Cached layers from building containers. Rebuilt automatically on next build."
            .to_string(),
        size_bytes: df.build_cache_reclaimable,
        safety: SafetyLevel::Green,
        category: ScanCategory::DockerBuildCache,
        last_modified: None,
    }])
}
