//#![feature(proc_macro_hygiene)]

#[macro_use]
mod print;

fn main() {
    println!("Hello wasm64!");
}

// Program entry point by wasmtime
#[no_mangle]
pub fn _start() {
    main();
}
