const ADDON_SUFFIX = "_addon";

fn set_var(name, value) {
  variable::set(name, value);
  debug(`${name}: ${value}`);
}

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
set_var("addon_name", addon_name);
// flash-loaner
set_var("addon-name", to_kebab_case(addon_name));

// // flash_loaner
// variable::set("short_addon_name", addon_name.sub_string(0, addon_name.len - ADDON_SUFFIX.len);

let pascal_addon = to_pascal_case(addon_name);

// FlashLoanerAddon
set_var("addon_contract", to_pascal_case(addon_name + ADDON_SUFFIX));

// FlashLoanerResult
set_var("addon_result", pascal_addon + "Result");

// FlashLoanerError
set_var("addon_error", pascal_addon + "Error");

// FlashLoanerExecuteMsg
set_var("addon_execute_msg", pascal_addon + "ExecuteMsg");

// FlashLoanerInstantiateMsg
set_var("addon_instantiate_msg", pascal_addon + "InstantiateMsg");

// FlashLoanerQueryMsg
set_var("addon_query_msg", pascal_addon + "QueryMsg");

// FlashLoanerMigrateMsg
set_var("addon_migrate_msg", pascal_addon + "MigrateMsg");

// rename the script files
// Rename the old readme to usage
file::rename("README.md", "USAGE.md");
// Rename the old usage to readme
file::rename("TEMPLATE.md", "README.md");
