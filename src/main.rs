#[macro_use]
extern crate nom;
extern crate num;

use num::BigUint;
use num::bigint::Sign;
use std::io::{stdin, stdout, Write};
use std::str;

static WHITESPACE: &str = "
\u{09}
\u{04}
\u{0a}
\u{0b}
\u{0c}
\u{0d}
\u{20}
\u{85}
\u{a0}
\u{1680}
\u{2000}
\u{2001}
\u{2002}
\u{2003}
\u{2004}
\u{2005}
\u{2006}
\u{2007}
\u{2008}
\u{2009}
\u{200a}
\u{2028}
\u{2029}
\u{202f}
\u{205f}
\u{3000}
";

named!(space, eat_separator!(WHITESPACE));

#[macro_export]
macro_rules! uws (
  ($i:expr, $($args:tt)*) => (
    {
      sep!($i, space, $($args)*)
    }
  )
);

#[derive(Debug, PartialEq, Eq)]
struct Number {
    pub sign: Sign,
    pub whole: BigUint,
    pub decimal: BigUint,
}

#[derive(Debug, PartialEq, Eq)]
enum Lexed {
    Number(Number)
}

impl Lexed {
    fn parse_number(tup: (Sign, Vec<u32>, Option<Vec<u32>>)) -> Lexed {
        Lexed::Number(Number {
            sign: tup.0,
            whole: BigUint::new(tup.1),
            decimal: BigUint::new(tup.2.unwrap_or(vec![]))
        })
    }

    fn parse_digit(digit: char) -> u32 {
        digit.to_digit(10).unwrap()
    }

    fn parse_sign(sign: Option<char>) -> Sign {
        match sign {
            None => Sign::NoSign,
            Some(c) => match c {
                '+' => Sign::Plus,
                '-' => Sign::Minus,
                _ => unreachable!()
            }
        }
    }
}

named!(number<Lexed>, map!(
    do_parse!(
        sign: map!(opt!(alt!(
              one_of!("+")
            | one_of!("-")
        )), Lexed::parse_sign) >>
        whole: many1!(map!(
            one_of!("0123456789"),
            Lexed::parse_digit
        )) >>
        decimal: opt!(do_parse!(
            dot: tag!(".") >>
            digits: many1!(map!(
                one_of!("0123456789"),
                Lexed::parse_digit
            )) >>
            (digits)
        )) >>
        (sign, whole, decimal)
    ),
    Lexed::parse_number
));

named!(stalc<&[u8], Vec<Lexed> >,
    many0!(
        uws!(number)
    )
);

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
                println!("{:?}", stalc(input.as_ref()));
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
