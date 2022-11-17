# Abstract App Module Template
This is a repository for developing an app using [Abstract-OS](https://abstract.money).
It includes a template for the app module, as well as deployment scripts.

## Features
- Abstracted handlers for `instantiate`, `query`, `execute`, `migrate`


## Instructions
### Option 1
Use the template using [`cargo generate`](https://cargo-generate.github.io/cargo-generate/index.html)
```shell
cargo generate gh:Abstract-OS/app-module-template -b template
```
### Option 2:
Fork this repository and use the `untemplated` branch.


## Commands
- `cargo build`: Build the app
- `cargo test`: Run the tests
- `cargo deploy`: Deploy the app to Abstract Version Control
- `cargo wasm`: Compile the app to wasm
- `cargo doc`: Generate the documentation
- `cargo clippy`: Run the linter
