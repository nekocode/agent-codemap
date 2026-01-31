// ============================================================
// Markdown 输出
// ============================================================

use crate::symbol::{FileMap, Symbol};

/// 渲染单个文件的符号 (标题为相对路径)
pub fn render(map: &FileMap) -> String {
    let mut out = String::new();

    out.push_str(&format!("# {}\n\n", map.path));

    for sym in &map.symbols {
        render_symbol(&mut out, sym, 0);
    }

    out
}

/// 渲染多个文件，按顺序拼接
pub fn render_all(maps: &[FileMap]) -> String {
    maps.iter()
        .map(render)
        .collect::<Vec<_>>()
        .join("\n")
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
