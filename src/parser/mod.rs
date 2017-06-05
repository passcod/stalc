use self::whitespace::{is_whitespace, not_whitespace};

mod boolean;
mod datetime;
mod number;
mod numeral;
mod whitespace;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    ArgumentListStart,
    ArgumentListStop,
    Boolean(boolean::Boolean),
    Command(String),
    Datetime(datetime::Datetime),
    Number(number::Number),
}

fn flatten(vecovec: Vec<Vec<Token>>) -> Vec<Token> {
    vecovec
        .into_iter()
        .flat_map(|t| t.into_iter())
        .collect()
}

fn both(mut v1: Vec<Token>, mut v2: Vec<Token>) -> Vec<Token> {
    v1.append(&mut v2);
    v1
}

fn notargtok (c: char) -> bool {
    match c {
        '(' | ')' => false,
        c @ _ => not_whitespace(c)
    }
}

named!(aname(&str) -> &str, take_while1_s!(notargtok));

named!(token(&str) -> Vec<Token>, alt_complete!(
    // End of argument list
    map!(
        terminated!(tag_s!(")"), eof!()),
        |_| vec![Token::ArgumentListStop]
    )

  | map!(terminated!(boolean::boolean, eof!()), |b| { vec![Token::Boolean(b)] })
  | map!(terminated!(datetime::datetime, eof!()), |d| { vec![Token::Datetime(d)] })
  | map!(terminated!(number::number, eof!()), |n| { vec![Token::Number(n)] })

    // Command with opening argument list
  | map!(do_parse!(
        command: aname >>
        tag_s!("(") >>
        eof!() >>
        (command)
    ), |c: &str| { vec![
      Token::Command(c.into()),
      Token::ArgumentListStart
    ] })

    // Command with first argument
  | map!(do_parse!(
        command: aname >>
        tag_s!("(") >>
        any: flat_map!(aname, token) >>
        eof!() >>
        (command, any)
    ), |t: (&str, Vec<Token>)| both(vec![
        Token::Command(t.0.into()),
        Token::ArgumentListStart
    ], t.1))

    // Command with unary argument list
  | map!(do_parse!(
        command: aname >>
        any: delimited!(
            tag_s!("("),
            flat_map!(take_until_s!(")"), token),
            tag_s!(")")
        ) >>
        eof!() >>
        (command, any)
    ), |t: (&str, Vec<Token>)| flatten(vec![vec![
        Token::Command(t.0.into()),
        Token::ArgumentListStart,
    ], t.1, vec![
        Token::ArgumentListStop
    ]]))

    // Command with empty argument list
  | map!(do_parse!(
        command: aname >>
        tag_s!("()") >>
        eof!() >>
        (command)
    ), |c: &str| vec![
        Token::Command(c.into()),
        Token::ArgumentListStart,
        Token::ArgumentListStop
    ])

    // Bare command
  | map!(do_parse!(
        command: aname >>
        eof!() >>
        (command)
    ), |c: &str| vec![
        Token::Command(c.into())
    ])

    // Last argument with closing
  | map!(do_parse!(
        any: flat_map!(aname, token) >>
        tag_s!(")") >>
        eof!() >>
        (any)
    ), |t: Vec<Token>| both(t, vec![
        Token::ArgumentListStop
    ]))
));

named!(pub stalc(&str) -> Vec<Token>, map!(many0!(
    do_parse!(
        tokens: flat_map!(take_while_s!(not_whitespace), token) >>
        take_while_s!(is_whitespace) >>
        (tokens)
    )
), flatten));
