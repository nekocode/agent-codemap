// ============================================================
// Scanner: 文件扫描 (自动 gitignore + 输出目录过滤)
// ============================================================

use anyhow::Result;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

/// 扫描目录，返回所有源码文件路径
/// - 自动读取 .gitignore
/// - 过滤输出目录
pub fn scan(input: &Path, output: &Path) -> Result<Vec<PathBuf>> {
    let input_abs = input.canonicalize().unwrap_or_else(|_| input.to_path_buf());
    let output_abs = output.canonicalize().unwrap_or_else(|_| {
        // 输出目录可能不存在，构造绝对路径
        if output.is_absolute() {
            output.to_path_buf()
        } else {
            std::env::current_dir()
                .unwrap_or_default()
                .join(output)
        }
    });

    let mut files = Vec::new();
    let walker = WalkBuilder::new(&input_abs)
        .hidden(true)      // 跳过隐藏文件
        .git_ignore(true)  // 尊重 .gitignore
        .build();

    for entry in walker.flatten() {
        let entry_path = entry.path();
        if !entry_path.is_file() {
            continue;
        }

        // 过滤输出目录
        let entry_abs = entry_path.canonicalize()
            .unwrap_or_else(|_| entry_path.to_path_buf());
        if entry_abs.starts_with(&output_abs) {
            continue;
        }

        files.push(entry_path.to_path_buf());
    }

    Ok(files)
}

/// 计算输出路径
/// input_dir: 输入目录
/// source_file: 源文件路径
/// output_dir: 输出目录
/// 返回: 输出文件路径 (.md)
pub fn output_path(input_dir: &Path, source_file: &Path, output_dir: &Path) -> PathBuf {
    let input_abs = input_dir.canonicalize()
        .unwrap_or_else(|_| input_dir.to_path_buf());
    let source_abs = source_file.canonicalize()
        .unwrap_or_else(|_| source_file.to_path_buf());

    // 计算相对路径
    let rel_path = source_abs.strip_prefix(&input_abs)
        .unwrap_or(&source_abs);

    // 构造输出路径，追加 .md 扩展名
    let file_name = rel_path.file_name()
        .map(|n| format!("{}.md", n.to_string_lossy()))
        .unwrap_or_else(|| "index.md".to_string());
    output_dir.join(rel_path.parent().unwrap_or(Path::new(""))).join(file_name)
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
    fn test_scan_directory() {
        let tmp = TempDir::new().unwrap();
        let out = tmp.path().join("out");
        fs::create_dir(&out).unwrap();

        fs::write(tmp.path().join("a.rs"), "").unwrap();
        fs::write(tmp.path().join("b.py"), "").unwrap();

        let result = scan(tmp.path(), &out).unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_scan_excludes_output_dir() {
        let tmp = TempDir::new().unwrap();
        let out = tmp.path().join("out");
        fs::create_dir(&out).unwrap();

        fs::write(tmp.path().join("a.rs"), "").unwrap();
        fs::write(out.join("b.md"), "").unwrap();

        let result = scan(tmp.path(), &out).unwrap();
        assert_eq!(result.len(), 1);
        assert!(result[0].to_string_lossy().ends_with("a.rs"));
    }

    #[test]
    fn test_output_path_simple() {
        let tmp = TempDir::new().unwrap();
        let input = tmp.path();
        let source = tmp.path().join("main.rs");
        fs::write(&source, "").unwrap();
        let output = tmp.path().join("out");

        let result = output_path(input, &source, &output);
        assert!(result.to_string_lossy().ends_with("main.rs.md"));
    }

    #[test]
    fn test_output_path_nested() {
        let tmp = TempDir::new().unwrap();
        let input = tmp.path();
        let sub = tmp.path().join("components");
        fs::create_dir(&sub).unwrap();
        let source = sub.join("Button.js");
        fs::write(&source, "").unwrap();
        let output = tmp.path().join("out");

        let result = output_path(input, &source, &output);
        assert!(result.to_string_lossy().contains("components"));
        assert!(result.to_string_lossy().ends_with("Button.js.md"));
    }

    #[test]
    fn test_scan_relative_output_not_exist() {
        let tmp = TempDir::new().unwrap();
        let saved_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(tmp.path()).unwrap();

        fs::write(tmp.path().join("a.rs"), "").unwrap();

        // Output dir doesn't exist yet, relative path
        let result = scan(tmp.path(), Path::new("out"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);

        std::env::set_current_dir(saved_dir).unwrap();
    }

    #[test]
    fn test_scan_absolute_output_not_exist() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("a.rs"), "").unwrap();

        // Absolute output path that doesn't exist
        let out = tmp.path().join("nonexistent_out");
        let result = scan(tmp.path(), &out);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }
}
