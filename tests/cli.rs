use std::io::Write;
use std::process::{Command, Output};

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
