use std::io::{Error as IoError, Write};

use crate::node::Node;

pub fn generate(node: &Node) -> Result<Vec<u8>, IoError> {
    let mut buf = Vec::new();
    writeln!(buf, ".intel_syntax noprefix")?;
    writeln!(buf, ".globl main")?;
    writeln!(buf, "main:")?;

    write_node(&mut buf, node)?;
    writeln!(buf, "  pop rax")?;
    writeln!(buf, "  ret")?;

    Ok(buf)
}

fn write_node(buf: &mut Vec<u8>, node: &Node) -> Result<(), IoError> {
    match node {
        Node::Number(n) => {
            writeln!(buf, "  push {}", n)?;
            return Ok(());
        }
        Node::Addition { lhs, rhs }
        | Node::Subtraction { lhs, rhs }
        | Node::Multiplication { lhs, rhs }
        | Node::Division { lhs, rhs }
        | Node::Equal { lhs, rhs }
        | Node::NotEqual { lhs, rhs }
        | Node::GraterThen { lhs, rhs }
        | Node::GraterEqual { lhs, rhs }
        | Node::LowerThen { lhs, rhs }
        | Node::LowerEqual { lhs, rhs } => {
            write_node(buf, lhs)?;
            write_node(buf, rhs)?;

            writeln!(buf, "  pop rdi")?;
            writeln!(buf, "  pop rax")?;

            match node {
                Node::Number(_) => unreachable!(),
                Node::Addition { lhs: _, rhs: _ } => {
                    writeln!(buf, "  add rax, rdi")?;
                }
                Node::Subtraction { lhs: _, rhs: _ } => {
                    writeln!(buf, "  sub rax, rdi")?;
                }
                Node::Multiplication { lhs: _, rhs: _ } => {
                    writeln!(buf, "  imul rax, rdi")?;
                }
                Node::Division { lhs: _, rhs: _ } => {
                    writeln!(buf, "  cqo")?;
                    writeln!(buf, "  idiv rdi")?;
                }
                Node::Equal { lhs: _, rhs: _ } => {
                    writeln!(buf, "  cmp rax, rdi")?;
                    writeln!(buf, "  sete al")?;
                    writeln!(buf, "  movzb rax, al")?;
                }
                Node::NotEqual { lhs: _, rhs: _ } => {
                    writeln!(buf, "  cmp rax, rdi")?;
                    writeln!(buf, "  setne al")?;
                    writeln!(buf, "  movzb rax, al")?;
                }
                Node::GraterThen { lhs: _, rhs: _ } => {
                    writeln!(buf, "  cmp rax, rdi")?;
                    writeln!(buf, "  setg al")?;
                    writeln!(buf, "  movzb rax, al")?;
                }
                Node::GraterEqual { lhs: _, rhs: _ } => {
                    writeln!(buf, "  cmp rax, rdi")?;
                    writeln!(buf, "  setge al")?;
                    writeln!(buf, "  movzb rax, al")?;
                }
                Node::LowerThen { lhs: _, rhs: _ } => {
                    writeln!(buf, "  cmp rax, rdi")?;
                    writeln!(buf, "  setl al")?;
                    writeln!(buf, "  movzb rax, al")?;
                }
                Node::LowerEqual { lhs: _, rhs: _ } => {
                    writeln!(buf, "  cmp rax, rdi")?;
                    writeln!(buf, "  setle al")?;
                    writeln!(buf, "  movzb rax, al")?;
                }
            }
        }
    }

    writeln!(buf, "  push rax")?;

    Ok(())
}
