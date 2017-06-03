use std::str;

named!(pub boolean(&str) -> bool, map!(alt_complete!(
    tag_s!("true")
  | tag_s!("false")
), |b| {
    match b {
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
            IResult::Done("", true),
            boolean("true")
        );
    }

    #[test]
    fn false_() {
        assert_eq!(
            IResult::Done("", false),
            boolean("false")
        );
    }

    #[test]
    fn invalid() {
        assert_eq!(
            IResult::Error(ErrorKind::Alt),
            boolean("invalid")
        );
    }

    #[test]
    fn empty() {
        assert_eq!(
            IResult::Incomplete(Needed::Size(5)),
            boolean("")
        );
    }
}
