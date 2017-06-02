use std::io::{stdin, stdout, Write};

fn main() {
    println!("Stalc {}", env!("CARGO_PKG_VERSION"));

    loop {
        print!("> ");
        stdout().flush().unwrap_or(());

        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{} bytes read", n);
                println!("{}", input);
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
