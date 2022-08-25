use std::{env::args, process::exit};

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("invalid number of arguments");
        exit(1);
    }

    let input = args[1].as_str();
    let exit_code = input.parse::<i32>().unwrap_or_default();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {exit_code}");
    println!("  ret");
}
