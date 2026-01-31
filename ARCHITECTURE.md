# Architecture

> agent-codemap: AI 代码索引生成器

## 目录结构

```
agent-codemap/
├── src/
│   ├── main.rs          # 入口: CLI 解析 → 扫描 → 提取 → 输出
│   ├── cli.rs           # 命令行参数定义 (clap)
│   ├── scanner.rs       # 文件扫描 (支持单文件/目录，自动 gitignore)
│   ├── detector.rs      # 语言检测 (扩展名 → Language)
│   ├── extractor.rs     # Tree-sitter 符号提取核心
│   ├── symbol.rs        # 符号数据结构 (Symbol, FileMap)
│   ├── output/          # 输出格式化
│   │   ├── mod.rs       # 输出调度
│   │   ├── markdown.rs  # Markdown 格式
│   │   └── json.rs      # JSON 格式
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
├── npm/                 # npm 分发包 (@nekocode/*)
│   ├── agent-codemap/              # 主包 (JS wrapper)
│   ├── agent-codemap-darwin-arm64/ # macOS ARM64 二进制
│   ├── agent-codemap-darwin-x64/   # macOS x64 二进制
│   ├── agent-codemap-linux-x64/    # Linux x64 二进制
│   └── agent-codemap-win32-x64/    # Windows x64 二进制
├── scripts/
│   ├── build-npm.sh     # 构建 npm 包 (编译 + 复制二进制)
│   └── publish-npm.sh   # 发布 npm 包
└── tests/
    ├── integration_test.rs   # 集成测试 (stdout 比对)
    ├── expected/             # 预期输出 (每语言一个 .md)
    └── fixtures/             # 测试输入 (每语言一个 basic.*)
```

## 数据流

```
CLI 参数 (input, format)
    ↓
scanner::scan()          → Vec<PathBuf>  (支持单文件/目录，自动 gitignore)
    ↓
detector::detect()       → Language
    ↓
extractor::extract()     → FileMap
    ↓
output::render_all()     → stdout (Markdown 或 JSON)
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
 └── output       (输出)
      ├── cli     (OutputFormat)
      └── symbol
```

## CLI 接口

```bash
agent-codemap <input> [-f format]
```

| 参数 | 说明 |
|------|------|
| `input` | 输入文件或目录 (默认: .) |
| `-f, --format` | 输出格式: markdown (默认) 或 json |

## 输出格式

### Markdown

```markdown
# relative/path/file.ext

- [function] `main` (line 1)
- [class] `User` (line 5)
  - [method] `__init__` (line 6)
```

多文件按顺序拼接，排序规则：浅层优先，同深度按路径字典序。

### JSON

```json
[
  {
    "path": "relative/path/file.ext",
    "language": "python",
    "symbols": [...]
  }
]
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

## npm 分发

采用 esbuild 风格的 platform-specific packages 方案：

```bash
# 全局安装
npm install -g @nekocode/agent-codemap

# 或项目内安装
npm install @nekocode/agent-codemap
npx agent-codemap --help
```

### 构建与发布

```bash
# 构建当前平台
./scripts/build-npm.sh current

# 构建所有平台 (需要 cross)
./scripts/build-npm.sh all

# 发布 (先构建，再发布)
./scripts/publish-npm.sh 0.1.0
```

### 工作原理

1. 主包 `@nekocode/agent-codemap` 通过 `optionalDependencies` 引用平台包
2. npm 根据当前平台自动只安装匹配的平台包
3. JS wrapper 检测平台，调用对应二进制
