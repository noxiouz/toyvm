pub mod instructions;
pub mod vm;

fn main() {
    let mut vm = vm::VM::new();
    if let Err(err) = vm.run() {
        println!("VM finished with err {}", err);
    }
}
