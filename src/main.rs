pub mod assembler;
pub mod instructions;
pub mod repl;
pub mod vm;

extern crate nom;

fn main() {
    let mut r = repl::REPL::new();
    r.run();
}
