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

## Usage with AI Agents

### Just ask the agent

The simplest approach - just tell your agent to use it:

```
Use agent-codemap to understand the project structure. Run agent-codemap --help for usage.
```

The `--help` output is comprehensive and most agents can figure it out from there.

### AGENTS.md / CLAUDE.md

For more consistent results, add to your project instructions:

````markdown
## Code Structure

Use `agent-codemap` to get code outlines. Run `agent-codemap --help` for all options.

Usage:
- `agent-codemap .` - Scan entire project
- `agent-codemap src/` - Scan specific directory
- `agent-codemap path/to/file.py` - Scan single file

Output format (each file as a section):
```
# path/to/file.py

- [class] `ClassName` (line N)
  - [method] `method_name` (line N)
  - [variable] `var_name` (line N)
- [function] `func_name` (line N)
```

Symbol types: class, function, method, variable, constant, interface, enum, module, property, constructor, field, type, namespace
````

## License

MIT
