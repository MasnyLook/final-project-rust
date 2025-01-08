# Rust WebAssembly Game

## Description

This project is a web-based game written in Rust and compiled to WebAssembly (Wasm). The game includes multiple modes of "Guess the Number".

### Description of the first part

The game I've implemented is built with HTML and styled with Bootstrap. Most of the code is implemented in Rust using the `wasm-bindgen` and `web-sys` tools.

In the first part of the project, the foundation for extending it in the second part is ready. We can launch the site on localhost, which includes a list of games that use the same template but have different versions of functions to interact with.

The game tracks various statistics, such as the number of attempts and the time taken to guess the number.

I've also added some unit tests.

I spent a lot of time working on this project learning how the `wasm-bindgen` library works. I considered many architectural approaches for the project, but finally decided to use JavaScript scripts that invoke compiled `#[wasm_bindgen]` functions from Rust to WebAssembly. I've attached the sources I used.

#### Sources (1 part)

- https://rustwasm.github.io/docs/book/
- https://github.com/BekBrace/rust-webass-tax
- https://wasmbyexample.dev/home.en-us.html
- https://github.com/rustwasm/wasm-bindgen/tree/main/examples
- https://rustwasm.github.io/wasm-bindgen/

### Description of the second part

The main goal for the second part was to implement a backend server to provide account creation functionality. I achieved this goal by completing the following consecutive subtasks:

#### 1) Add a database

I've considered many options for this goal. Finally, I've decided to use `tokio-postgres` (after failing to connect to aws db). I attach my inspiration for the database.

- https://youtu.be/FW4oUXHly8c?si=-5YoAXrpFHqbGOIE
- https://github.com/mimuw-jnp2-rust/project-bartek-sadlej/tree/main

#### 2) Create a web-app for the database

Again after searching through various sources, I've decided to use the `actix-web` library. Below is another inspiration:

- https://actix.rs/docs/getting-started
- https://youtu.be/L8tWKqSMKUI?si=I4_qMUKdvv_KMbYn
- https://github.com/MoonKraken/youtube/tree/main/ActixWebTaskService

#### 3) Connect frontend and backend app: creating accounts

The first difficulty was the cross-origin policy, which aborts the connection from the unknown sources. This was resolved by adding extra arguments to the actix-web app. 

Additionally, I couldn't use the method of sending requests in the `web-sys` library. After hours of failures I decided to change the approach and use the `reqwest` library.

The next challenge was how to store information about whether user is logged in or not. I decided to use localStorage to store authentication tokens.
(I also created the entire login system).

- https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
- https://stackoverflow.com/questions/72521659/how-to-make-post-request-with-json-body-using-web-sys-in-webassembly
- https://www.frontstack.pl/blog/czym-jest-local-storage-i-jak-uzywac
- https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Storage.html

#### 4) Table of results

One is the history of games for each user. The second one is a leaderboard on the main page.

#### 5) Websockets

I implemented a system to remembering all connected users in the backend app, such that when the leaderboard changes they get a notification (`actix` lib). Local app uses built websockets in wasm. 

- https://rustwasm.github.io/wasm-bindgen/examples/websockets.html

Additionally during refactoring the code I add some macros to generate html code.

## Installation

Setup the database before.
First set environmental variables, (check the tokio-db/src/credentials.rs file) and then create the database.

example setup:
```sh
   sudo -i -u postgres
   psql
   CREATE USER <POSTGRES_USER> WITH PASSWORD '<POSTGRES_PASSWORD>';
   CREATE DATABASE <POSTGRES_DB>;
   GRANT ALL PRIVILEGES ON DATABASE <POSTGRES_DB> TO <POSTGRES_USER>;
   \q
```

- backend app

```sh
   cd tokio-db
   cargo run
```

- frontend app

```sh
   npm install
   wasm-pack build
   npm run serve
```

then go to http://localhost:8080/main.html

