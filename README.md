# Simple Wasm64 Rust Program

## Prerequisites

1. Rust nightly build: Version >= 1.80.0

    ```
    rustup override set nightly
    ```

2. Rust sources

    E.g. for MacOS x64:
    ```
    rustup component add rust-src --toolchain nightly-x86_64-apple-darwin
    ```

3. wasmtime: Version >= 20.0.2

## Compilation

```
cargo build --target=wasm64-unknown-unknown -Zbuild-std=std,panic_abort
```

## Execution

```
wasmtime -W memory64 target/wasm64-unknown-unknown/debug/rust64.wasm
```
