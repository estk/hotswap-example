# Salutation

Steps to repro:

```sh
cargo component build -p types
cargo component build -p salutation

cd target/wasm32-wasip1/debug/

wac plug --plug types.wasm salutation.wasm -o tysal.wasm
```
