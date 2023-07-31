# near-template

[![Rust CI](https://github.com/Akagi201/near-template/actions/workflows/ci.yml/badge.svg)](https://github.com/Akagi201/near-template/actions/workflows/ci.yml) [![Super Linter](https://github.com/Akagi201/near-template/actions/workflows/super_linter.yml/badge.svg)](https://github.com/Akagi201/near-template/actions/workflows/super_linter.yml)

NEAR Rust contract template code

## Features

- [x] Use make to build and test.
- [x] Support raen to build the contract.

## Near Tools

- [raen](https://github.com/raendev/raen)
- [wasm-opt](http://llever.com/rustwasm-book/reference/code-size.zh.html)
- [rustle](https://github.com/blocksecteam/rustle)

## Build

Install build tool

```sh
cargo install raen
npm install --global raen-cli # alternative to npm
```

```sh
raen build --workspace --release
```

## Near docs

- <https://github.com/hanakannzashi/near-course-base/>

## Refs

- <https://github.com/NEARFoundation/near-smart-contract-rust-template/>
