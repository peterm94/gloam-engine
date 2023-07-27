# Gloam

## Setup

- Install [rust](https://rustup.rs/)
- Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
  - This should install the wasm32 target for you, but if it doesn't you can do it yourself with `rustup target add wasm32-unknown-unknown`
- Install [Node.js](https://nodejs.org/en/download) 

## Build
Run `cargo post build --target wasm32-unknown-unknown --release`.

A run configuration is provided for CLion.

## Web Project
- 
- Run `npm i`
- Run `npm run dev`

Rebuilding the rust will automatically reload the web project.