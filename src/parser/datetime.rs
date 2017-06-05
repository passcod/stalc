use super::numeral::digit;

#[derive(Debug, PartialEq, Eq)]
pub struct Datetime {
    pub date: String,
    pub time: String,
    pub zone: String,
}

named!(year(&str) -> Vec<char>, count!(digit, 4));
named!(month(&str) -> Vec<char>, count!(digit, 2));
named!(day(&str) -> Vec<char>, count!(digit, 2));

named!(pub datetime(&str) -> Datetime, map!(do_parse!(
    year: year >>
    tag_s!("-") >>
    month: month >>
    tag_s!("-") >>
    day: day >>
    (year, month, day)
), |tup: (Vec<char>, Vec<char>, Vec<char>)| {
    let (year, month, day) = (
        tup.0.iter().collect::<String>(),
        tup.1.iter().collect::<String>(),
        tup.2.iter().collect::<String>()
    );

    Datetime {
        date: format!("{}-{}-{}", year, month, day),
        time: "".into(),
        zone: "".into()
    }
}));
