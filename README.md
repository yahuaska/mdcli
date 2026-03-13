```
                 _      _ _
  _ __ ___   __| | ___| (_)
 | '_ ` _ \ / _` |/ __| | |
 | | | | | | (_| | (__| | |
 |_| |_| |_|\__,_|\___|_|_|
```

**Markdown, but in your terminal. Beautifully.**

---

A fast, zero-config CLI tool that renders Markdown right where you live — your terminal. No browser, no preview pane, no nonsense. Just gorgeous, readable Markdown with full Unicode box-drawing tables, ANSI colors, and automatic paging.

## Features

- 🎨 **Color-coded headings** — H1 through H6, each with a distinct color
- ✨ **Rich text** — bold, italic, and `inline code`
- 📦 **Fenced code blocks** — properly indented and styled
- 📝 **Lists** — ordered, unordered, and nested
- 🔗 **Links & images** — URLs displayed inline
- 📊 **Tables** — Unicode box-drawing borders with left/center/right alignment
- 📟 **Auto-pager** — pipes through `less` (or your `$PAGER`) for long docs
- 📥 **Flexible input** — read from a file or stdin

## Installation

### Download a release

Grab the latest binary from [Releases](../../releases) for your platform:

| Platform             | Architecture |
|----------------------|--------------|
| Linux                | x86_64       |
| Linux                | ARM64        |
| macOS                | Intel        |
| macOS                | Apple Silicon|
| Windows              | x86_64       |

### Build from source

```sh
cargo install --path .
```

## Usage

```sh
# Render a file
mdcli README.md

# Pipe from stdin
cat doc.md | mdcli

# Use a custom pager
PAGER=bat mdcli notes.md
```

## Example output

Here's what a table looks like in your terminal:

```
┌────────┬───────────┬────────┐
│ Name   │ Status    │  Score │
├────────┼───────────┼────────┤
│ Alice  │ Active    │    98  │
│ Bob    │ Pending   │    74  │
│ Carol  │ Active    │    87  │
└────────┴───────────┴────────┘
```

Full Unicode box-drawing. Proper column alignment. No compromises.

## Development

```sh
cargo build          # build
cargo test           # run all tests
cargo clippy         # lint
cargo fmt            # format
```

## License

TBD
