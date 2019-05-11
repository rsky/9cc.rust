use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut numbers = Vec::new();

    for arg in std::env::args().skip(1) {
        numbers.push(u8::from_str(&arg).expect("error parsing argument"))
    }

    if numbers.len() != 1 {
        writeln!(std::io::stderr(), "Invalid number of arguments.").unwrap();
        std::process::exit(1);
    }

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    println!("  mov rax, {}", numbers[0]);
    println!("  ret");
}
