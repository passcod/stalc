use num::BigUint;
use num::bigint::Sign;

#[derive(Debug, PartialEq, Eq)]
pub struct Number {
    pub sign: Sign,
    pub whole: BigUint,
    pub decimal: BigUint,
}

impl Number {
    pub fn parse(sign: Sign, whole: Vec<u32>, decimal: Vec<u32>) -> Self {
        Self {
            sign,
            whole: BigUint::new(whole),
            decimal: BigUint::new(decimal)
        }
    }

    pub fn parse_digit(digit: char) -> u32 {
        digit.to_digit(10).unwrap()
    }

    pub fn parse_sign(sign: Option<char>) -> Sign {
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

named!(pub sign<Sign>, map!(opt!(alt!(
    one_of!("+")
  | one_of!("-")
)), Number::parse_sign));

named!(pub digits<Vec<u32> >, many1!(map!(
    one_of!("0123456789"),
    Number::parse_digit
)));

named!(pub number<Number>, alt!(
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
