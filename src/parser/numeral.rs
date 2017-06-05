use nom::{ErrorKind, IResult};
use num::{BigInt, Num, One, PrimInt, Zero};
use num::rational::BigRational;

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

fn char_code(c: char) -> u8 {
    let mut buf = [0; 4];
    c.encode_utf8(&mut buf).as_bytes()[0]
}

pub fn big_power(n: &BigInt, exp: isize) -> BigRational {
    if exp == 0 {
        return BigRational::one();
    }

    let mut acc = BigInt::one();
    let posit = exp > 0;
    let exp = if posit { exp } else { -exp };

    for _ in 0..exp {
        acc = acc * n;
    }

    if posit {
        BigRational::new(acc, BigInt::one())
    } else {
        BigRational::new(BigInt::one(), acc)
    }
}

pub fn parse_str_radix(integer: String, fractional: String, radix: u8) -> Result<BigRational, String> {
    if 1 > radix && radix >= 62 {
        return Err(format!("Radix must be between 1 and 62 inclusive, found {}", radix));
    }

    let num = format!("{}{}", integer, fractional);
    let num_offset = fractional.len();

    if integer.is_empty() && fractional.is_empty() {
        return Ok(BigRational::zero());
    }

    if radix == 1 {
        let intg = BigRational::from_integer(BigInt::from(integer.len()));
        let dec_frac = BigInt::from(num_offset);

        if dec_frac == BigInt::zero() {
            return Ok(intg);
        }

        let frac = BigRational::new(
            dec_frac.clone(),
            BigInt::from(10.pow(format!("{}", dec_frac).len() as u32))
        );

        return Ok(intg + frac);
    }

    if fractional.is_empty() && 2 <= radix && radix <= 36 {
        return match BigInt::from_str_radix(integer.as_str(), radix as u32) {
            Ok(bu) => Ok(BigRational::from_integer(bu)),
            Err(e) => Err(format!("Could not parse {}.{} as radix {}: {}", integer, fractional, radix, e))
        };
    }

    let digits = num.chars().map(|d| {
        let dec = match d {
            '0'...'9' => char_code(d) - char_code('0'),
            'A'...'Z' => char_code(d) - char_code('A') + 10,
            'a'...'z' => if radix <= 36 {
                char_code(d) - char_code('a') + 10
            } else {
                char_code(d) - char_code('a') + 36
            },
            _ => unreachable!()
        };

        if dec >= radix {
            println!("Could not parse numeral {} as radix {}", d, radix);
            Err(format!("Could not parse numeral {} as radix {}", d, radix))
        } else {
            Ok(BigInt::from(dec))
        }
    }).collect::<Result<Vec<BigInt>, String>>()?;

    let mut acc = BigRational::zero();
    let mut i = 0;
    let rad = BigInt::from(radix);
    let total = (digits.len() - num_offset) as isize - 1;
    for n in digits.iter() {
        let dec = BigRational::from_integer(n.clone()) * big_power(&rad, total - i);

        println!("{} × {} ^ {} = {}",
            n, rad, total - i, dec
        );

        acc = &acc + dec;
        i += 1;
    }

    Ok(acc)
}

