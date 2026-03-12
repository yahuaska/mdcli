# 006: Table Support

## Behavior

- Tables are rendered with Unicode box-drawing characters (`┌┬┐├┼┤└┴┘─│`).
- Column widths are computed from the widest cell in each column (ANSI codes excluded from width measurement).
- Each cell has 1 space of padding on each side.
- Header row is rendered bold.
- A horizontal separator (`├─┼─┤`) appears between the header and body rows.
- Alignment (left/center/right) from the Markdown separator row is respected.
- Inline formatting (bold, italic, code) inside cells is preserved.
- Requires `Options::ENABLE_TABLES` in pulldown-cmark parser.

## Rationale

Tables are common in documentation and need proper alignment to be readable in the terminal.
