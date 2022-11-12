# Abstract Addon Template

`RUSTFLAGS='-C link-arg=-s' cargo wasm`
## Commands
- `./template-scripts/to-template.sh` - takes the valid rust and turns it into the template
- `./template-scripts/from-template.sh` - takes the templated rust and turns it into valid rust

## Vars
### Explicit
> These vars are either included by default by [`cargo generate`](https://cargo-generate.github.io/cargo-generate/templates/builtin_placeholders.html) or are explicitly defined in the template's [cargo-generate.toml](cargo-generate.toml) file.
- `{{ project-name }}` - the name of the project (e.g. `cron-cat` or `cron-cat-addon`)

### Generated
> These are generated from the `project-name` var and if the "addon" suffix is present, it is removed to prevent "AddonAddon"
- `{{ addon_name }}` - (e.g. `cron_cat`)
- `{{ addon_migrate_msg }}` - (e.g. `CronCatMigrateMsg`)
- `{{ addon_migrate_msg }}` - (e.g. `CronCatMigrateMsg`)
- `{{ addon_instantiate_msg }}` - (e.g. `CronCatInstantiateMsg`)
- `{{ addon_execute_msg }}` - (e.g. `CronCatExecuteMsg`)
- `{{ addon_query_msg }}` - (e.g. `CronCatQueryMsg`)
- `{{ addon_reply_msg }}` - (e.g. `CronCatReplyMsg`)
- `{{ addon_result }}` - (e.g. `CronCatResult`)
- `{{ addon_contract }}` - (e.g. `CronCatAddon`)

