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

impl Number {
    fn parse(sign: Sign, whole: Vec<u32>, decimal: Vec<u32>) -> Self {
        Self {
            sign,
            whole: BigUint::new(whole),
            decimal: BigUint::new(decimal)
        }
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

#[derive(Debug, PartialEq, Eq)]
enum Type {
    Number(Number)
}

named!(sign<Sign>, map!(opt!(alt!(
      one_of!("+")
    | one_of!("-")
)), Number::parse_sign));

named!(digits<Vec<u32> >, many1!(map!(
    one_of!("0123456789"),
    Number::parse_digit
)));

named!(number<Number>, alt!(
    map!(do_parse!(
        sign: sign >>
        whole: digits >>
        decimal: opt!(do_parse!(
            dot: tag!(".") >>
            digits: digits >>
            (digits)
        )) >>
        (sign, whole, decimal)
    ), |tup: (Sign, Vec<u32>, Option<Vec<u32>>)| {
        Number::parse(tup.0, tup.1, tup.2.unwrap_or(vec![]))
    })
  | map!(do_parse!(
        sign: sign >>
        dot: tag!(".") >>
        decimal: digits >>
        (sign, decimal)
    ), |tup: (Sign, Vec<u32>)| {
        Number::parse(tup.0, vec![], tup.1)
    })
));

named!(stalc<&[u8], Vec<Type> >,
    many0!(
        uws!(map!(number, |n| { Type::Number(n) }))
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
