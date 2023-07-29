# Gloam

## Setup

- Install [rust](https://rustup.rs/) with the nightly channel
- Install the `wasm32` target with `rustup target add wasm32-unknown-unknown`
- install the wasm-bindgen cli tool with `cargo install -f wasm-bindgen-cli`
- Install cargo post with `cargo install cargo-post`
- To build the web project, install [Node.js](https://nodejs.org/en/download) 

## Build

Run `cargo post build --target wasm32-unknown-unknown --release`.

A run configuration is provided for CLion.

## Web Project
- 
- Run `npm i`
- Run `npm run dev`

Rebuilding the rust will automatically reload the web project.