use std::str;

use anyhow::{anyhow, Error as AnyhowError};

#[derive(Debug)]
pub enum Token {
    /// `+`
    Addition,
    /// `-`
    Subtraction,
    /// `*`
    Multiplication,
    /// `/`
    Division,
    /// number
    Number(u64),
    /// `(`
    ParenthesisBegin,
    /// `)`
    ParenthesisEnd,
    /// `==`
    Equal,
    /// `!=`
    NotEqual,
    /// `>`
    GraterThen,
    /// `>=`
    GraterEqual,
    /// `<`
    LowerThen,
    /// `<=`
    LowerEqual,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, AnyhowError> {
    let mut tokens = Vec::<Token>::new();
    let mut p = input;

    loop {
        if let Some(s) = p.get(0..2) {
            match s {
                "==" => {
                    p = &p[2..];
                    tokens.push(Token::Equal);
                    continue;
                }
                "!=" => {
                    p = &p[2..];
                    tokens.push(Token::NotEqual);
                    continue;
                }
                ">=" => {
                    p = &p[2..];
                    tokens.push(Token::GraterEqual);
                    continue;
                }
                "<=" => {
                    p = &p[2..];
                    tokens.push(Token::LowerEqual);
                    continue;
                }
                _ => (),
            }
        }

        match p.get(..1) {
            Some(s) => match s {
                " " | "\n" => {
                    p = &p[1..];
                }
                "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                    let (n, cnt) = read_number(p)?;
                    p = &p[cnt..];
                    tokens.push(Token::Number(n));
                }
                "+" => {
                    p = &p[1..];
                    tokens.push(Token::Addition);
                }
                "-" => {
                    p = &p[1..];
                    tokens.push(Token::Subtraction);
                }
                "*" => {
                    p = &p[1..];
                    tokens.push(Token::Multiplication);
                }
                "/" => {
                    p = &p[1..];
                    tokens.push(Token::Division);
                }
                "(" => {
                    p = &p[1..];
                    tokens.push(Token::ParenthesisBegin);
                }
                ")" => {
                    p = &p[1..];
                    tokens.push(Token::ParenthesisEnd);
                }
                ">" => {
                    p = &p[1..];
                    tokens.push(Token::GraterThen);
                }
                "<" => {
                    p = &p[1..];
                    tokens.push(Token::LowerThen);
                }
                _ => {
                    let ws = " ".repeat(p.as_ptr() as usize - input.as_ptr() as usize + 7);
                    return Err(anyhow!("{input}\n{ws}^ invalid token"));
                }
            },
            None => break,
        }
    }

    Ok(tokens)
}

fn read_number(s: &str) -> Result<(u64, usize), AnyhowError> {
    let mut n = 0;
    let mut cnt = 0;

    for c in s.chars() {
        match c.to_digit(10) {
            Some(u) => {
                n = n * 10 + u as u64;
                cnt += 1;
            }
            None => break,
        }
    }

    if cnt == 0 {
        return Err(anyhow!("invalid number: `{s}`"));
    }

    Ok((n, cnt))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_return_u64() {
        assert_eq!(read_number("1").unwrap(), (1, 1));
        assert_eq!(read_number("12").unwrap(), (12, 2));
        assert_eq!(read_number("123").unwrap(), (123, 3));
    }
}
