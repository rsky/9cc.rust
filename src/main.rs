use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        writeln!(std::io::stderr(), "Invalid number of arguments.").unwrap();
        std::process::exit(1);
    }

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    println!("  mov rax, {}", args[1]);
    println!("  ret");
}
