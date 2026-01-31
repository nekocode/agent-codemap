// ============================================================
// JSON 输出
// ============================================================

use crate::symbol::FileMap;

/// 渲染为 JSON
pub fn render_all(maps: &[FileMap]) -> String {
    serde_json::to_string_pretty(maps).unwrap_or_else(|_| "[]".to_string())
}
