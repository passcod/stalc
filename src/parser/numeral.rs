use nom::{ErrorKind, IResult};

pub static UNARY: char = '1';

pub static DECIMAL: &'static str = "0123456789";
pub static LOWERCASE: &'static str = "abcdefghijklmnopqrstuvwxyz";
pub static UPPERCASE: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

lazy_static! {
    pub static ref DIGITS: String = format!("{}{}{}", DECIMAL, UPPERCASE, LOWERCASE);
}

named!(pub decimal(&str) -> char, one_of!(DECIMAL));
named!(pub decimals(&str) -> &str, is_a_s!(DECIMAL));
pub fn decimals_ne(d: &str) -> IResult<&str, &str> {
    if d.len() == 0 {
        IResult::Error(ErrorKind::Eof)
    } else {
        decimals(d)
    }
}

named!(pub digit(&str) -> char, one_of!(DIGITS.as_str()));
named!(pub digits(&str) -> &str, is_a_s!(DIGITS.as_str()));
pub fn digits_ne(d: &str) -> IResult<&str, &str> {
    if d.len() == 0 {
        IResult::Error(ErrorKind::Eof)
    } else {
        digits(d)
    }
}
