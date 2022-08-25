use std::env::args;

use anyhow::{anyhow, Error as AnyhowError};

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

    let mut input = args[1].as_str();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    let (n, cnt) = read_number(input)?;
    input = &input[cnt..];
    println!("  mov rax, {n}");

    while let Some(s) = input.get(..1) {
        match s {
            "+" => {
                input = &input[1..];

                let (n, cnt) = read_number(input)?;
                input = &input[cnt..];
                println!("  add rax, {n}");
            }
            "-" => {
                input = &input[1..];

                let (n, cnt) = read_number(input)?;
                input = &input[cnt..];
                println!("  sub rax, {n}");
            }
            _ => return Err(anyhow!("unexpected token: `{s}`")),
        }
    }

    println!("  ret");

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
