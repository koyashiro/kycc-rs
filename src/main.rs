use std::{
    env::args,
    io::{stdout, Error as IoError, Write},
    iter::Peekable,
    rc::Rc,
    str,
};

use anyhow::{anyhow, Error as AnyhowError};

#[derive(Debug)]
enum Token {
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

fn tokenize(input: &str) -> Result<Vec<Token>, AnyhowError> {
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

#[derive(Debug)]
enum Node {
    Addition {
        lhs: Rc<Node>,
        rhs: Rc<Node>,
    },
    Subtraction {
        lhs: Rc<Node>,
        rhs: Rc<Node>,
    },
    Multiplication {
        lhs: Rc<Node>,
        rhs: Rc<Node>,
    },
    Division {
        lhs: Rc<Node>,
        rhs: Rc<Node>,
    },
    Number(u64),
    Equal {
        lhs: Rc<Node>,
        rhs: Rc<Node>,
    },
    /// `!=`
    NotEqual {
        lhs: Rc<Node>,
        rhs: Rc<Node>,
    },
    /// `>`
    GraterThen {
        lhs: Rc<Node>,
        rhs: Rc<Node>,
    },
    /// `>=`
    GraterEqual {
        lhs: Rc<Node>,
        rhs: Rc<Node>,
    },
    /// `<`
    LowerThen {
        lhs: Rc<Node>,
        rhs: Rc<Node>,
    },
    /// `<=`
    LowerEqual {
        lhs: Rc<Node>,
        rhs: Rc<Node>,
    },
}

fn parse(tokens: &[Token]) -> Result<Node, AnyhowError> {
    let mut iter = tokens.iter().peekable();
    let node = expr(&mut iter)?;

    Ok(node)
}

fn expr<'a, I: Iterator<Item = &'a Token>>(iter: &mut Peekable<I>) -> Result<Node, AnyhowError> {
    equality(iter)
}

fn equality<'a, I: Iterator<Item = &'a Token>>(
    iter: &mut Peekable<I>,
) -> Result<Node, AnyhowError> {
    let mut node = relational(iter)?;

    loop {
        match iter.peek() {
            Some(Token::Equal) => {
                iter.next();
                node = Node::Equal {
                    lhs: Rc::new(node),
                    rhs: Rc::new(relational(iter)?),
                };
            }
            Some(Token::NotEqual) => {
                iter.next();
                node = Node::NotEqual {
                    lhs: Rc::new(node),
                    rhs: Rc::new(relational(iter)?),
                };
            }
            _ => return Ok(node),
        }
    }
}

fn relational<'a, I: Iterator<Item = &'a Token>>(
    iter: &mut Peekable<I>,
) -> Result<Node, AnyhowError> {
    let mut node = add(iter)?;

    loop {
        match iter.peek() {
            Some(Token::LowerThen) => {
                iter.next();
                node = Node::LowerThen {
                    lhs: Rc::new(node),
                    rhs: Rc::new(add(iter)?),
                };
            }
            Some(Token::LowerEqual) => {
                iter.next();
                node = Node::LowerEqual {
                    lhs: Rc::new(node),
                    rhs: Rc::new(add(iter)?),
                };
            }
            Some(Token::GraterThen) => {
                iter.next();
                node = Node::GraterThen {
                    lhs: Rc::new(node),
                    rhs: Rc::new(add(iter)?),
                };
            }
            Some(Token::GraterEqual) => {
                iter.next();
                node = Node::GraterEqual {
                    lhs: Rc::new(node),
                    rhs: Rc::new(add(iter)?),
                };
            }
            _ => return Ok(node),
        }
    }
}

fn add<'a, I: Iterator<Item = &'a Token>>(iter: &mut Peekable<I>) -> Result<Node, AnyhowError> {
    let mut node = mul(iter)?;

    loop {
        match iter.peek() {
            Some(Token::Addition) => {
                iter.next();
                node = Node::Addition {
                    lhs: Rc::new(node),
                    rhs: Rc::new(mul(iter)?),
                };
            }
            Some(Token::Subtraction) => {
                iter.next();
                node = Node::Subtraction {
                    lhs: Rc::new(node),
                    rhs: Rc::new(mul(iter)?),
                };
            }
            _ => return Ok(node),
        }
    }
}

fn mul<'a, I: Iterator<Item = &'a Token>>(iter: &mut Peekable<I>) -> Result<Node, AnyhowError> {
    let mut node = unary(iter)?;

    loop {
        match iter.peek() {
            Some(Token::Multiplication) => {
                iter.next();
                node = Node::Multiplication {
                    lhs: Rc::new(node),
                    rhs: Rc::new(unary(iter)?),
                };
            }
            Some(Token::Division) => {
                iter.next();
                node = Node::Division {
                    lhs: Rc::new(node),
                    rhs: Rc::new(unary(iter)?),
                };
            }
            _ => return Ok(node),
        }
    }
}

fn unary<'a, I: Iterator<Item = &'a Token>>(iter: &mut Peekable<I>) -> Result<Node, AnyhowError> {
    match iter.peek() {
        Some(Token::Addition) => {
            iter.next();
            unary(iter)
        }
        Some(Token::Subtraction) => {
            iter.next();
            Ok(Node::Subtraction {
                lhs: Rc::new(Node::Number(0)),
                rhs: Rc::new(unary(iter)?),
            })
        }
        _ => primary(iter),
    }
}

fn primary<'a, I: Iterator<Item = &'a Token>>(iter: &mut Peekable<I>) -> Result<Node, AnyhowError> {
    match iter.next() {
        Some(Token::ParenthesisBegin) => {
            let node = expr(iter)?;
            match iter.next() {
                Some(Token::ParenthesisEnd) => Ok(node),
                _ => Err(anyhow!("expect `}}`")),
            }
        }
        Some(Token::Number(n)) => Ok(Node::Number(*n)),
        _ => Err(anyhow!("expect number or `}}`")),
    }
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

fn write_node(buf: &mut Vec<u8>, node: &Node) -> Result<(), IoError> {
    match node {
        Node::Number(n) => {
            writeln!(buf, "  push {}", n)?;
            return Ok(());
        }
        Node::Addition { lhs, rhs } => {
            write_node(buf.by_ref(), lhs)?;
            write_node(buf.by_ref(), rhs)?;
            writeln!(buf, "  pop rdi")?;
            writeln!(buf, "  pop rax")?;
            writeln!(buf, "  add rax, rdi")?;
        }
        Node::Subtraction { lhs, rhs } => {
            write_node(buf, lhs)?;
            write_node(buf, rhs)?;
            writeln!(buf, "  pop rdi")?;
            writeln!(buf, "  pop rax")?;
            writeln!(buf, "  sub rax, rdi")?;
        }
        Node::Multiplication { lhs, rhs } => {
            write_node(buf, lhs)?;
            write_node(buf, rhs)?;
            writeln!(buf, "  pop rdi")?;
            writeln!(buf, "  pop rax")?;
            writeln!(buf, "  imul rax, rdi")?;
        }
        Node::Division { lhs, rhs } => {
            write_node(buf, lhs)?;
            write_node(buf, rhs)?;
            writeln!(buf, "  pop rdi")?;
            writeln!(buf, "  pop rax")?;
            writeln!(buf, "  cqo")?;
            writeln!(buf, "  idiv rdi")?;
        }
        Node::Equal { lhs, rhs } => {
            write_node(buf, lhs)?;
            write_node(buf, rhs)?;
            writeln!(buf, "  pop rdi")?;
            writeln!(buf, "  pop rax")?;
            writeln!(buf, "  cmp rax, rdi")?;
            writeln!(buf, "  sete al")?;
            writeln!(buf, "  movzb rax, al")?;
        }
        Node::NotEqual { lhs, rhs } => {
            write_node(buf, lhs)?;
            write_node(buf, rhs)?;
            writeln!(buf, "  pop rdi")?;
            writeln!(buf, "  pop rax")?;
            writeln!(buf, "  cmp rax, rdi")?;
            writeln!(buf, "  setne al")?;
            writeln!(buf, "  movzb rax, al")?;
        }
        Node::GraterThen { lhs, rhs } => {
            write_node(buf, lhs)?;
            write_node(buf, rhs)?;
            writeln!(buf, "  pop rdi")?;
            writeln!(buf, "  pop rax")?;
            writeln!(buf, "  cmp rax, rdi")?;
            writeln!(buf, "  setg al")?;
            writeln!(buf, "  movzb rax, al")?;
        }
        Node::GraterEqual { lhs, rhs } => {
            write_node(buf, lhs)?;
            write_node(buf, rhs)?;
            writeln!(buf, "  pop rdi")?;
            writeln!(buf, "  pop rax")?;
            writeln!(buf, "  cmp rax, rdi")?;
            writeln!(buf, "  setge al")?;
            writeln!(buf, "  movzb rax, al")?;
        }
        Node::LowerThen { lhs, rhs } => {
            write_node(buf, lhs)?;
            write_node(buf, rhs)?;
            writeln!(buf, "  pop rdi")?;
            writeln!(buf, "  pop rax")?;
            writeln!(buf, "  cmp rax, rdi")?;
            writeln!(buf, "  setl al")?;
            writeln!(buf, "  movzb rax, al")?;
        }
        Node::LowerEqual { lhs, rhs } => {
            write_node(buf, lhs)?;
            write_node(buf, rhs)?;
            writeln!(buf, "  pop rdi")?;
            writeln!(buf, "  pop rax")?;
            writeln!(buf, "  cmp rax, rdi")?;
            writeln!(buf, "  setle al")?;
            writeln!(buf, "  movzb rax, al")?;
        }
    }
    writeln!(buf, "  push rax")?;
    Ok(())
}

fn generate(node: &Node) -> Result<Vec<u8>, IoError> {
    let mut buf = Vec::new();
    writeln!(buf, ".intel_syntax noprefix")?;
    writeln!(buf, ".globl main")?;
    writeln!(buf, "main:")?;

    write_node(&mut buf, node)?;
    writeln!(buf, "  pop rax")?;
    writeln!(buf, "  ret")?;
    Ok(buf)
}

fn main() -> Result<(), AnyhowError> {
    let args = args().collect::<Vec<String>>();
    if args.len() != 2 {
        return Err(anyhow!("invalid number of arguments"));
    }

    let input = args[1].as_str();
    let tokens = tokenize(input)?;
    let node = parse(&tokens)?;
    let buf = generate(&node)?;
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
