# 004: Pager Support

## Behavior

- When stdout is a terminal, rendered output is piped through a pager.
- Pager is determined by `$PAGER` env var, falling back to `less`.
- When using `less`, `-R` is automatically added (if not already present) to support ANSI escape codes.
- `$PAGER` may contain arguments (e.g. `less -X`), which are split on whitespace.
- If the pager fails to start, output is written directly to stdout.
- When stdout is not a terminal (piped), output goes directly to stdout (no pager).

## Rationale

Long Markdown documents should be scrollable. Paging by default matches the behavior of `man` and `git log`.
