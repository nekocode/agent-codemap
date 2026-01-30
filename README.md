# agent-codemap

AI-friendly source code index generator. Extracts symbols from source files and outputs structured Markdown for LLM context.

## Install

```bash
# npm (recommended)
npm install -g @nekocode/agent-codemap

# From source
cargo install --path .
```

## Usage

```bash
# Generate index for a directory
agent-codemap ./src -o ./codemap

# Watch mode (regenerate on file changes)
agent-codemap ./src -o ./codemap -w
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

Output `codemap/user.py.md`:
```markdown
# src/user.py

Language: python

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

- Respects `.gitignore` automatically
- Preserves directory structure in output
- Watch mode for incremental updates
- Nested symbol extraction (methods inside classes, etc.)

## License

MIT
