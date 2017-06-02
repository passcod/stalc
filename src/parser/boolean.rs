use std::str;

named!(pub boolean<bool>, map!(alt!(
    tag!("true")
  | tag!("false")
), |b| {
    match str::from_utf8(b).unwrap() {
        "true" => true,
        "false" => false,
        _ => unreachable!()
    }
}));

#[cfg(test)]
mod test {
    use nom::{ErrorKind, IResult, Needed};
    use super::boolean;

    #[test]
    fn true_() {
        assert_eq!(
            IResult::Done(&[][..], true),
            boolean("true".as_bytes())
        );
    }

    #[test]
    fn false_() {
        assert_eq!(
            IResult::Done(&[][..], false),
            boolean("false".as_bytes())
        );
    }

    #[test]
    fn invalid() {
        assert_eq!(
            IResult::Error(ErrorKind::Alt),
            boolean("invalid".as_bytes())
        );
    }

    #[test]
    fn empty() {
        assert_eq!(
            IResult::Incomplete(Needed::Size(4)),
            boolean("".as_bytes())
        );
    }
}
