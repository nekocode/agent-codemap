// ============================================================
// Extractor: Tree-sitter 符号提取
// ============================================================

use crate::detector::Language;
use crate::symbol::{FileMap, Position, Range, Symbol, SymbolKind};
use anyhow::{Context, Result};
use std::collections::HashSet;
use std::path::Path;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Parser, Query, QueryCursor};

// ------------------------------------------------------------
// 公开接口
// ------------------------------------------------------------

/// 从文件提取符号
pub fn extract(path: &Path, lang: &Language) -> Result<FileMap> {
    let code = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    let symbols = extract_symbols(&code, lang)?;

    Ok(FileMap {
        path: path.to_string_lossy().to_string(),
        language: lang.as_str().to_string(),
        symbols,
    })
}

// ------------------------------------------------------------
// 内部实现
// ------------------------------------------------------------

/// 扁平符号 (带原始范围信息用于嵌套计算)
struct FlatSymbol {
    name: String,
    kind: SymbolKind,
    start_byte: usize,
    end_byte: usize,
    range: Range,
}

fn extract_symbols(code: &str, lang: &Language) -> Result<Vec<Symbol>> {
    let mut parser = Parser::new();
    let ts_lang = get_language(lang);
    parser.set_language(&ts_lang)?;

    let tree = parser
        .parse(code, None)
        .context("Tree-sitter parse failed")?;

    let query_src = get_query_source(lang);
    let query = Query::new(&ts_lang, query_src)?;
    let mut cursor = QueryCursor::new();

    // 第一步: 收集扁平符号列表
    let mut flat_symbols = Vec::new();
    let mut seen = HashSet::new();
    let mut matches = cursor.matches(&query, tree.root_node(), code.as_bytes());
    let name_idx = query.capture_index_for_name("name");

    while let Some(m) = matches.next() {
        let mut name_text = String::new();
        let mut kind = SymbolKind::Function;
        let mut start_byte = 0usize;
        let mut end_byte = 0usize;
        let mut range = None;

        for cap in m.captures {
            let node = cap.node;
            let text = node.utf8_text(code.as_bytes()).unwrap_or("");
            let cap_name = query.capture_names()[cap.index as usize];

            // 跳过以 _ 开头的辅助 capture
            if cap_name.starts_with('_') {
                continue;
            }

            if Some(cap.index) == name_idx {
                name_text = text.to_string();
            } else {
                kind = parse_kind(cap_name);
                start_byte = node.start_byte();
                end_byte = node.end_byte();
                range = Some(Range {
                    start: Position {
                        line: node.start_position().row + 1,
                        column: node.start_position().column,
                    },
                    end: Position {
                        line: node.end_position().row + 1,
                        column: node.end_position().column,
                    },
                });
            }
        }

        if !name_text.is_empty() {
            if let Some(r) = range {
                let key = (name_text.clone(), r.start.line);
                if seen.insert(key) {
                    flat_symbols.push(FlatSymbol {
                        name: name_text,
                        kind,
                        start_byte,
                        end_byte,
                        range: r,
                    });
                }
            }
        }
    }

    // 第二步: 按 start_byte 排序
    flat_symbols.sort_by_key(|s| s.start_byte);

    // 第三步: 构建嵌套树
    Ok(build_nested_tree(flat_symbols))
}

/// 基于范围包含关系构建嵌套树
fn build_nested_tree(flat_symbols: Vec<FlatSymbol>) -> Vec<Symbol> {
    if flat_symbols.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();
    let mut stack: Vec<(Symbol, usize)> = Vec::new(); // (symbol, end_byte)

    for flat in flat_symbols {
        let symbol = Symbol {
            name: flat.name,
            kind: flat.kind,
            detail: None,
            range: flat.range,
            children: Vec::new(),
        };

        // 弹出所有已结束的父符号
        while let Some((_, parent_end)) = stack.last() {
            if flat.start_byte >= *parent_end {
                let (completed, _) = stack.pop().unwrap();
                if let Some((parent, _)) = stack.last_mut() {
                    parent.children.push(completed);
                } else {
                    result.push(completed);
                }
            } else {
                break;
            }
        }

        // 检查是否应该作为当前栈顶的子节点
        if let Some((_, parent_end)) = stack.last() {
            if flat.end_byte <= *parent_end {
                // 当前符号在父符号范围内，入栈
                stack.push((symbol, flat.end_byte));
                continue;
            }
        }

        // 否则作为顶层符号入栈
        stack.push((symbol, flat.end_byte));
    }

    // 清空栈
    while let Some((completed, _)) = stack.pop() {
        if let Some((parent, _)) = stack.last_mut() {
            parent.children.push(completed);
        } else {
            result.push(completed);
        }
    }

    result
}

