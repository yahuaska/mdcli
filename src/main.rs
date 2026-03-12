use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    io::stdout().write_all(input.as_bytes()).unwrap();
}
