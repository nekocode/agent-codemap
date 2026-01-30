// ============================================================
// Detector: 语言检测
// ============================================================

use std::path::Path;

/// 支持的语言
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Python,
    TypeScript,
    Tsx,
    JavaScript,
    Jsx,
    Go,
    Rust,
    Java,
    C,
    Cpp,
    Ruby,
    Markdown,
    Swift,
    ObjC,
    Kotlin,
    CSharp,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Python => "python",
            Self::TypeScript => "typescript",
            Self::Tsx => "tsx",
            Self::JavaScript => "javascript",
            Self::Jsx => "jsx",
            Self::Go => "go",
            Self::Rust => "rust",
            Self::Java => "java",
            Self::C => "c",
            Self::Cpp => "cpp",
            Self::Ruby => "ruby",
            Self::Markdown => "markdown",
            Self::Swift => "swift",
            Self::ObjC => "objc",
            Self::Kotlin => "kotlin",
            Self::CSharp => "csharp",
        }
    }
}

/// 根据文件路径检测语言
pub fn detect(path: &Path) -> Option<Language> {
    let ext = path.extension()?.to_str()?;
    match ext {
        "py" | "pyi" => Some(Language::Python),
        "ts" => Some(Language::TypeScript),
        "tsx" => Some(Language::Tsx),
        "js" | "mjs" | "cjs" => Some(Language::JavaScript),
        "jsx" => Some(Language::Jsx),
        "go" => Some(Language::Go),
        "rs" => Some(Language::Rust),
        "java" => Some(Language::Java),
        "c" | "h" => Some(Language::C),
        "cpp" | "cc" | "cxx" | "hpp" | "hxx" => Some(Language::Cpp),
        "rb" => Some(Language::Ruby),
        "md" | "markdown" => Some(Language::Markdown),
        "swift" => Some(Language::Swift),
        "m" | "mm" => Some(Language::ObjC),
        "kt" | "kts" => Some(Language::Kotlin),
        "cs" => Some(Language::CSharp),
        _ => None,
    }
}

// ============================================================
// 单元测试
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_python() {
        assert_eq!(detect(Path::new("test.py")), Some(Language::Python));
        assert_eq!(detect(Path::new("test.pyi")), Some(Language::Python));
    }

    #[test]
    fn test_detect_typescript() {
        assert_eq!(detect(Path::new("test.ts")), Some(Language::TypeScript));
        assert_eq!(detect(Path::new("test.tsx")), Some(Language::Tsx));
    }

    #[test]
    fn test_detect_javascript() {
        assert_eq!(detect(Path::new("test.js")), Some(Language::JavaScript));
        assert_eq!(detect(Path::new("test.mjs")), Some(Language::JavaScript));
        assert_eq!(detect(Path::new("test.cjs")), Some(Language::JavaScript));
        assert_eq!(detect(Path::new("test.jsx")), Some(Language::Jsx));
    }

    #[test]
    fn test_detect_go() {
        assert_eq!(detect(Path::new("test.go")), Some(Language::Go));
    }

    #[test]
    fn test_detect_rust() {
        assert_eq!(detect(Path::new("test.rs")), Some(Language::Rust));
    }

    #[test]
    fn test_detect_java() {
        assert_eq!(detect(Path::new("Test.java")), Some(Language::Java));
    }

    #[test]
    fn test_detect_c() {
        assert_eq!(detect(Path::new("test.c")), Some(Language::C));
        assert_eq!(detect(Path::new("test.h")), Some(Language::C));
    }

    #[test]
    fn test_detect_cpp() {
        assert_eq!(detect(Path::new("test.cpp")), Some(Language::Cpp));
        assert_eq!(detect(Path::new("test.cc")), Some(Language::Cpp));
        assert_eq!(detect(Path::new("test.cxx")), Some(Language::Cpp));
        assert_eq!(detect(Path::new("test.hpp")), Some(Language::Cpp));
        assert_eq!(detect(Path::new("test.hxx")), Some(Language::Cpp));
    }

    #[test]
    fn test_detect_ruby() {
        assert_eq!(detect(Path::new("test.rb")), Some(Language::Ruby));
    }

    #[test]
    fn test_detect_markdown() {
        assert_eq!(detect(Path::new("test.md")), Some(Language::Markdown));
        assert_eq!(detect(Path::new("test.markdown")), Some(Language::Markdown));
    }

    #[test]
    fn test_detect_swift() {
        assert_eq!(detect(Path::new("test.swift")), Some(Language::Swift));
    }

    #[test]
    fn test_detect_objc() {
        assert_eq!(detect(Path::new("test.m")), Some(Language::ObjC));
        assert_eq!(detect(Path::new("test.mm")), Some(Language::ObjC));
    }

    #[test]
    fn test_detect_kotlin() {
        assert_eq!(detect(Path::new("test.kt")), Some(Language::Kotlin));
        assert_eq!(detect(Path::new("build.gradle.kts")), Some(Language::Kotlin));
    }

    #[test]
    fn test_detect_csharp() {
        assert_eq!(detect(Path::new("Program.cs")), Some(Language::CSharp));
    }

    #[test]
    fn test_detect_unknown() {
        assert_eq!(detect(Path::new("test.txt")), None);
        assert_eq!(detect(Path::new("test")), None);
    }

    #[test]
    fn test_language_as_str() {
        assert_eq!(Language::Python.as_str(), "python");
        assert_eq!(Language::TypeScript.as_str(), "typescript");
        assert_eq!(Language::Tsx.as_str(), "tsx");
        assert_eq!(Language::JavaScript.as_str(), "javascript");
        assert_eq!(Language::Jsx.as_str(), "jsx");
        assert_eq!(Language::Go.as_str(), "go");
        assert_eq!(Language::Rust.as_str(), "rust");
        assert_eq!(Language::Java.as_str(), "java");
        assert_eq!(Language::C.as_str(), "c");
        assert_eq!(Language::Cpp.as_str(), "cpp");
        assert_eq!(Language::Ruby.as_str(), "ruby");
        assert_eq!(Language::Markdown.as_str(), "markdown");
        assert_eq!(Language::Swift.as_str(), "swift");
        assert_eq!(Language::ObjC.as_str(), "objc");
        assert_eq!(Language::Kotlin.as_str(), "kotlin");
        assert_eq!(Language::CSharp.as_str(), "csharp");
    }
}
