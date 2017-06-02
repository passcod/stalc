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

#[cfg(test)]
mod test {
    use nom::{ErrorKind, IResult, Needed};
    use num::BigUint;
    use num::bigint::Sign;
    use super::{number, Number};

    named!(spaced_number<Number>, ws!(number));

    #[test]
    fn natural() {
        assert_eq!(
            IResult::Done(&[][..], Number {
                sign: Sign::NoSign,
                whole: BigUint::new(vec![1, 2, 4, 0, 7]),
                decimal: BigUint::new(vec![])
            }),
            spaced_number("12407 ".as_bytes())
        )
    }

    #[test]
    fn decimal() {
        assert_eq!(
            IResult::Done(&[][..], Number {
                sign: Sign::NoSign,
                whole: BigUint::new(vec![1, 2, 4]),
                decimal: BigUint::new(vec![0, 7])
            }),
            number("124.07".as_bytes())
        )
    }

    #[test]
    fn negative_decimal() {
        assert_eq!(
            IResult::Done(&[][..], Number {
                sign: Sign::Minus,
                whole: BigUint::new(vec![1, 2, 4]),
                decimal: BigUint::new(vec![0, 7])
            }),
            number("-124.07".as_bytes())
        )
    }

    #[test]
    fn positive_decimal() {
        assert_eq!(
            IResult::Done(&[][..], Number {
                sign: Sign::Plus,
                whole: BigUint::new(vec![1, 2, 4]),
                decimal: BigUint::new(vec![0, 7])
            }),
            number("+124.07".as_bytes())
        )
    }

    #[test]
    fn bare_decimal() {
        assert_eq!(
            IResult::Done(&[][..], Number {
                sign: Sign::NoSign,
                whole: BigUint::new(vec![]),
                decimal: BigUint::new(vec![3, 9, 2, 6])
            }),
            number(".3926".as_bytes())
        )
    }

    #[test]
    fn negative_bare_decimal() {
        assert_eq!(
            IResult::Done(&[][..], Number {
                sign: Sign::Minus,
                whole: BigUint::new(vec![]),
                decimal: BigUint::new(vec![3, 9, 2, 6])
            }),
            number("-.3926".as_bytes())
        )
    }

    #[test]
    fn positive_bare_decimal() {
        assert_eq!(
            IResult::Done(&[][..], Number {
                sign: Sign::Plus,
                whole: BigUint::new(vec![]),
                decimal: BigUint::new(vec![3, 9, 2, 6])
            }),
            number("+.3926".as_bytes())
        )
    }

    // TODO: test cases for various failures
    // e.g. `-.`, `123.`, `12-.`, `123.-12`, ``, `$$$`
}
