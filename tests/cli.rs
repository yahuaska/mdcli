use std::io::Write;
use std::process::{Command, Output};

fn run_with_file(path: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_mdcli"))
        .arg(path)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .and_then(|child| child.wait_with_output())
        .unwrap()
}

fn run_with_input(input: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_mdcli"))
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            child
                .stdin
                .take()
                .unwrap()
                .write_all(input.as_bytes())
                .unwrap();
            child.wait_with_output()
        })
        .unwrap()
}

#[test]
fn plain_text_paragraph() {
    let output = run_with_input("hello\n");
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "hello\n\n");
}

#[test]
fn heading_h1() {
    let output = run_with_input("# Hello\n");
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "\x1b[1;97mHello\x1b[0m\n\n"
    );
}

#[test]
fn heading_h2_different_color() {
    let output = run_with_input("## World\n");
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "\x1b[1;36mWorld\x1b[0m\n\n"
    );
}

#[test]
fn bold_text() {
    let output = run_with_input("**bold**\n");
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "\x1b[1mbold\x1b[0m\n\n"
    );
}

#[test]
fn italic_text() {
    let output = run_with_input("*italic*\n");
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "\x1b[3mitalic\x1b[0m\n\n"
    );
}

#[test]
fn inline_code() {
    let output = run_with_input("`code`\n");
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "\x1b[7mcode\x1b[0m\n\n"
    );
}

#[test]
fn paragraph_separation() {
    let output = run_with_input("first\n\nsecond\n");
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "first\n\nsecond\n\n"
    );
}

#[test]
fn fenced_code_block() {
    let output = run_with_input("```\nhello world\n```\n");
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "\x1b[2mhello world\n\x1b[0m\n\n"
    );
}

#[test]
fn fenced_code_block_with_language() {
    let output = run_with_input("```rust\nfn main() {}\n```\n");
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "\x1b[2mfn main() {}\n\x1b[0m\n\n"
    );
}

#[test]
fn code_block_multiline() {
    let output = run_with_input("```\nline1\nline2\n```\n");
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "\x1b[2mline1\nline2\n\x1b[0m\n\n"
    );
}

#[test]
fn unordered_list() {
    let output = run_with_input("* first\n* second\n");
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "\u{2022} first\n\u{2022} second\n\n"
    );
}

#[test]
fn ordered_list() {
    let output = run_with_input("1. first\n2. second\n");
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "1. first\n2. second\n\n"
    );
}

#[test]
fn link() {
    let output = run_with_input("[click here](https://example.com)\n");
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "\x1b[4mclick here\x1b[0m (https://example.com)\n\n"
    );
}

#[test]
fn image_alt_text() {
    let output = run_with_input("![diagram](pic.png)\n");
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "\x1b[2m[diagram]\x1b[0m\n\n"
    );
}

#[test]
fn heading_then_list_spacing() {
    let output = run_with_input("## Title\n* item\n");
    assert!(output.status.success());
    let out = String::from_utf8_lossy(&output.stdout);
    assert!(
        out.contains("\x1b[0m\n\n\u{2022}"),
        "heading should have blank line before list"
    );
}

#[test]
fn read_file_argument() {
    let dir = std::env::temp_dir().join("mdcli_test_read_file");
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("test.md");
    std::fs::write(&file, "# Hello\n").unwrap();

    let output = run_with_file(file.to_str().unwrap());
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "\x1b[1;97mHello\x1b[0m\n\n"
    );

    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn file_not_found_error() {
    let output = run_with_file("/tmp/mdcli_nonexistent_file.md");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("mdcli:"));
    assert!(stderr.contains("No such file"));
}

#[test]
fn simple_table() {
    let output = run_with_input("| A | B |\n|---|---|\n| c | d |\n");
    assert!(output.status.success());
    let out = String::from_utf8_lossy(&output.stdout);
    // Check box-drawing borders
    assert!(out.contains("┌"));
    assert!(out.contains("┘"));
    // Header row should be bold
    assert!(out.contains("\x1b[1m"));
    // Cell content present
    assert!(out.contains("A"));
    assert!(out.contains("d"));
}

#[test]
fn table_column_widths() {
    let output = run_with_input("| Short | X |\n|---|---|\n| Longer text | Y |\n");
    assert!(output.status.success());
    let out = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = out.lines().collect();
    // Top border (first line) and bottom border (last non-empty line) should have same length
    let top = lines.first().unwrap();
    let bottom = lines.iter().rev().find(|l| !l.is_empty()).unwrap();
    assert!(top.starts_with('┌'));
    assert!(bottom.starts_with('└'));
    assert_eq!(top.len(), bottom.len());
}

#[test]
fn table_right_alignment() {
    let output = run_with_input("| N |\n|--:|\n| x |\n| longer |\n");
    assert!(output.status.success());
    let out = String::from_utf8_lossy(&output.stdout);
    // "x" should be right-padded with spaces on the left
    // Find the data row with "x" - it should have leading spaces
    let x_line = out
        .lines()
        .find(|l| l.contains("x") && l.contains("│"))
        .unwrap();
    let cell_content = x_line.split('│').nth(1).unwrap();
    // Should have leading spaces for right alignment
    assert!(
        cell_content.starts_with("  "),
        "expected right-aligned padding, got: {:?}",
        cell_content
    );
}
