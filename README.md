# agent-codemap

English | [中文](./README_zh.md)

AI-friendly source code index generator. Extracts symbols from source files and outputs structured Markdown for LLM context.

![Cover](cover.jpg)

## Install

**Option 1: npm (recommended)**
```bash
npm install -g agent-codemap
```

**Option 2: Build from source**
```bash
cargo install --path .
```

## Usage

```bash
# Generate index for current directory (stdout)
agent-codemap .

# Generate index for a specific file
agent-codemap src/main.rs

# Output as JSON
agent-codemap . --format json

# Save to file
agent-codemap . > codemap.md
```

## Example Output

Input `src/user.py`:
```python
class User:
    def __init__(self, name):
        self.name = name

    def validate(self):
        return len(self.name) > 0

def create_user(name):
    return User(name)
```

Output:
```markdown
# src/user.py

- [class] `User` (line 1)
  - [method] `__init__` (line 2)
  - [method] `validate` (line 5)
- [function] `create_user` (line 9)
```

## Supported Languages

| Language | Extensions |
|----------|------------|
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

## Features

- Outputs to stdout (pipe-friendly)
- Supports Markdown and JSON formats
- Respects `.gitignore` automatically
- Nested symbol extraction (methods inside classes, etc.)

## License

MIT
