// ============================================================
// update - 版本更新检查
// ============================================================

use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use anyhow::{bail, Context, Result};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const CHECK_INTERVAL: Duration = Duration::from_secs(24 * 60 * 60); // 24 hours
const MARKER_FILE: &str = "last_update_check";

/// 获取 base 目录: ~/.agent-codemap
pub fn base_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("cannot determine home directory")?;
    Ok(home.join(".agent-codemap"))
}

/// 是否该检查更新 (每 24 小时一次)
pub fn should_check(base_dir: &Path) -> bool {
    let marker = base_dir.join(MARKER_FILE);
    if !marker.exists() {
        return true;
    }

    marker
        .metadata()
        .and_then(|m| m.modified())
        .map(|mtime| SystemTime::now().duration_since(mtime).unwrap_or_default() > CHECK_INTERVAL)
        .unwrap_or(true)
}

/// 标记已检查
pub fn mark_checked(base_dir: &Path) -> Result<()> {
    std::fs::create_dir_all(base_dir)?;
    let marker = base_dir.join(MARKER_FILE);
    std::fs::write(&marker, "")?;
    Ok(())
}

/// 版本比较: latest > current 返回 true
pub fn compare_versions(current: &str, latest: &str) -> bool {
    let parse = |v: &str| -> Vec<u32> {
        v.split('.')
            .filter_map(|s| s.parse().ok())
            .collect()
    };

    let current_parts = parse(current);
    let latest_parts = parse(latest);

    for i in 0..current_parts.len().max(latest_parts.len()) {
        let c = current_parts.get(i).copied().unwrap_or(0);
        let l = latest_parts.get(i).copied().unwrap_or(0);
        if l > c {
            return true;
        }
        if l < c {
            return false;
        }
    }
    false
}

/// 从 npm registry 检查最新版本
/// 有新版本返回 Some(version), 否则 None
pub fn check_update(current_version: &str) -> Result<Option<String>> {
    let url = "https://registry.npmjs.org/agent-codemap/latest";

    let agent = ureq::Agent::new_with_config(
        ureq::config::Config::builder()
            .timeout_global(Some(Duration::from_secs(5)))
            .build(),
    );

    let body: String = agent
        .get(url)
        .call()
        .context("failed to fetch npm registry")?
        .body_mut()
        .read_to_string()
        .context("failed to read response body")?;

    // 手动解析 JSON 中的 version 字段
    let version = body
        .split("\"version\":")
        .nth(1)
        .and_then(|s: &str| s.split('"').nth(1))
        .context("version field not found in response")?;

    if compare_versions(current_version, version) {
        Ok(Some(version.to_string()))
    } else {
        Ok(None)
    }
}

/// 后台检查入口: spawn 线程，启动时调用
pub fn spawn_background_check(dir: PathBuf) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        if let Ok(Some(latest)) = check_update(VERSION) {
            eprintln!(
                "\x1b[33mA new version of agent-codemap is available: {} -> {}\x1b[0m",
                VERSION, latest
            );
            eprintln!("\x1b[33mRun `agent-codemap --update` to update\x1b[0m");
        }
        let _ = mark_checked(&dir);
    })
}

/// 执行更新
pub fn run_update() -> Result<()> {
    eprintln!("Checking for updates...");

    match check_update(VERSION)? {
        None => {
            eprintln!("Already up to date ({})", VERSION);
        }
        Some(latest) => {
            eprintln!("Updating agent-codemap: {} -> {}", VERSION, latest);

            let status = std::process::Command::new("npm")
                .args(["install", "-g", "agent-codemap@latest"])
                .status()
                .context("failed to run npm")?;

            if !status.success() {
                bail!("npm install failed");
            }

            eprintln!("Updated successfully!");
        }
    }

    Ok(())
}

// ============================================================
// 单元测试
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_should_check_no_marker_file() {
        let temp = TempDir::new().unwrap();
        assert!(should_check(temp.path()));
    }

    #[test]
    fn test_should_check_fresh_marker() {
        let temp = TempDir::new().unwrap();
        let marker = temp.path().join("last_update_check");
        std::fs::write(&marker, "").unwrap();
        assert!(!should_check(temp.path()));
    }

    #[test]
    fn test_should_check_stale_marker() {
        let temp = TempDir::new().unwrap();
        let marker = temp.path().join("last_update_check");
        std::fs::write(&marker, "").unwrap();

        // 设置 mtime 为 25 小时前
        let old_time = SystemTime::now() - Duration::from_secs(25 * 60 * 60);
        filetime::set_file_mtime(&marker, filetime::FileTime::from_system_time(old_time)).unwrap();

        assert!(should_check(temp.path()));
    }

    #[test]
    fn test_mark_checked_creates_marker() {
        let temp = TempDir::new().unwrap();
        let marker = temp.path().join("last_update_check");
        assert!(!marker.exists());

        mark_checked(temp.path()).unwrap();

        assert!(marker.exists());
    }

    #[test]
    fn test_compare_versions_same() {
        assert!(!compare_versions("0.4.5", "0.4.5"));
    }

    #[test]
    fn test_compare_versions_older() {
        assert!(!compare_versions("0.4.5", "0.4.4"));
    }

    #[test]
    fn test_compare_versions_newer() {
        assert!(compare_versions("0.4.5", "0.4.6"));
        assert!(compare_versions("0.4.5", "0.5.0"));
        assert!(compare_versions("0.4.5", "1.0.0"));
    }

    #[test]
    fn test_compare_versions_edge_cases() {
        assert!(compare_versions("0.4", "0.4.1"));
        assert!(!compare_versions("0.4.1", "0.4"));
        assert!(compare_versions("0.9.9", "0.10.0"));
    }

    #[test]
    fn test_base_dir() {
        let dir = base_dir().unwrap();
        assert!(dir.ends_with(".agent-codemap"));
    }
}
