mod codegen;
mod node;
mod token;

use std::{
    env::args,
    io::{stdout, Write},
};

use anyhow::{anyhow, Error as AnyhowError};

use crate::{codegen::generate, node::parse, token::tokenize};

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
