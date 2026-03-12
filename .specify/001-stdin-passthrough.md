# 001: Stdin Passthrough

## Behavior

- `mdcli` reads Markdown text from stdin until EOF.
- Writes it to stdout unchanged (no rendering yet).
- Exits with code 0 on success.

## Rationale

Walking skeleton — proves the binary works end-to-end before adding rendering.
