// ============================================================
// 集成测试: agent-codemap
// ============================================================

use std::fs;
use std::path::{Path, PathBuf};
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

fn expected_path(lang: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/expected")
        .join(format!("{}.md", lang))
}

/// 提取输出内容（跳过路径行）
fn extract_content(output: &str) -> String {
    output.lines().skip(2).collect::<Vec<_>>().join("\n")
}

/// 精确比对输出与预期文件
fn assert_output_matches(output_file: &Path, expected_file: &Path) {
    let output = fs::read_to_string(output_file)
        .unwrap_or_else(|_| panic!("Failed to read output: {}", output_file.display()));
    let expected = fs::read_to_string(expected_file)
        .unwrap_or_else(|_| panic!("Failed to read expected: {}", expected_file.display()));

    let output_content = extract_content(&output);
    let expected_content = expected.trim();

    assert_eq!(
        output_content, expected_content,
        "\n\nOutput mismatch!\n\nExpected:\n{}\n\nActual:\n{}\n",
        expected_content, output_content
    );
}

// ------------------------------------------------------------
// 基本功能测试
// ------------------------------------------------------------

#[test]
fn test_single_file_output() {
    let tmp = TempDir::new().unwrap();
    let out_dir = tmp.path().join("out");

    run_cli(&[
        fixtures_path("python", "basic.py").parent().unwrap().to_str().unwrap(),
        "-o",
        out_dir.to_str().unwrap(),
    ]);

    let output_file = out_dir.join("basic.py.md");
    assert!(output_file.exists(), "Output file should exist");
    assert_output_matches(&output_file, &expected_path("python"));
}

#[test]
fn test_directory_structure_preserved() {
    let tmp = TempDir::new().unwrap();
    let out_dir = tmp.path().join("out");
    let input = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");

    run_cli(&[
        input.to_str().unwrap(),
        "-o",
        out_dir.to_str().unwrap(),
    ]);

    // 检查目录结构保留
    assert!(out_dir.join("python").exists());
    assert!(out_dir.join("typescript").exists());
    assert!(out_dir.join("javascript").exists());
    assert!(out_dir.join("go").exists());
    assert!(out_dir.join("rust").exists());

    // 检查具体文件
    assert!(out_dir.join("python/basic.py.md").exists());
    assert!(out_dir.join("typescript/basic.ts.md").exists());
}

#[test]
fn test_output_dir_excluded() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path();
    let out_dir = tmp.path().join("out");

    fs::write(input.join("test.rs"), "fn main() {}").unwrap();
    fs::create_dir(&out_dir).unwrap();
    fs::write(out_dir.join("existing.md"), "# existing").unwrap();

    run_cli(&[
        input.to_str().unwrap(),
        "-o",
        out_dir.to_str().unwrap(),
    ]);

    assert!(out_dir.join("test.rs.md").exists());

    // existing.md 不应该被修改
    let content = fs::read_to_string(out_dir.join("existing.md")).unwrap();
    assert_eq!(content, "# existing");
}

#[test]
fn test_gitignore_respected() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path();
    let out_dir = tmp.path().join("out");

    std::process::Command::new("git")
        .args(["init"])
        .current_dir(input)
        .output()
        .ok();

    fs::write(input.join(".gitignore"), "ignored.rs\n").unwrap();
    fs::write(input.join("included.rs"), "fn included() {}").unwrap();
    fs::write(input.join("ignored.rs"), "fn ignored() {}").unwrap();

    run_cli(&[
        input.to_str().unwrap(),
        "-o",
        out_dir.to_str().unwrap(),
    ]);

    assert!(out_dir.join("included.rs.md").exists());
    assert!(!out_dir.join("ignored.rs.md").exists());
}

#[test]
fn test_empty_directory() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("empty");
    let out_dir = tmp.path().join("out");

    fs::create_dir(&input).unwrap();

    let (_, stderr) = run_cli(&[
        input.to_str().unwrap(),
        "-o",
        out_dir.to_str().unwrap(),
    ]);

    assert!(!stderr.contains("Error"));
}

#[test]
fn test_unsupported_file_type() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path();
    let out_dir = tmp.path().join("out");

    fs::write(input.join("readme.txt"), "Hello").unwrap();
    fs::write(input.join("test.rs"), "fn main() {}").unwrap();

    run_cli(&[
        input.to_str().unwrap(),
        "-o",
        out_dir.to_str().unwrap(),
    ]);

    assert!(!out_dir.join("readme.txt.md").exists());
    assert!(out_dir.join("test.rs.md").exists());
}

// ------------------------------------------------------------
// 语言支持测试 (精确比对)
// ------------------------------------------------------------

macro_rules! language_test {
    ($name:ident, $lang:expr, $file:expr) => {
        #[test]
        fn $name() {
            let tmp = TempDir::new().unwrap();
            let out_dir = tmp.path().join("out");

            run_cli(&[
                fixtures_path($lang, $file).parent().unwrap().to_str().unwrap(),
                "-o",
                out_dir.to_str().unwrap(),
            ]);

            let output_file = out_dir.join(format!("{}.md", $file));
            assert!(output_file.exists(), "Output file {} should exist", output_file.display());
            assert_output_matches(&output_file, &expected_path($lang));
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
