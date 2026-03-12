use std::io::{self, Read};

mod render;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    render::render(&input, &mut io::stdout()).unwrap();
}
