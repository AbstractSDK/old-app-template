const APP_SUFFIX = "_app";

fn set_var(name, value) {
  variable::set(name, value);
  debug(`${name}: ${value}`);
}

// Ex: flash-loaner
let project_name = variable::get("project-name");
debug(`project_name: ${project_name}`);

// flash_loaner
let app_name = to_snake_case(project_name);


// check for the app suffix and remove it if present
if app_name.ends_with(APP_SUFFIX) {
  app_name = app_name.sub_string(0, app_name.len - APP_SUFFIX.len);
}


// flash_loaner
set_var("app_name", app_name);
// flash-loaner
set_var("app-name", to_kebab_case(app_name));

// // flash_loaner
// variable::set("short_app_name", app_name.sub_string(0, app_name.len - APP_SUFFIX.len);

let pascal_app = to_pascal_case(app_name);

// FlashLoanerApp
set_var("app_contract", to_pascal_case(app_name + APP_SUFFIX));

// FlashLoanerResult
set_var("app_result", pascal_app + "Result");

// FlashLoanerError
set_var("app_error", pascal_app + "Error");

// FlashLoanerExecuteMsg
set_var("app_execute_msg", pascal_app + "ExecuteMsg");

// FlashLoanerInstantiateMsg
set_var("app_instantiate_msg", pascal_app + "InstantiateMsg");

// FlashLoanerQueryMsg
set_var("app_query_msg", pascal_app + "QueryMsg");

// FlashLoanerMigrateMsg
set_var("app_migrate_msg", pascal_app + "MigrateMsg");

// rename the script files
// Rename the old readme to usage
file::rename("README.md", "USAGE.md");
// Rename the old usage to readme
file::rename("TEMPLATE.md", "README.md");
