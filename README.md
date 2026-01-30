# agent-codemap

English | [中文](./README_zh.md)

AI-friendly source code index generator. Extracts symbols from source files and outputs structured Markdown for LLM context.

![Cover](cover.jpg)

## Install

**Option 1: npm (recommended)**
```bash
npm install -g @nekocode/agent-codemap
```

**Option 2: Build from source**
```bash
cargo install --path .
```

## Usage

```bash
# Generate index for a directory
agent-codemap . -o ./.codemap

# Watch mode (regenerate on file changes)
agent-codemap . -o ./.codemap -w
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

Output `.codemap/src/user.py.md`:
```markdown
# OUTLINE

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


## Usage with AI Agents

**Step 1: Generate code index**

```bash
agent-codemap . -o ./.codemap
```

**Step 2: Add to your AI agent instructions**

Add the following to your `CLAUDE.md`, `AGENTS.md`, or equivalent:

````markdown
## Code Outline

The `.codemap/` directory contains outline files for each source file. Path rule: source file's relative path + `.md` suffix.

Example: `src/A.js` → `.codemap/src/A.js.md`

Outline files contain AST structure info, format example:
```
# OUTLINE

- [class] `User` (line 12)
  - [variable] `id` (line 15)
  - [method] `authenticate` (line 28)
- [function] `create_session` (line 45)
```
````

**Tip**: Add `.codemap/` to `.gitignore` if you prefer not to commit the index.

## License

MIT
