#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;
extern crate num;

use std::io::{stdin, stdout, Write};

mod parser;

fn main() {
    println!("Stalc {}", env!("CARGO_PKG_VERSION"));

    loop {
        print!("> ");
        stdout().flush().unwrap_or(());

        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{} bytes read", n);
                println!("{:?}", input);
                stdout().flush().unwrap_or(());
                println!("{:?}", parser::stalc(input.as_ref()));
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
