mod common;
mod parser;

use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let program = parser::parse(&buffer);
    println!("{:#?}", program);
    Ok(())
}