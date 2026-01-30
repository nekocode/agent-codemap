# agent-codemap

[English](./README.md) | 中文

AI 友好的源码索引生成器。从源文件中提取符号，输出结构化 Markdown 供 LLM 使用。

![Cover](cover.jpg)

## 安装

**方式一：npm（推荐）**
```bash
npm install -g @nekocode/agent-codemap
```

**方式二：从源码构建**
```bash
cargo install --path .
```

## 使用

```bash
# 为目录生成索引
agent-codemap . -o ./.codemap

# 监听模式（文件变动时自动重新生成）
agent-codemap . -o ./.codemap -w
```

## 输出示例

输入 `src/user.py`：
```python
class User:
    def __init__(self, name):
        self.name = name

    def validate(self):
        return len(self.name) > 0

def create_user(name):
    return User(name)
```

输出 `.codemap/src/user.py.md`：
```markdown
# OUTLINE

- [class] `User` (line 1)
  - [method] `__init__` (line 2)
  - [method] `validate` (line 5)
- [function] `create_user` (line 9)
```

## 支持的语言

| 语言 | 扩展名 |
|------|--------|
| Python | .py, .pyi |
| TypeScript | .ts, .tsx |
| JavaScript | .js, .mjs, .cjs, .jsx |
| Go | .go |
| Rust | .rs |
| Java | .java |
| C | .c, .h |
| C++ | .cpp, .cc, .cxx, .hpp, .hxx |
| Ruby | .rb |
| Swift | .swift |
| Objective-C | .m, .mm |
| Kotlin | .kt, .kts |
| C# | .cs |
| Markdown | .md, .markdown |

## 特性

- 自动遵循 `.gitignore`
- 输出保持目录结构
- 监听模式支持增量更新
- 嵌套符号提取（类内方法等）

## 与 AI Agent 配合使用

**第一步：生成代码索引**

```bash
agent-codemap . -o ./.codemap
```

**第二步：添加到 AI Agent 指令中**

将以下内容添加到你的 `CLAUDE.md`、`AGENTS.md` 或类似文件：

````markdown
## 代码大纲

在 `.codemap/` 目录下，每个源码文件都有对应的大纲文件，路径规则为：源码的相对路径 + `.md` 后缀。

例如：`src/A.js` → `.codemap/src/A.js.md`

大纲文件包含 AST 结构信息，格式示例：
```
# OUTLINE

- [class] `User` (line 12)
  - [variable] `id` (line 15)
  - [method] `authenticate` (line 28)
- [function] `create_session` (line 45)
```
````

**提示**：如果不想提交索引文件，可将 `.codemap/` 添加到 `.gitignore`。

## 许可证

MIT
