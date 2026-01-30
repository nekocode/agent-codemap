// ============================================================
// Markdown 输出
// ============================================================

use crate::symbol::{FileMap, Symbol};

/// 渲染单个文件的符号
pub fn render_single(map: &FileMap) -> String {
    let mut out = String::new();

    out.push_str("# OUTLINE\n\n");

    for sym in &map.symbols {
        render_symbol(&mut out, sym, 0);
    }

    out
}

fn render_symbol(out: &mut String, sym: &Symbol, depth: usize) {
    let indent = "  ".repeat(depth);
    let kind = sym.kind.as_str();
    let line = sym.range.start.line;

    out.push_str(&format!(
        "{}- [{}] `{}` (line {})\n",
        indent, kind, sym.name, line
    ));

    for child in &sym.children {
        render_symbol(out, child, depth + 1);
    }
}
