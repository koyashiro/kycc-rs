use std::{
    env::args,
    io::{stdout, Write},
    str,
};

use anyhow::{anyhow, Error as AnyhowError};

#[derive(Debug)]
enum Token {
    Plus,
    Minus,
    Number(u64),
}

fn tokenize(input: &str) -> Result<Vec<Token>, AnyhowError> {
    let mut tokens = Vec::<Token>::new();
    let mut p = input;

    while let Some(s) = p.get(..1) {
        match s {
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
                tokens.push(Token::Plus);
            }
            "-" => {
                p = &p[1..];
                tokens.push(Token::Minus);
            }
            _ => return Err(anyhow!("unexpected token")),
        }
    }
    Ok(tokens)
}

fn generate(tokens: &[Token]) -> Result<Vec<u8>, AnyhowError> {
    let mut buf = Vec::new();

    let mut iter = tokens.iter().peekable();

    writeln!(buf, ".intel_syntax noprefix")?;
    writeln!(buf, ".globl main")?;
    writeln!(buf, "main:")?;

    let unexpected_token = anyhow!("unexpected token");

    match iter.next() {
        Some(Token::Number(n)) => {
            writeln!(buf, "  mov rax, {}", n)?;
        }
        _ => return Err(unexpected_token),
    }

    while let Some(t) = iter.next() {
        match t {
            Token::Plus => write!(buf, "  add rax, ")?,
            Token::Minus => write!(buf, "  sub rax, ")?,
            _ => return Err(unexpected_token),
        }

        match iter.next() {
            Some(Token::Number(n)) => writeln!(buf, "{n}")?,
            _ => return Err(unexpected_token),
        };
    }

    writeln!(buf, "  ret")?;

    Ok(buf)
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

fn main() -> Result<(), AnyhowError> {
    let args = args().collect::<Vec<String>>();
    if args.len() != 2 {
        return Err(anyhow!("invalid number of arguments"));
    }

    let input = args[1].as_str();
    let tokens = tokenize(input)?;
    let buf = generate(&tokens)?;
    stdout().write_all(&buf)?;

    Ok(())
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
