# Rust WebAssembly Game

## Description

This project is a web-based game written in Rust and compiled to WebAssembly (Wasm). The game includes multiple modes of "Guess the Number".

### Description of the first part

The game I've implemented is built with HTML and styled with Bootstrap. Most of the code is implemented in Rust using the `wasm-bindgen` and `web-sys` tools.

In the first part of the project, the foundation for extending it in the second part is ready. We can launch the site on localhost, which includes a list of games that use the same template but have different versions of functions to interact with.

The game tracks various statistics, such as the number of attempts and the time taken to guess the number.

I've also added some unit tests.

I spent a lot of time working on this project learning how the `wasm-bindgen` library works. I considered many architectural approaches for the project, but finally decided to use JavaScript scripts that invoke compiled `#[wasm_bindgen]` functions from Rust to WebAssembly. I've attached the sources I used.

## Installation
   ```sh
   npm install
   wasm-pack build
   npm run serve
```

then go to http://localhost:8080/main.html

## Sources

- https://rustwasm.github.io/docs/book/
- https://github.com/BekBrace/rust-webass-tax
- https://wasmbyexample.dev/home.en-us.html
- https://github.com/rustwasm/wasm-bindgen/tree/main/examples
- https://rustwasm.github.io/wasm-bindgen/


