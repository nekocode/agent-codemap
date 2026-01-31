// ============================================================
// 集成测试: agent-codemap
// ============================================================

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

// ------------------------------------------------------------
// 测试工具函数
// ------------------------------------------------------------

fn run_cli(args: &[&str]) -> (String, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_agent-codemap"))
        .args(args)
        .output()
        .expect("Failed to execute command");

    (
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

fn fixtures_path(lang: &str, file: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(lang)
        .join(file)
}

fn expected_content(lang: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/expected")
        .join(format!("{}.md", lang));
    fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read expected: {}", path.display()))
}

/// 精确比对 stdout 与预期
fn assert_stdout_matches(stdout: &str, expected: &str) {
    let actual = stdout.trim();
    let expected = expected.trim();

    assert_eq!(
        actual, expected,
        "\n\nOutput mismatch!\n\nExpected:\n{}\n\nActual:\n{}\n",
        expected, actual
    );
}

// ------------------------------------------------------------
// 基本功能测试
// ------------------------------------------------------------

#[test]
fn test_single_file_input() {
    let file = fixtures_path("python", "basic.py");
    let (stdout, _) = run_cli(&[file.to_str().unwrap()]);

    assert_stdout_matches(&stdout, &expected_content("python"));
}

#[test]
fn test_directory_input() {
    let dir = fixtures_path("python", "basic.py").parent().unwrap().to_path_buf();
    let (stdout, _) = run_cli(&[dir.to_str().unwrap()]);

    assert_stdout_matches(&stdout, &expected_content("python"));
}

#[test]
fn test_json_format() {
    let file = fixtures_path("python", "basic.py");
    let (stdout, _) = run_cli(&[file.to_str().unwrap(), "-f", "json"]);

    // 验证是有效 JSON
    let parsed: serde_json::Value = serde_json::from_str(&stdout)
        .expect("Output should be valid JSON");

    assert!(parsed.is_array());
    let arr = parsed.as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["path"], "basic.py");
    assert_eq!(arr[0]["language"], "python");
}

#[test]
fn test_gitignore_respected() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path();

    std::process::Command::new("git")
        .args(["init"])
        .current_dir(input)
        .output()
        .ok();

    fs::write(input.join(".gitignore"), "ignored.rs\n").unwrap();
    fs::write(input.join("included.rs"), "fn included() {}").unwrap();
    fs::write(input.join("ignored.rs"), "fn ignored() {}").unwrap();

    let (stdout, _) = run_cli(&[input.to_str().unwrap()]);

    assert!(stdout.contains("# included.rs"));
    assert!(!stdout.contains("ignored.rs"));
}

#[test]
fn test_empty_directory() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("empty");
    fs::create_dir(&input).unwrap();

    let (stdout, stderr) = run_cli(&[input.to_str().unwrap()]);

    assert!(stdout.is_empty() || stdout.trim().is_empty());
    assert!(!stderr.contains("Error"));
}

#[test]
fn test_unsupported_file_type_ignored() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path();

    fs::write(input.join("readme.txt"), "Hello").unwrap();
    fs::write(input.join("test.rs"), "fn main() {}").unwrap();

    let (stdout, _) = run_cli(&[input.to_str().unwrap()]);

    assert!(stdout.contains("# test.rs"));
    assert!(!stdout.contains("readme.txt"));
}

#[test]
fn test_sorted_output() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path();
    let sub = input.join("sub");
    fs::create_dir(&sub).unwrap();

    fs::write(input.join("z.rs"), "fn z() {}").unwrap();
    fs::write(input.join("a.rs"), "fn a() {}").unwrap();
    fs::write(sub.join("m.rs"), "fn m() {}").unwrap();

    let (stdout, _) = run_cli(&[input.to_str().unwrap()]);

    // 验证顺序: sub/m.rs (目录优先), a.rs, z.rs
    let a_pos = stdout.find("# a.rs").unwrap();
    let z_pos = stdout.find("# z.rs").unwrap();
    let m_pos = stdout.find("# sub/m.rs").unwrap();

    assert!(m_pos < a_pos, "sub/m.rs should come before a.rs (dirs first)");
    assert!(a_pos < z_pos, "a.rs should come before z.rs");
}

#[test]
fn test_multiple_files_concatenated() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path();

    fs::write(input.join("a.rs"), "fn a() {}").unwrap();
    fs::write(input.join("b.rs"), "fn b() {}").unwrap();

    let (stdout, _) = run_cli(&[input.to_str().unwrap()]);

    // 两个文件都应该出现
    assert!(stdout.contains("# a.rs"));
    assert!(stdout.contains("# b.rs"));

    // 内容都应该存在
    assert!(stdout.contains("`a`"));
    assert!(stdout.contains("`b`"));
}

// ------------------------------------------------------------
// 语言支持测试 (精确比对)
// ------------------------------------------------------------

macro_rules! language_test {
    ($name:ident, $lang:expr, $file:expr) => {
        #[test]
        fn $name() {
            let file = fixtures_path($lang, $file);
            let (stdout, _) = run_cli(&[file.to_str().unwrap()]);
            assert_stdout_matches(&stdout, &expected_content($lang));
        }
    };
}

language_test!(test_python_output, "python", "basic.py");
language_test!(test_typescript_output, "typescript", "basic.ts");
language_test!(test_javascript_output, "javascript", "basic.js");
language_test!(test_go_output, "go", "basic.go");
language_test!(test_rust_output, "rust", "basic.rs");
language_test!(test_java_output, "java", "Basic.java");
language_test!(test_c_output, "c", "basic.c");
language_test!(test_cpp_output, "cpp", "basic.cpp");
language_test!(test_ruby_output, "ruby", "basic.rb");
language_test!(test_markdown_output, "markdown", "basic.md");
language_test!(test_swift_output, "swift", "basic.swift");
language_test!(test_objc_output, "objc", "basic.m");
language_test!(test_kotlin_output, "kotlin", "basic.kt");
language_test!(test_csharp_output, "csharp", "basic.cs");