fn parse_kind(capture_name: &str) -> SymbolKind {
    if capture_name.contains("enum_member") {
        SymbolKind::EnumMember
    } else if capture_name.contains("enum") {
        SymbolKind::Enum
    } else if capture_name.contains("class") {
        SymbolKind::Class
    } else if capture_name.contains("method") {
        SymbolKind::Method
    } else if capture_name.contains("function") {
        SymbolKind::Function
    } else if capture_name.contains("interface") {
        SymbolKind::Interface
    } else if capture_name.contains("field") {
        SymbolKind::Field
    } else if capture_name.contains("property") {
        SymbolKind::Property
    } else if capture_name.contains("variable") {
        SymbolKind::Variable
    } else if capture_name.contains("constant") {
        SymbolKind::Constant
    } else if capture_name.contains("module") {
        SymbolKind::Module
    } else if capture_name.contains("namespace") {
        SymbolKind::Namespace
    } else if capture_name.contains("type") {
        SymbolKind::Type
    } else {
        SymbolKind::Function
    }
}

fn get_language(lang: &Language) -> tree_sitter::Language {
    match lang {
        Language::Python => tree_sitter_python::LANGUAGE.into(),
        Language::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        Language::Tsx => tree_sitter_typescript::LANGUAGE_TSX.into(),
        Language::JavaScript => tree_sitter_javascript::LANGUAGE.into(),
        Language::Jsx => tree_sitter_javascript::LANGUAGE.into(),
        Language::Go => tree_sitter_go::LANGUAGE.into(),
        Language::Rust => tree_sitter_rust::LANGUAGE.into(),
        Language::Java => tree_sitter_java::LANGUAGE.into(),
        Language::C => tree_sitter_c::LANGUAGE.into(),
        Language::Cpp => tree_sitter_cpp::LANGUAGE.into(),
        Language::Ruby => tree_sitter_ruby::LANGUAGE.into(),
        Language::Markdown => tree_sitter_md::LANGUAGE.into(),
        Language::Swift => tree_sitter_swift::LANGUAGE.into(),
        Language::ObjC => tree_sitter_objc::LANGUAGE.into(),
        Language::Kotlin => tree_sitter_kotlin_ng::LANGUAGE.into(),
        Language::CSharp => tree_sitter_c_sharp::LANGUAGE.into(),
    }
}

fn get_query_source(lang: &Language) -> &'static str {
    match lang {
        Language::Python => include_str!("queries/python.scm"),
        Language::TypeScript | Language::Tsx => include_str!("queries/typescript.scm"),
        Language::JavaScript | Language::Jsx => include_str!("queries/javascript.scm"),
        Language::Go => include_str!("queries/go.scm"),
        Language::Rust => include_str!("queries/rust.scm"),
        Language::Java => include_str!("queries/java.scm"),
        Language::C => include_str!("queries/c.scm"),
        Language::Cpp => include_str!("queries/cpp.scm"),
        Language::Ruby => include_str!("queries/ruby.scm"),
        Language::Markdown => include_str!("queries/markdown.scm"),
        Language::Swift => include_str!("queries/swift.scm"),
        Language::ObjC => include_str!("queries/objc.scm"),
        Language::Kotlin => include_str!("queries/kotlin.scm"),
        Language::CSharp => include_str!("queries/csharp.scm"),
    }
}
