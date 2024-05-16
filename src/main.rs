#[macro_use]
mod print;

fn main() {
    println!("Hello wasm64!");
}

// Program entry point by wasmtime
#[no_mangle]
#[allow(clippy::main_recursion)]
pub fn _start() {
    main();
}
