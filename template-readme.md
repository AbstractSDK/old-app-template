# Abstract App Template

`RUSTFLAGS='-C link-arg=-s' cargo wasm`
## Commands
- `./template-scripts/to-template.sh` - takes the valid rust and turns it into the template
- `./template-scripts/from-template.sh` - takes the templated rust and turns it into valid rust

### Updating mainline to the latest template
```shell
git checkout untemplated
./template-scripts/to-template.sh
git checkout mainline
git merge untemplated --squash
git checkout --theirs .
git add .
git commit -m "Update template ..."
```

## Vars
### Explicit
> These vars are either included by default by [`cargo generate`](https://cargo-generate.github.io/cargo-generate/templates/builtin_placeholders.html) or are explicitly defined in the template's [cargo-generate.toml](cargo-generate.toml) file.
- `{{ project-name }}` - the name of the project (e.g. `cron-cat` or `cron-cat-app`)

### Generated
> These are generated from the `project-name` var and if the "app" suffix is present, it is removed to prevent "AppApp"
- `{{ app_name }}` - (e.g. `cron_cat`)
- `{{ app_migrate_msg }}` - (e.g. `CronCatMigrateMsg`)
- `{{ app_migrate_msg }}` - (e.g. `CronCatMigrateMsg`)
- `{{ app_instantiate_msg }}` - (e.g. `CronCatInstantiateMsg`)
- `{{ app_execute_msg }}` - (e.g. `CronCatExecuteMsg`)
- `{{ app_query_msg }}` - (e.g. `CronCatQueryMsg`)
- `{{ app_reply_msg }}` - (e.g. `CronCatReplyMsg`)
- `{{ app_result }}` - (e.g. `CronCatResult`)
- `{{ app_contract }}` - (e.g. `CronCatApp`)

