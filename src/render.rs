use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};
use std::io::Write;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const ITALIC: &str = "\x1b[3m";
const REVERSE: &str = "\x1b[7m";

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
            Event::Code(text) => {
                write!(writer, "{}{}{}", REVERSE, text, RESET)?;
                for s in &style_stack {
                    write!(writer, "{}", s)?;
                }
            }
            Event::Text(text) => {
                write!(writer, "{}", text)?;
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
