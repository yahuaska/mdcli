use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

mod render;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut rendered = Vec::new();
    render::render(&input, &mut rendered).unwrap();

    if atty::is(atty::Stream::Stdout) {
        if pipe_to_pager(&rendered).is_err() {
            io::stdout().write_all(&rendered).unwrap();
        }
    } else {
        io::stdout().write_all(&rendered).unwrap();
    }
}

fn pipe_to_pager(content: &[u8]) -> io::Result<()> {
    let pager = std::env::var("PAGER").unwrap_or_else(|_| "less".to_string());
    let mut parts = pager.split_whitespace();
    let cmd = parts.next().unwrap_or("less");
    let args: Vec<&str> = parts.collect();

    // For less, ensure -R is present for ANSI color support
    let mut final_args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    if cmd == "less" && !final_args.iter().any(|a| a.contains('R')) {
        final_args.push("-R".to_string());
    }

    let mut child = Command::new(cmd)
        .args(&final_args)
        .stdin(Stdio::piped())
        .spawn()?;

    child.stdin.take().unwrap().write_all(content)?;
    child.wait()?;
    Ok(())
}
