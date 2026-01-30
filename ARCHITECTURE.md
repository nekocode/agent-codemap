# Architecture

> agent-codemap: AI 代码索引生成器

## 目录结构

```
agent-codemap/
├── src/
│   ├── main.rs          # 入口: CLI 解析 → 扫描 → 提取 → 输出
│   ├── cli.rs           # 命令行参数定义 (clap)
│   ├── scanner.rs       # 文件扫描 (自动 gitignore + 输出目录过滤)
│   ├── detector.rs      # 语言检测 (扩展名 → Language)
│   ├── extractor.rs     # Tree-sitter 符号提取核心
│   ├── symbol.rs        # 符号数据结构 (Symbol, FileMap)
│   ├── watch.rs         # Watch 模式 (增量更新 + 内存缓存)
│   ├── output/          # 输出格式化
│   │   ├── mod.rs       # 输出调度
│   │   └── markdown.rs  # Markdown 格式
│   └── queries/         # Tree-sitter 查询 (S-expression)
│       ├── python.scm
│       ├── typescript.scm
│       ├── javascript.scm
│       ├── go.scm
│       ├── rust.scm
│       ├── java.scm
│       ├── c.scm
│       ├── cpp.scm
│       ├── ruby.scm
│       ├── markdown.scm
│       ├── swift.scm
│       ├── objc.scm
│       ├── kotlin.scm
│       └── csharp.scm
└── tests/
    ├── integration_test.rs   # 集成测试 (精确文件比对)
    ├── expected/             # 预期输出 (每语言一个 .md)
    └── fixtures/             # 测试输入 (每语言一个 basic.*)
```

## 数据流

```
CLI 参数 (input_dir, output_dir)
    ↓
scanner::scan()      → Vec<PathBuf>  (自动过滤 .gitignore + output_dir)
    ↓
detector::detect()   → Language
    ↓
extractor::extract() → FileMap
    ↓
output::write_single() → output_dir/relative_path.md
```

## 模块依赖

```
main
 ├── cli          (参数解析)
 ├── scanner      (文件扫描)
 ├── detector     (语言检测)
 ├── extractor    (符号提取)
 │    ├── detector
 │    └── symbol
 ├── watch        (监听)
 │    ├── scanner
 │    ├── detector
 │    ├── extractor
 │    └── output
 └── output       (输出)
      └── symbol
```

## CLI 接口

```bash
agent-codemap <input_dir> -o <output_dir> [-w]
```

| 参数 | 说明 |
|------|------|
| `input_dir` | 输入目录 (默认: .) |
| `-o, --output` | 输出目录 (必填) |
| `-w, --watch` | Watch 模式 |

## 输出结构

输入目录结构会镜像到输出目录，文件名追加 `.md` 扩展名:

```
输入: src/components/Button.tsx
输出: out/components/Button.tsx.md

输入: ./foo.rs (当前目录)
输出: out/foo.rs.md
```

## 支持语言

| 语言 | 扩展名 | 查询文件 |
|------|--------|----------|
| Python | .py, .pyi | python.scm |
| TypeScript | .ts, .tsx | typescript.scm |
| JavaScript | .js, .mjs, .cjs, .jsx | javascript.scm |
| Go | .go | go.scm |
| Rust | .rs | rust.scm |
| Java | .java | java.scm |
| C | .c, .h | c.scm |
| C++ | .cpp, .cc, .cxx, .hpp, .hxx | cpp.scm |
| Ruby | .rb | ruby.scm |
| Markdown | .md, .markdown | markdown.scm |
| Swift | .swift | swift.scm |
| Objective-C | .m, .mm | objc.scm |
| Kotlin | .kt, .kts | kotlin.scm |
| C# | .cs | csharp.scm |

## 符号类型

```rust
enum SymbolKind {
    Class, Function, Method, Variable, Constant,
    Interface, Enum, Module, Property, Constructor,
    Field, EnumMember, Type, Namespace
}
```
