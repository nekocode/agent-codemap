// ============================================================
// Scanner: 文件扫描 (支持单文件/目录，自动 gitignore)
// ============================================================

use anyhow::Result;
use ignore::WalkBuilder;
use std::cmp::Ordering;
use std::path::{Path, PathBuf};

/// 扫描输入路径，返回所有源码文件
/// - 单文件: 直接返回
/// - 目录: 递归扫描，尊重 .gitignore
/// - 排序: 目录深度优先，同级按名字字典序
pub fn scan(input: &Path) -> Result<Vec<PathBuf>> {
    let input_abs = input.canonicalize().unwrap_or_else(|_| input.to_path_buf());

    // 单文件直接返回
    if input_abs.is_file() {
        return Ok(vec![input_abs]);
    }

    let mut files = Vec::new();
    let walker = WalkBuilder::new(&input_abs)
        .hidden(true)
        .git_ignore(true)
        .build();

    for entry in walker.flatten() {
        let entry_path = entry.path();
        if entry_path.is_file() {
            files.push(entry_path.to_path_buf());
        }
    }

    // 排序: 目录深度优先，同级按名字字典序
    files.sort_by(|a, b| compare_paths(a, b, &input_abs));

    Ok(files)
}

/// 计算相对路径
pub fn relative_path(input: &Path, file: &Path) -> PathBuf {
    let input_abs = input.canonicalize().unwrap_or_else(|_| input.to_path_buf());
    let file_abs = file.canonicalize().unwrap_or_else(|_| file.to_path_buf());

    // 单文件输入: 返回文件名
    if input_abs.is_file() {
        return file_abs
            .file_name()
            .map(PathBuf::from)
            .unwrap_or_else(|| file_abs.clone());
    }

    // 目录输入: 返回相对路径
    file_abs
        .strip_prefix(&input_abs)
        .map(PathBuf::from)
        .unwrap_or_else(|_| file_abs)
}

/// 路径比较: 同级目录优先于文件，各自按字典序
fn compare_paths(a: &Path, b: &Path, base: &Path) -> Ordering {
    let rel_a = a.strip_prefix(base).unwrap_or(a);
    let rel_b = b.strip_prefix(base).unwrap_or(b);

    let comps_a: Vec<_> = rel_a.components().collect();
    let comps_b: Vec<_> = rel_b.components().collect();

    // 逐级比较
    for i in 0..comps_a.len().min(comps_b.len()) {
        let ca = &comps_a[i];
        let cb = &comps_b[i];

        if ca != cb {
            // 检查是否有更深层级（即当前组件是目录）
            let a_has_more = i + 1 < comps_a.len();
            let b_has_more = i + 1 < comps_b.len();

            // 目录优先于文件
            if a_has_more && !b_has_more {
                return Ordering::Less;
            }
            if !a_has_more && b_has_more {
                return Ordering::Greater;
            }

            // 都是目录或都是文件，按字典序
            return ca.cmp(cb);
        }
    }

    // 前缀相同，短路径优先（理论上不会到这里，因为文件路径不会互为前缀）
    comps_a.len().cmp(&comps_b.len())
}

// ============================================================
// 单元测试
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_scan_single_file() {
        let tmp = TempDir::new().unwrap();
        let file = tmp.path().join("main.rs");
        fs::write(&file, "fn main() {}").unwrap();

        let result = scan(&file).unwrap();
        assert_eq!(result.len(), 1);
        assert!(result[0].to_string_lossy().ends_with("main.rs"));
    }

    #[test]
    fn test_scan_directory() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("a.rs"), "").unwrap();
        fs::write(tmp.path().join("b.py"), "").unwrap();

        let result = scan(tmp.path()).unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_scan_sorted_dirs_first() {
        let tmp = TempDir::new().unwrap();
        let sub = tmp.path().join("sub");
        fs::create_dir(&sub).unwrap();

        fs::write(tmp.path().join("z.rs"), "").unwrap();
        fs::write(tmp.path().join("a.rs"), "").unwrap();
        fs::write(sub.join("m.rs"), "").unwrap();

        let result = scan(tmp.path()).unwrap();
        let rel_paths: Vec<_> = result
            .iter()
            .map(|p| {
                p.strip_prefix(tmp.path().canonicalize().unwrap())
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
            })
            .collect();

        // 目录优先: sub/m.rs 在前，然后 a.rs, z.rs
        assert_eq!(rel_paths, vec!["sub/m.rs", "a.rs", "z.rs"]);
    }

    #[test]
    fn test_relative_path_single_file() {
        let tmp = TempDir::new().unwrap();
        let file = tmp.path().join("main.rs");
        fs::write(&file, "").unwrap();

        let rel = relative_path(&file, &file);
        assert_eq!(rel, PathBuf::from("main.rs"));
    }

    #[test]
    fn test_relative_path_directory() {
        let tmp = TempDir::new().unwrap();
        let sub = tmp.path().join("src");
        fs::create_dir(&sub).unwrap();
        let file = sub.join("lib.rs");
        fs::write(&file, "").unwrap();

        let rel = relative_path(tmp.path(), &file);
        assert_eq!(rel, PathBuf::from("src/lib.rs"));
    }

    #[test]
    fn test_scan_respects_gitignore() {
        let tmp = TempDir::new().unwrap();

        // Init git repo
        std::process::Command::new("git")
            .args(["init"])
            .current_dir(tmp.path())
            .output()
            .ok();

        fs::write(tmp.path().join(".gitignore"), "ignored.rs\n").unwrap();
        fs::write(tmp.path().join("included.rs"), "").unwrap();
        fs::write(tmp.path().join("ignored.rs"), "").unwrap();

        let result = scan(tmp.path()).unwrap();
        assert_eq!(result.len(), 1);
        assert!(result[0].to_string_lossy().ends_with("included.rs"));
    }
}
