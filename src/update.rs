// ============================================================
// update - 版本更新检查
// ============================================================

use std::time::Duration;

use anyhow::{bail, Context, Result};

const VERSION: &str = env!("CARGO_PKG_VERSION");

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
}
