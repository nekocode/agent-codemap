# agent-codemap

[English](./README.md) | 中文

AI 友好的源码索引生成器。从源文件中提取符号，输出结构化 Markdown 供 LLM 使用。

![Cover](cover.jpg)

## 安装

**方式一：npm（推荐）**
```bash
npm install -g agent-codemap
```

**方式二：从源码构建**
```bash
cargo install --path .
```

## 使用

```bash
# 为当前目录生成索引（输出到 stdout）
agent-codemap .

# 为指定文件生成索引
agent-codemap src/main.rs

# 输出 JSON 格式
agent-codemap . --format json

# 保存到文件
agent-codemap . > codemap.md
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

输出：
```markdown
# src/user.py

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

- 输出到 stdout（管道友好）
- 支持 Markdown 和 JSON 格式
- 自动遵循 `.gitignore`
- 嵌套符号提取（类内方法等）

## 许可证

MIT
