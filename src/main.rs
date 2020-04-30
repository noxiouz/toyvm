pub mod instructions;
pub mod repl;
pub mod vm;

fn main() {
    let mut r = repl::REPL::new();
    r.run();
}
