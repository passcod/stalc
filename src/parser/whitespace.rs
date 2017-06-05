pub static WHITESPACE: [char; 25] = [
    '\u{09}',
    '\u{0a}',
    '\u{0b}',
    '\u{0c}',
    '\u{0d}',
    '\u{20}',
    '\u{85}',
    '\u{a0}',
    '\u{1680}',
    '\u{2000}',
    '\u{2001}',
    '\u{2002}',
    '\u{2003}',
    '\u{2004}',
    '\u{2005}',
    '\u{2006}',
    '\u{2007}',
    '\u{2008}',
    '\u{2009}',
    '\u{200a}',
    '\u{2028}',
    '\u{2029}',
    '\u{202f}',
    '\u{205f}',
    '\u{3000}'
];

pub fn is_whitespace(c: char) -> bool {
    WHITESPACE.contains(&c)
}

pub fn not_whitespace(c: char) -> bool {
    ! is_whitespace(c)
}
