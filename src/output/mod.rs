// ============================================================
// Output: 输出格式化 (Markdown / JSON)
// ============================================================

mod json;
mod markdown;

use crate::cli::OutputFormat;
use crate::symbol::FileMap;

/// 渲染所有文件
pub fn render_all(maps: &[FileMap], format: OutputFormat) -> String {
    match format {
        OutputFormat::Markdown => markdown::render_all(maps),
        OutputFormat::Json => json::render_all(maps),
    }
}

// ============================================================
// 单元测试
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol::{Position, Range, Symbol, SymbolKind};

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
    fn test_render_markdown() {
        let maps = vec![make_map("test.rs")];
        let output = render_all(&maps, OutputFormat::Markdown);
        assert!(output.contains("# test.rs"));
        assert!(output.contains("[function]"));
        assert!(output.contains("`test`"));
    }

    #[test]
    fn test_render_json() {
        let maps = vec![make_map("test.rs")];
        let output = render_all(&maps, OutputFormat::Json);
        assert!(output.contains("\"path\": \"test.rs\""));
        assert!(output.contains("\"name\": \"test\""));
    }

    #[test]
    fn test_render_multiple_files() {
        let maps = vec![make_map("a.rs"), make_map("b.rs")];
        let output = render_all(&maps, OutputFormat::Markdown);
        assert!(output.contains("# a.rs"));
        assert!(output.contains("# b.rs"));
    }
}
