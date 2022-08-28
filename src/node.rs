use std::{iter::Peekable, rc::Rc};

use anyhow::{anyhow, Error as AnyhowError};

use crate::token::Token;

#[derive(Debug)]
pub enum Node {
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

pub fn parse(tokens: &[Token]) -> Result<Node, AnyhowError> {
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
