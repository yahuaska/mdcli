# 003: Code Blocks, Lists, Links, Images

## Behavior

### Fenced Code Blocks

- Rendered with dim style `\x1b[2m`.
- Content output as-is (no inner Markdown parsing).
- Ends with `\x1b[0m` and two newlines.
- Language tag ignored (no syntax highlighting).

### Lists

- Unordered: bullet character `\u{2022}` followed by space.
- Ordered: number + `.` + space, incrementing from the start number.
- Nested lists indent 2 spaces per level.
- List ends with a blank line (when top-level).

### Links

- Link text rendered underlined `\x1b[4m`.
- URL shown after text in parentheses: `text (url)`.

### Images

- Alt text shown in brackets with dim style: `\x1b[2m[alt text]\x1b[0m`.

## Rationale

These elements appear frequently in real-world Markdown and are needed for useful rendering.
