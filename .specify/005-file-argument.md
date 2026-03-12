# 005: File Argument

## Behavior

- `mdcli <file>` reads Markdown from the given file path.
- `mdcli` (no arguments) reads from stdin as before.
- If the file does not exist or cannot be read, prints `mdcli: <path>: <error>` to stderr and exits with code 1.

## Rationale

Matches the interface of `less` and `cat` — users expect to pass a filename directly.
