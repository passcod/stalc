use num::bigint::Sign;
use num::rational::BigRational;
use super::numeral::{decimal, decimals, decimals_ne, digits, digits_ne, parse_str_radix};

#[derive(Debug, PartialEq, Eq)]
pub struct Number {
    pub sign: Sign,
    pub rational: BigRational,
    pub debug: String,
    pub radix: u8,
}

impl Number {
    pub fn parse(sign: Sign, integer: String, fractional: String, radix: u8) -> Result<Self, String> {
        let rational = parse_str_radix(integer, fractional, radix)?;
        let debug = format!("{}", rational);

        Ok(Self {
            sign,
            rational,
            debug,
            radix
        })
    }
}

named!(pub sign(&str) -> Sign, map!(opt!(alt!(
    tag_s!("+")
  | tag_s!("-")
)), |sign: Option<&str>| {
    sign.map(|s| match s {
        "+" => Sign::Plus,
        "-" => Sign::Minus,
        _ => unreachable!()
    }).unwrap_or(Sign::NoSign)
}));

named!(pub radixpoint(&str) -> &str, alt!(
    tag_s!(".")
  | tag_s!(",")
));

named!(pub radix(&str) -> u8, map!(do_parse!(
    tag_s!("_") >>
    r: many_m_n!(1, 2, decimal) >>
    (r)
), |r: Vec<char>| {
    let rad: String = r.into_iter().collect();
    rad.parse::<u8>().unwrap()
}));

named!(pub number(&str) -> Number, map_res!(do_parse!(
    sign: sign >>
    numerated: alt_complete!(
        // Radixed .123
        map!(do_parse!(
            radixpoint >>
            fractional: digits_ne >>
            radix: radix >>
            (fractional, radix)
        ), |t: (&str, u8)| { (None, Some(t.0.into()), Some(t.1)) })

        // Radixed 12.3 or 12.
      | map!(do_parse!(
            integer: digits_ne >>
            radixpoint >>
            fractional: digits >>
            radix: radix >>
            (integer, fractional, radix)
        ), |t: (&str, &str, u8)| { (Some(t.0.into()), Some(t.1.into()), Some(t.2)) })

        // Radixed 123
      | map!(do_parse!(
            integer: digits_ne >>
            radix: radix >>
            (integer, radix)
        ), |t: (&str, u8)| { (Some(t.0.into()), None, Some(t.1)) })

        // Radixless .123
      | map!(do_parse!(
            radixpoint >>
            fractional: decimals_ne >>
            (fractional)
        ), |n: &str| { (None, Some(n.into()), None) })

        // Radixless 12.3 or 12.
      | map!(do_parse!(
            integer: decimals_ne >>
            radixpoint >>
            fractional: decimals >>
            (integer, fractional)
        ), |t: (&str, &str)| { (Some(t.0.into()), Some(t.1.into()), None) })

        // Radixless 123
      | map!(decimals_ne, |n: &str| { (Some(n.into()), None, None) })
    ) >>
    eof!() >>
    (sign, numerated)
), |tup: (Sign, (Option<String>, Option<String>, Option<u8>))| {
    Number::parse(
        tup.0,
        (tup.1).0.unwrap_or("".into()),
        (tup.1).1.unwrap_or("".into()),
        (tup.1).2.unwrap_or(10)
    )
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
