// ============================================================
// Symbol: 符号数据结构
// ============================================================

use serde::{Deserialize, Serialize};

// ------------------------------------------------------------
// 符号类型枚举
// ------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolKind {
    Class,
    Function,
    Method,
    Variable,
    Constant,
    Interface,
    Enum,
    Module,
    Property,
    Constructor,
    Field,
    EnumMember,
    Type,
    Namespace,
}

impl SymbolKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Class => "class",
            Self::Function => "function",
            Self::Method => "method",
            Self::Variable => "variable",
            Self::Constant => "constant",
            Self::Interface => "interface",
            Self::Enum => "enum",
            Self::Module => "module",
            Self::Property => "property",
            Self::Constructor => "constructor",
            Self::Field => "field",
            Self::EnumMember => "enum_member",
            Self::Type => "type",
            Self::Namespace => "namespace",
        }
    }
}

// ------------------------------------------------------------
// 位置信息
// ------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

// ------------------------------------------------------------
// 符号定义
// ------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub detail: Option<String>,
    pub range: Range,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub children: Vec<Symbol>,
}

// ------------------------------------------------------------
// 文件 Map
// ------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMap {
    pub path: String,
    pub language: String,
    pub symbols: Vec<Symbol>,
}

// ============================================================
// 单元测试
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_kind_as_str() {
        assert_eq!(SymbolKind::Class.as_str(), "class");
        assert_eq!(SymbolKind::Function.as_str(), "function");
        assert_eq!(SymbolKind::Method.as_str(), "method");
        assert_eq!(SymbolKind::Variable.as_str(), "variable");
        assert_eq!(SymbolKind::Constant.as_str(), "constant");
        assert_eq!(SymbolKind::Interface.as_str(), "interface");
        assert_eq!(SymbolKind::Enum.as_str(), "enum");
        assert_eq!(SymbolKind::Module.as_str(), "module");
        assert_eq!(SymbolKind::Property.as_str(), "property");
        assert_eq!(SymbolKind::Constructor.as_str(), "constructor");
        assert_eq!(SymbolKind::Field.as_str(), "field");
        assert_eq!(SymbolKind::EnumMember.as_str(), "enum_member");
        assert_eq!(SymbolKind::Type.as_str(), "type");
        assert_eq!(SymbolKind::Namespace.as_str(), "namespace");
    }
}
