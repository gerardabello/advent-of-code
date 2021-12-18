use nom::{branch::alt, bytes::complete::tag, combinator::map, multi::many1, IResult};
use std::fmt;

use crate::parsers::{full, lines, unsigned_int};

pub type Number = Vec<Token>;

#[derive(Clone)]
pub enum Token {
    Open,
    Close,
    Comma,
    Value(usize),
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => write!(f, "["),
            Self::Close => write!(f, "]"),
            Self::Comma => write!(f, ","),
            Self::Value(v) => write!(f, "{}", v),
        }
    }
}

pub fn parse_line(input: &str) -> IResult<&str, Number> {
    many1(alt((
        map(tag("["), |_| Token::Open),
        map(tag("]"), |_| Token::Close),
        map(tag(","), |_| Token::Comma),
        map(unsigned_int::<usize>, Token::Value),
    )))(input)
}

pub fn parse_input(input: &str) -> Vec<Number> {
    full(lines(parse_line))(input).unwrap().1
}

pub fn sum(n1: &[Token], n2: &[Token]) -> Number {
    let mut res = vec![Token::Open];
    res.append(&mut n1.to_owned());
    res.push(Token::Comma);
    res.append(&mut n2.to_owned());
    res.push(Token::Close);

    res
}

pub fn sum_two_tokens(t1: &Token, t2: &Token) -> Option<Token> {
    match t1 {
        Token::Value(v) => match t2 {
            Token::Value(vn1) => Some(Token::Value(v + vn1)),
            _ => None,
        },
        _ => None,
    }
}

pub fn explode(number: &mut Number) -> bool {
    let mut depth = 0;
    for i in 0..number.len() {
        let token = &number[i];
        match token {
            Token::Open => depth += 1,
            Token::Close => depth -= 1,
            _ => {}
        }

        if depth > 4 {
            let removed = [
                number.remove(i), // open
                number.remove(i), // number
                number.remove(i), //comma
                number.remove(i), //number
                number.remove(i), // close
            ];

            number.insert(i, Token::Value(0));

            assert!(matches!(removed[1], Token::Value(_)));
            assert!(matches!(removed[3], Token::Value(_)));

            for li in (0..i).rev() {
                if let Some(t) = sum_two_tokens(&number[li], &removed[1]) {
                    number[li] = t;
                    break;
                }
            }

            for ri in i + 1..number.len() {
                if let Some(t) = sum_two_tokens(&number[ri], &removed[3]) {
                    number[ri] = t;
                    break;
                }
            }

            return true;
        }
    }

    false
}

pub fn div_up(a: usize, b: usize) -> usize {
    // We *know* that the hint is exact, this is thus precisely the amount of chunks of length `b` each
    (0..a).step_by(b).size_hint().0
}

pub fn div_down(a: usize, b: usize) -> usize {
    a / b
}

pub fn split(number: &mut Number) -> bool {
    for i in 0..number.len() {
        let token = &number[i].clone();
        if let Token::Value(v) = token {
            if *v >= 10 {
                number.remove(i);
                number.insert(i, Token::Close);
                number.insert(i, Token::Value(div_up(*v, 2)));
                number.insert(i, Token::Comma);
                number.insert(i, Token::Value(div_down(*v, 2)));
                number.insert(i, Token::Open);

                return true;
            }
        }
    }

    false
}

pub fn reduce(number: &mut Number) -> bool {
    if explode(number) || split(number) {
        reduce(number)
    } else {
        false
    }
}

pub fn slice_single_number(number: &[Token]) -> &[Token] {
    match number[0] {
        Token::Open => slice_until_matching_closing(number),
        Token::Value(_) => &number[0..1],
        _ => unreachable!(),
    }
}

pub fn slice_until_matching_closing(number: &[Token]) -> &[Token] {
    assert!(matches!(number[0], Token::Open));
    let mut depth = 1;
    for i in 1..number.len() {
        let token = &number[i];
        match token {
            Token::Open => depth += 1,
            Token::Close => depth -= 1,
            _ => {}
        }

        if depth == 0 {
            return &number[..i + 1];
        }
    }

    unreachable!();
}

pub fn magnitude(number: &[Token]) -> usize {
    match number[0] {
        Token::Value(v) => v,
        Token::Open => {
            let first = slice_single_number(&number[1..]);
            let second = slice_single_number(&number[first.len() + 2..number.len() - 1]);

            3 * magnitude(first) + 2 * magnitude(second)
        }
        _ => unreachable!(),
    }
}

pub fn solve(input: &str) -> usize {
    let numbers = parse_input(input);

    let mut base: Number = numbers[0].clone();

    for number in numbers.iter().skip(1) {
        base = sum(&base, number);
        reduce(&mut base);
    }

    magnitude(&base)
}
