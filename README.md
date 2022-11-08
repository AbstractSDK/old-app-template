# Abstract Add-On Template
This is a repository for developing an add-on using [Abstract-OS](https://abstract.money).

## Features
- Abstracted handlers for `instantiate`, `query`, `execute`, `migrate`


## File Structure
- [`src`](src) - source code
  - [`contract.rs`](src/contract.rs) - contract implementation with the top-level handlers for `instantiate`, `query`, `execute`, `migrate`
  - [`handlers`](src/handlers) - contains the handlers for the add-on
    - [`queries.rs`](src/handlers/queries.rs) - contains the msg handlers for the `query` entrypoint
    - [`executes.rs`](src/handlers/executes.rs) - contains the msg handlers for the `execute` entrypoint
    - [`replies.rs`](src/handlers/replies.rs) - contains the msg handlers for the `reply` entrypoint
  - [`package`](src/package) - contains the package definitions for the add-on
    - [`state.rs`](src/package/state.rs) - contains the state of the contract
    - [`msg.rs`](src/package/msg.rs) - contains the messages and responses

## Instructions
1. Fork this repository
2. Update the name of the package in (easiest via find/replace "Template" -> "YourAddOnName"):
   1. [Cargo.toml](Cargo.toml)
   2. [src/contract.rs](src/contract.rs)::
      1. `CONTRACT_NAME`
      2. `TemplateAddOn` type -> `<YourAddOnName>AddOn`
      3. `TemplateResult` -> `<YourAddOnName>Result`
   3. [src/package/msg.rs](src/package/msg.rs)
      1. `TemplateError` -> `<YourAddOnName>Error`
      2. `TemplateInstantiateMsg` -> `<YourAddOnName>InstantiateMsg`
      3. `TemplateQueryMsg` -> `<YourAddOnName>QueryMsg`
      4. `TemplateExecuteMsg` -> `<YourAddOnName>ExecuteMsg`
      5. `TemplateReplyMsg` -> `<YourAddOnName>ReplyMsg`
      6. `TemplateMigrateMsg` -> `<YourAddOnName>MigrateMsg`

## Commands
- `cargo build`: Build the add-on
- `cargo test`: Run the tests
- `cargo wasm`: Compile the add-on to wasm
- `cargo doc`: Generate the documentation
- `cargo clippy`: Run the linter

## Tests
The test cases covered by this dapp are located in [the README file under src/tests/](src/tests/README.md).
