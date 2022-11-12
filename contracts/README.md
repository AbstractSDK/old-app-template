# contracts

## File Structure
- [`src`](src) - source code
  - [`contract.rs`](src/contract.rs) - contract implementation with the top-level handlers for `instantiate`, `query`, `execute`, `migrate`
  - [`handlers`](src/handlers) - contains the handlers for the add-on
    - [`instantiate.rs`](src/handlers/instantiate.rs) - contains the msg handlers for the `instantiate` entrypoint
    - [`query.rs`](src/handlers/query.rs) - contains the msg handlers for the `query` entrypoint
    - [`commands.rs`](src/handlers/execute.rs) - contains the msg handlers for the `execute` entrypoint
    - [`migrate.rs`](src/handlers/migrate.rs) - contains the msg handlers for the `migrate` entrypoint
    - [`reply.rs`](src/handlers/reply.rs) - contains the msg handlers for the `reply` entrypoint
  - [`state.rs`](src/package/state.rs) - contains the state of the contract
  - [`msg.rs`](src/package/msg.rs) - contains the messages and responses
