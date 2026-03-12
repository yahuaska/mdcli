# 002: Basic Markdown Rendering

## Behavior

mdcli parses Markdown from stdin using pulldown-cmark and renders to stdout with ANSI escape codes.

### Headings (H1–H6)

- Rendered bold with a distinct color per level:
  - H1: bright white `\x1b[1;97m`
  - H2: cyan `\x1b[1;36m`
  - H3: green `\x1b[1;32m`
  - H4: yellow `\x1b[1;33m`
  - H5: blue `\x1b[1;34m`
  - H6: magenta `\x1b[1;35m`
- Followed by reset `\x1b[0m` and two newlines.

### Inline styles

- **Bold**: `\x1b[1m` … `\x1b[0m`
- **Italic**: `\x1b[3m` … `\x1b[0m`
- **Inline code**: reverse video `\x1b[7m` … `\x1b[0m`
- Nested styles re-apply the remaining stack after reset.

### Paragraphs

- Paragraph text followed by `\n\n`.

### Breaks

- `SoftBreak` → `\n`
- `HardBreak` → `\n`

### Unhandled elements

- Silently ignored (no output).

## Rationale

Core feature: render Markdown readably in the terminal using standard ANSI codes.
