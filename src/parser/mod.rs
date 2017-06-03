use self::whitespace::{is_whitespace, not_whitespace};

mod boolean;
mod number;
mod whitespace;

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Number(number::Number),
    Boolean(bool)
}

named!(token(&str) -> Type, alt_complete!(
    map!(boolean::boolean, |b| { Type::Boolean(b) })
  | map!(number::number, |n| { Type::Number(n) })
));

named!(pub stalc(&str) -> Vec<Type>, many0!(
    do_parse!(
        token: flat_map!(take_while_s!(not_whitespace), token) >>
        take_while_s!(is_whitespace) >>
        (token)
    )
));
