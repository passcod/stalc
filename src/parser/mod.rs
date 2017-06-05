use self::whitespace::{is_whitespace, not_whitespace};

mod boolean;
mod datetime;
mod number;
mod numeral;
mod whitespace;

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Boolean(boolean::Boolean),
    Datetime(datetime::Datetime),
    Number(number::Number),
}

named!(token(&str) -> Type, alt_complete!(
    map!(boolean::boolean, |b| { Type::Boolean(b) })
  | map!(datetime::datetime, |d| { Type::Datetime(d) })
  | map!(number::number, |n| { Type::Number(n) })
));

named!(pub stalc(&str) -> Vec<Type>, many0!(
    do_parse!(
        token: flat_map!(take_while_s!(not_whitespace), token) >>
        take_while_s!(is_whitespace) >>
        (token)
    )
));
