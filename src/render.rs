use pulldown_cmark::{Alignment, CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use std::io::Write;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const ITALIC: &str = "\x1b[3m";
const REVERSE: &str = "\x1b[7m";
const DIM: &str = "\x1b[2m";

fn heading_style(level: HeadingLevel) -> &'static str {
    match level {
        HeadingLevel::H1 => "\x1b[1;97m",
        HeadingLevel::H2 => "\x1b[1;36m",
        HeadingLevel::H3 => "\x1b[1;32m",
        HeadingLevel::H4 => "\x1b[1;33m",
        HeadingLevel::H5 => "\x1b[1;34m",
        HeadingLevel::H6 => "\x1b[1;35m",
    }
}

/// Strip ANSI escape sequences to get the visible length of a string.
fn visible_len(s: &str) -> usize {
    let mut len = 0;
    let mut in_escape = false;
    for c in s.chars() {
        if in_escape {
            if c.is_ascii_alphabetic() {
                in_escape = false;
            }
        } else if c == '\x1b' {
            in_escape = true;
        } else {
            len += 1;
        }
    }
    len
}

/// Pad a styled string to `width` visible characters according to alignment.
fn pad_cell(styled: &str, width: usize, alignment: &Alignment) -> String {
    let vis = visible_len(styled);
    if vis >= width {
        return styled.to_string();
    }
    let padding = width - vis;
    match alignment {
        Alignment::Right => format!("{}{}", " ".repeat(padding), styled),
        Alignment::Center => {
            let left = padding / 2;
            let right = padding - left;
            format!("{}{}{}", " ".repeat(left), styled, " ".repeat(right))
        }
        _ => format!("{}{}", styled, " ".repeat(padding)),
    }
}

struct TableState {
    alignments: Vec<Alignment>,
    rows: Vec<Vec<String>>, // each row is a vec of styled cell strings
    current_row: Vec<String>,
    current_cell: Vec<u8>, // buffer for rendering current cell
    is_head: bool,
}

fn render_table(table: &TableState, writer: &mut dyn Write) -> std::io::Result<()> {
    let num_cols = table.alignments.len();

    // Compute column widths
    let mut widths = vec![0usize; num_cols];
    for row in &table.rows {
        for (i, cell) in row.iter().enumerate() {
            if i < num_cols {
                widths[i] = widths[i].max(visible_len(cell));
            }
        }
    }

    // Draw top border
    write!(writer, "┌")?;
    for (i, w) in widths.iter().enumerate() {
        write!(writer, "{}", "─".repeat(w + 2))?;
        write!(writer, "{}", if i + 1 < num_cols { "┬" } else { "┐" })?;
    }
    writeln!(writer)?;

    for (row_idx, row) in table.rows.iter().enumerate() {
        // Data row
        write!(writer, "│")?;
        for (i, w) in widths.iter().enumerate() {
            let cell = row.get(i).map(|s| s.as_str()).unwrap_or("");
            let aligned = pad_cell(cell, *w, &table.alignments[i]);
            if row_idx == 0 {
                write!(writer, " {}{}{} │", BOLD, aligned, RESET)?;
            } else {
                write!(writer, " {} │", aligned)?;
            }
        }
        writeln!(writer)?;

        // Separator after header
        if row_idx == 0 {
            write!(writer, "├")?;
            for (i, w) in widths.iter().enumerate() {
                write!(writer, "{}", "─".repeat(w + 2))?;
                write!(writer, "{}", if i + 1 < num_cols { "┼" } else { "┤" })?;
            }
            writeln!(writer)?;
        }
    }

    // Bottom border
    write!(writer, "└")?;
    for (i, w) in widths.iter().enumerate() {
        write!(writer, "{}", "─".repeat(w + 2))?;
        write!(writer, "{}", if i + 1 < num_cols { "┴" } else { "┘" })?;
    }
    writeln!(writer)?;
    writeln!(writer)?;

    Ok(())
}

