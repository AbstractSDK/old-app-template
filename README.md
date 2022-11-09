# {{ project-name }}
This is a repository for developing an add-on using [Abstract-OS](https://abstract.money).

## Features
- Abstracted handlers for `instantiate`, `query`, `execute`, `migrate`


## File Structure
- [`src`](src) - source code
  - [`contract.rs`](src/contract.rs) - contract implementation with the top-level handlers for `instantiate`, `query`, `execute`, `migrate`
  - [`handlers`](src/handlers) - contains the handlers for the add-on
    - [`instantiate.rs`](src/handlers/instantiate.rs) - contains the msg handlers for the `instantiate` entrypoint
    - [`query.rs`](src/handlers/query.rs) - contains the msg handlers for the `query` entrypoint
    - [`commands.rs`](src/handlers/execute.rs) - contains the msg handlers for the `execute` entrypoint
    - [`migrate.rs`](src/handlers/migrate.rs) - contains the msg handlers for the `migrate` entrypoint
    - [`reply.rs`](src/handlers/reply.rs) - contains the msg handlers for the `reply` entrypoint
  - [`package`](src/package) - contains the package definitions for the add-on
    - [`state.rs`](src/package/state.rs) - contains the state of the contract
    - [`msg.rs`](src/package/msg.rs) - contains the messages and responses

## Instructions
1. Fork this repository
2. Update the name of the package in [Cargo.toml](Cargo.toml)



## Commands
- `cargo build`: Build the add-on
- `cargo test`: Run the tests
- `cargo wasm`: Compile the add-on to wasm
- `cargo doc`: Generate the documentation
- `cargo clippy`: Run the linter

# Tests
The test cases covered by this dapp are located in [the README file under src/tests/](src/tests/README.md).
