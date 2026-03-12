use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Parser, Tag, TagEnd};
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

pub fn render(markdown: &str, writer: &mut dyn Write) -> std::io::Result<()> {
    let parser = Parser::new(markdown);
    let mut style_stack: Vec<&str> = Vec::new();
    let mut list_stack: Vec<Option<u64>> = Vec::new();
    let mut in_image = false;
    let mut link_url: Option<String> = None;

    for event in parser {
        match event {
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
