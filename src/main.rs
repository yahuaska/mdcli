use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

mod render;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let input = if args.is_empty() {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).unwrap();
        buf
    } else {
        let path = &args[0];
        match std::fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("mdcli: {}: {}", path, e);
                std::process::exit(1);
            }
        }
    };

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
