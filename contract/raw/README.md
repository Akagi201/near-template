# raw wasm contract

```sh
cargo build --release -p rawcon --target wasm32-unknown-unknown
```

wast text format

```sh
brew install wabt
wasm2wat ./target/wasm32-unknown-unknown/release/rawcon.wasm -o rawcon.wast
```
