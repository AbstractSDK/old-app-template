# Abstract App Module Template
This is a repository for developing an app using [Abstract-OS](https://abstract.money).
It includes a template for the app module, deployment interfaces, and deployment scripts.

## Features
- Abstracted handlers for `instantiate`, `query`, `execute`, `migrate`, and `replies`
- Deployment scripts
- Asset value calculation using Abstract Name Service (ANS)


## Instructions

### Option 1
Use the template using [`cargo generate`](https://cargo-generate.github.io/cargo-generate/index.html)
```shell
cargo generate gh:Abstract-OS/app-template -b template
```
### Option 2:
Fork this repository and delete `template-scripts`, then rename the appropriate variables.


## Commands
- `cargo build`: Build the app
- `cargo test`: Run the tests
- `cargo deploy`: Deploy the app to Abstract Version Control
- `cargo wasm`: Compile the app to wasm
- `cargo doc`: Generate the documentation
- `cargo clippy`: Run the linter

## References
- [Getting Started with Abstract — How to create a Modular App | by Abstract | Nov, 2022 | Medium](https://medium.com/@abstractos/getting-started-with-abstract-how-to-create-your-own-app-module-5ddd644534f3)
