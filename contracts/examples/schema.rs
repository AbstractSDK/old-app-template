use std::env::current_dir;
use std::fs::create_dir_all;

use abstract_addon_template::contract::{{addon_contract}};
use abstract_addon_template::msg::ConfigResponse;
use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    {{addon_contract}}::export_schema(&out_dir);
    export_schema(&schema_for!(ConfigResponse), &out_dir);
}
