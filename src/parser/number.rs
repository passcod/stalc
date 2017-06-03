use nom::{ErrorKind, IResult};
use num::bigint::Sign;

#[derive(Debug, PartialEq, Eq)]
pub struct Number {
    pub sign: Sign,
    pub whole: String,
    pub decimal: String,
}

impl Number {
    pub fn parse(sign: Sign, whole: &str, decimal: &str) -> Self {
        Self {
            sign,
            whole: whole.into(),
            decimal: decimal.into(),
        }
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

named!(pub sign(&str) -> Sign, map!(opt!(alt!(
    one_of!("+")
  | one_of!("-")
)), Number::parse_sign));

named!(pub digits(&str) -> &str, is_a_s!("0123456789"));

pub fn digits_non_empty(d: &str) -> IResult<&str, &str> {
    if d.len() == 0 {
        IResult::Error(ErrorKind::Eof)
    } else {
        digits(d)
    }
}

named!(pub number(&str) -> Number, map!(do_parse!(
    sign: sign >>
    rational: alt_complete!(
        map!(do_parse!(
            dot: tag_s!(".") >>
            decimal: digits_non_empty >>
            (decimal)
        ), |decimal| {
            ("", decimal)
        })
      | do_parse!(
            whole: digits_non_empty >>
            dot: tag_s!(".") >>
            decimal: digits >>
            (whole, decimal)
        )
      | map!(digits_non_empty, |whole| {
            (whole, "")
        })
    ) >>
    eof!() >>
    (sign, rational)
), |tup: (Sign, (&str, &str))| {
    Number::parse(tup.0, (tup.1).0, (tup.1).1)
}));

#[cfg(test)]
mod test {
    use nom::{ErrorKind, IResult, Needed};
    use super::{number, Number, Sign};

    #[test]
    fn natural() {
        assert_eq!(
            IResult::Done("", Number {
                sign: Sign::NoSign,
                whole: "12407".into(),
                decimal: "".into()
            }),
            number("12407")
        )
    }

    #[test]
    fn decimal() {
        assert_eq!(
            IResult::Done("", Number {
                sign: Sign::NoSign,
                whole: "124".into(),
                decimal: "07".into()
            }),
            number("124.07")
        )
    }

    #[test]
    fn negative_decimal() {
        assert_eq!(
            IResult::Done("", Number {
                sign: Sign::Minus,
                whole: "124".into(),
                decimal: "07".into()
            }),
            number("-124.07")
        )
    }

    #[test]
    fn positive_decimal() {
        assert_eq!(
            IResult::Done("", Number {
                sign: Sign::Plus,
                whole: "124".into(),
                decimal: "07".into()
            }),
            number("+124.07")
        )
    }

    #[test]
    fn bare_decimal() {
        assert_eq!(
            IResult::Done("", Number {
                sign: Sign::NoSign,
                whole: "".into(),
                decimal: "3926".into()
            }),
            number(".3926")
        )
    }

    #[test]
    fn negative_bare_decimal() {
        assert_eq!(
            IResult::Done("", Number {
                sign: Sign::Minus,
                whole: "".into(),
                decimal: "3926".into()
            }),
            number("-.3926")
        )
    }

    #[test]
    fn positive_bare_decimal() {
        assert_eq!(
            IResult::Done("", Number {
                sign: Sign::Plus,
                whole: "".into(),
                decimal: "3926".into()
            }),
            number("+.3926")
        )
    }

    #[test]
    fn integer_with_leading_zeroes() {
        assert_eq!(
            IResult::Done("", Number {
                sign: Sign::NoSign,
                whole: "003926".into(),
                decimal: "".into()
            }),
            number("003926")
        )
    }

    #[test]
    fn decimal_with_leading_zeroes() {
        assert_eq!(
            IResult::Done("", Number {
                sign: Sign::NoSign,
                whole: "0".into(),
                decimal: "003926".into()
            }),
            number("0.003926")
        )
    }

    #[test]
    fn trailing_dot() {
        assert_eq!(
            IResult::Done("", Number {
                sign: Sign::NoSign,
                whole: "3926".into(),
                decimal: "".into()
            }),
            number("3926.")
        )
    }

    #[test]
    fn large_number() {
        assert_eq!(
            IResult::Done("", Number {
                sign: Sign::NoSign,
                whole: "2332428689328745932478943876394748356278545678928732409236889784596839084269459887432568783425".into(),
                decimal: "762489423675687397834274327568932457632856874683988987563257648239867843257684938587365425".into()
            }),
            number("2332428689328745932478943876394748356278545678928732409236889784596839084269459887432568783425.762489423675687397834274327568932457632856874683988987563257648239867843257684938587365425")
        )
    }

    #[test]
    fn invalid_sign_dot() {
        assert_eq!(
            IResult::Error(ErrorKind::Alt),
            number("-.")
        )
    }

    #[test]
    fn invalid_trailing_sign() {
        assert_eq!(
            IResult::Error(ErrorKind::Eof),
            number("927-")
        )
    }

    #[test]
    fn invalid_trailing_sign_dot() {
        assert_eq!(
            IResult::Error(ErrorKind::Eof),
            number("2816+.")
        )
    }

    #[test]
    fn invalid_sign_on_decimal() {
        assert_eq!(
            IResult::Error(ErrorKind::Eof),
            number("19263.-920")
        )
    }

    #[test]
    fn invalid_double_sign() {
        assert_eq!(
            IResult::Error(ErrorKind::Alt),
            number("++09826")
        )
    }

    #[test]
    fn invalid_garbage() {
        assert_eq!(
            IResult::Error(ErrorKind::Alt),
            number("$$%@")
        )
    }

    #[test]
    fn empty() {
        assert_eq!(
            IResult::Incomplete(Needed::Size(1)),
            number("")
        )
    }
}
