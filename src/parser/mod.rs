use self::whitespace::space;

mod boolean;
mod number;
mod whitespace;

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Number(number::Number),
    Boolean(bool)
}

macro_rules! uws (
  ($i:expr, $($args:tt)*) => (
    {
      sep!($i, space, $($args)*)
    }
  )
);

named!(pub stalc<&[u8], Vec<Type> >,
    many0!(
        uws!(alt!(
            map!(boolean::boolean, |b| { Type::Boolean(b) })
          | map!(number::number, |n| { Type::Number(n) })
        ))
    )
);
