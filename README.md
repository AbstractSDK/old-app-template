# {{ project-name }}
This is a repository for developing an add-on using [Abstract-OS](https://abstract.money).

## Features
- Abstracted handlers for `instantiate`, `query`, `execute`, `migrate`


## Instructions
### Option 1
Use the template using [`cargo generate`](https://cargo-generate.github.io/cargo-generate/index.html)
```shell
cargo generate gh:Abstract-OS/addon-module-template
```
### Option 2:
Fork this repository and use the `untemplated` branch


## Commands
- `cargo build`: Build the add-on
- `cargo test`: Run the tests
- `cargo wasm`: Compile the add-on to wasm
- `cargo doc`: Generate the documentation
- `cargo clippy`: Run the linter

# Tests
The test cases covered by this dapp are located in [the README file under src/tests/](src/tests/README.md).
