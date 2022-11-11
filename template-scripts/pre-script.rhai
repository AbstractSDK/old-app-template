const ADDON_SUFFIX = "_addon";

// Ex: flash-loaner
let project_name = variable::get("project-name");
debug(`project_name: ${project_name}`);

// flash_loaner
let addon_name = to_snake_case(project_name);


// check for the addon suffix and remove it if present
if addon_name.ends_with(ADDON_SUFFIX) {
  addon_name = addon_name.sub_string(0, addon_name.len - ADDON_SUFFIX.len);
}

// flash_loaner
variable::set("addon_name", addon_name);
// flash-loaner
variable::set("addon-name", to_kebab_case(addon_name));

// // flash_loaner
// variable::set("short_addon_name", addon_name.sub_string(0, addon_name.len - ADDON_SUFFIX.len);

let pascal_addon = to_pascal_case(addon_name);
variable::set("pascal_addon", pascal_addon);
debug(`pascal_addon: ${pascal_addon}`);

// FlashLoanerAddon
let addon_contract = to_pascal_case(addon_name + ADDON_SUFFIX);
variable::set("addon_contract", addon_contract);
debug(`addon_contract: ${addon_contract}`);

// FlashLoanerResult
let addon_result_name = pascal_addon + "Result";
variable::set("addon_result_name", addon_result_name);
debug(`addon_result_name: ${addon_result_name}`);

// FlashLoanerError
let addon_error = pascal_addon + "Error";
variable::set("addon_error", addon_error);
debug(`addon_error: ${addon_error}`);

// FlashLoanerExecuteMsg
let addon_execute_msg = pascal_addon + "ExecuteMsg";
variable::set("addon_execute_msg", addon_execute_msg);
debug(`addon_execute_msg: ${addon_execute_msg}`);

// FlashLoanerInstantiateMsg
let addon_instantiate_msg = pascal_addon + "InstantiateMsg";
variable::set("addon_instantiate_msg", addon_instantiate_msg);
debug(`addon_instantiate_msg: ${addon_instantiate_msg}`);

// FlashLoanerQueryMsg
let addon_query_msg = pascal_addon + "QueryMsg";
variable::set("addon_query_msg", addon_query_msg);
debug(`addon_query_msg: ${addon_query_msg}`);

// FlashLoanerMigrateMsg
let addon_migrate_msg = pascal_addon + "MigrateMsg";
variable::set("addon_migrate_msg", addon_migrate_msg);
debug(`addon_migrate_msg: ${addon_migrate_msg}`);

// rename the script files