pub fn render(markdown: &str, writer: &mut dyn Write) -> std::io::Result<()> {
    let parser = Parser::new_ext(markdown, Options::ENABLE_TABLES);
    let mut style_stack: Vec<&str> = Vec::new();
    let mut list_stack: Vec<Option<u64>> = Vec::new();
    let mut in_image = false;
    let mut link_url: Option<String> = None;
    let mut table: Option<TableState> = None;

    for event in parser {
        // When inside a table, buffer cell contents
        if let Some(ref mut tbl) = table {
            match event {
                Event::Start(Tag::TableHead) => {
                    tbl.is_head = true;
                    tbl.current_row = Vec::new();
                }
                Event::End(TagEnd::TableHead) => {
                    tbl.rows.push(std::mem::take(&mut tbl.current_row));
                    tbl.is_head = false;
                }
                Event::Start(Tag::TableRow) => {
                    tbl.current_row = Vec::new();
                }
                Event::End(TagEnd::TableRow) => {
                    tbl.rows.push(std::mem::take(&mut tbl.current_row));
                }
                Event::Start(Tag::TableCell) => {
                    tbl.current_cell = Vec::new();
                }
                Event::End(TagEnd::TableCell) => {
                    let styled = String::from_utf8(std::mem::take(&mut tbl.current_cell))
                        .unwrap_or_default();
                    tbl.current_row.push(styled);
                }
                Event::End(TagEnd::Table) => {
                    let tbl = table.take().unwrap();
                    render_table(&tbl, writer)?;
                    continue;
                }
                // Render inline content into the cell buffer
                Event::Text(text) => {
                    write!(tbl.current_cell, "{}", text)?;
                }
                Event::Code(text) => {
                    write!(tbl.current_cell, "{}{}{}", REVERSE, text, RESET)?;
                }
                Event::Start(Tag::Strong) => {
                    write!(tbl.current_cell, "{}", BOLD)?;
                }
                Event::End(TagEnd::Strong) => {
                    write!(tbl.current_cell, "{}", RESET)?;
                }
                Event::Start(Tag::Emphasis) => {
                    write!(tbl.current_cell, "{}", ITALIC)?;
                }
                Event::End(TagEnd::Emphasis) => {
                    write!(tbl.current_cell, "{}", RESET)?;
                }
                _ => {}
            }
            continue;
        }

        match event {
            Event::Start(Tag::Table(alignments)) => {
                table = Some(TableState {
                    alignments,
                    rows: Vec::new(),
                    current_row: Vec::new(),
                    current_cell: Vec::new(),
                    is_head: false,
                });
            }
            Event::Start(Tag::Heading { level, .. }) => {
                style_stack.push(heading_style(level));
                write!(writer, "{}", heading_style(level))?;
            }
            Event::End(TagEnd::Heading(_)) => {
                style_stack.pop();
                write!(writer, "{}\n\n", RESET)?;
            }
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(_) | CodeBlockKind::Indented)) => {
                write!(writer, "{}", DIM)?;
            }
            Event::End(TagEnd::CodeBlock) => {
                write!(writer, "{}\n\n", RESET)?;
            }
            Event::Start(Tag::List(start)) => {
                list_stack.push(start);
            }
            Event::End(TagEnd::List(_)) => {
                list_stack.pop();
                if list_stack.is_empty() {
                    writeln!(writer)?;
                }
            }
            Event::Start(Tag::Item) => {
                let indent = "  ".repeat(list_stack.len().saturating_sub(1));
                match list_stack.last_mut() {
                    Some(Some(n)) => {
                        write!(writer, "{}{}. ", indent, n)?;
                        *n += 1;
                    }
                    _ => {
                        write!(writer, "{}\u{2022} ", indent)?;
                    }
                }
            }
            Event::End(TagEnd::Item) => {
                writeln!(writer)?;
            }
            Event::Start(Tag::Paragraph) => {}
            Event::End(TagEnd::Paragraph) => {
                write!(writer, "\n\n")?;
            }
            Event::Start(Tag::Emphasis) => {
                style_stack.push(ITALIC);
                write!(writer, "{}", ITALIC)?;
            }
            Event::End(TagEnd::Emphasis) => {
                style_stack.pop();
                write!(writer, "{}", RESET)?;
                for s in &style_stack {
                    write!(writer, "{}", s)?;
                }
            }
            Event::Start(Tag::Strong) => {
                style_stack.push(BOLD);
                write!(writer, "{}", BOLD)?;
            }
            Event::End(TagEnd::Strong) => {
                style_stack.pop();
                write!(writer, "{}", RESET)?;
                for s in &style_stack {
                    write!(writer, "{}", s)?;
                }
            }
            Event::Start(Tag::Link { dest_url, .. }) => {
                const UNDERLINE: &str = "\x1b[4m";
                style_stack.push(UNDERLINE);
                write!(writer, "{}", UNDERLINE)?;
                link_url = Some(dest_url.to_string());
            }
            Event::End(TagEnd::Link) => {
                style_stack.pop();
                write!(writer, "{}", RESET)?;
                if let Some(url) = link_url.take() {
                    write!(writer, " ({})", url)?;
                }
                for s in &style_stack {
                    write!(writer, "{}", s)?;
                }
            }
            Event::Start(Tag::Image { .. }) => {
                in_image = true;
                write!(writer, "{}", DIM)?;
            }
            Event::End(TagEnd::Image) => {
                in_image = false;
                write!(writer, "{}", RESET)?;
            }
            Event::Code(text) => {
                write!(writer, "{}{}{}", REVERSE, text, RESET)?;
                for s in &style_stack {
                    write!(writer, "{}", s)?;
                }
            }
            Event::Text(text) => {
                if in_image {
                    write!(writer, "[{}]", text)?;
                } else {
                    write!(writer, "{}", text)?;
                }
            }
            Event::SoftBreak => {
                writeln!(writer)?;
            }
            Event::HardBreak => {
                writeln!(writer)?;
            }
            _ => {}
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visible_len_plain() {
        assert_eq!(visible_len("hello"), 5);
    }

    #[test]
    fn visible_len_ansi() {
        assert_eq!(visible_len("\x1b[1mbold\x1b[0m"), 4);
    }

    #[test]
    fn pad_cell_left() {
        assert_eq!(pad_cell("ab", 5, &Alignment::Left), "ab   ");
        assert_eq!(pad_cell("ab", 5, &Alignment::None), "ab   ");
    }

    #[test]
    fn pad_cell_right() {
        assert_eq!(pad_cell("ab", 5, &Alignment::Right), "   ab");
    }

    #[test]
    fn pad_cell_center() {
        assert_eq!(pad_cell("ab", 5, &Alignment::Center), " ab  ");
        assert_eq!(pad_cell("ab", 6, &Alignment::Center), "  ab  ");
    }
}
