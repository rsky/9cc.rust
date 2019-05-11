fn main() {
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    println!("  mov rax, 1");
    println!("  ret");
}
