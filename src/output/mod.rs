// ============================================================
// Output: Markdown 输出
// ============================================================

mod markdown;

use crate::symbol::FileMap;
use anyhow::Result;
use std::fs;
use std::path::Path;

/// 渲染单个文件的符号到 markdown
pub fn render_single(map: &FileMap) -> String {
    markdown::render_single(map)
}

/// 将单个文件的符号写入输出文件
pub fn write_single(map: &FileMap, output_path: &Path) -> Result<()> {
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = render_single(map);
    fs::write(output_path, content)?;
    Ok(())
}

// ============================================================
// 单元测试
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol::{FileMap, Position, Range, Symbol, SymbolKind};
    use tempfile::TempDir;

    fn make_map(path: &str) -> FileMap {
        FileMap {
            path: path.to_string(),
            language: "rust".to_string(),
            symbols: vec![Symbol {
                name: "test".to_string(),
                kind: SymbolKind::Function,
                detail: None,
                range: Range {
                    start: Position { line: 1, column: 0 },
                    end: Position { line: 1, column: 10 },
                },
                children: vec![],
            }],
        }
    }

    #[test]
    fn test_render_single() {
        let map = make_map("test.rs");
        let output = render_single(&map);
        assert!(output.contains("# OUTLINE"));
        assert!(output.contains("[function]"));
        assert!(output.contains("`test`"));
    }

    #[test]
    fn test_write_single() {
        let tmp = TempDir::new().unwrap();
        let output = tmp.path().join("sub/dir/test.rs.md");
        let map = make_map("test.rs");

        write_single(&map, &output).unwrap();
        assert!(output.exists());

        let content = fs::read_to_string(&output).unwrap();
        assert!(content.contains("# OUTLINE"));
    }
}
