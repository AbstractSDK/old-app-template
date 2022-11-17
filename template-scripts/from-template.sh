#!/usr/bin/env bash

#
sed -i '' -e 's/" {{app-name}}"/"template-app-name"/g' ./**/Cargo.toml
sed -i '' -e 's/" {{app_name}}"/"template_app_name"/g' ./**/Cargo.toml
sed -i '' -e 's/" {{app_name}}"/"template_app_name"/g' ./**/*.rs
sed -i '' -e 's/" {{username}}"/"template_namespace"/g' ./**/*.rs
sed -i '' -e 's/ {{app_migrate_msg}}/TemplateMigrateMsg/g' ./**/*.rs
sed -i '' -e 's/ {{app_execute_msg}}/TemplateExecuteMsg/g' ./**/*.rs
sed -i '' -e 's/ {{app_query_msg}}/TemplateQueryMsg/g' ./**/*.rs
sed -i '' -e 's/ {{app_instantiate_msg}}/TemplateInstantiateMsg/g' ./**/*.rs
sed -i '' -e 's/ {{app_result}}/TemplateResult/g' ./**/*.rs
sed -i '' -e 's/ {{app_error}}/TemplateError/g' ./**/*.rs
sed -i '' -e 's/ {{app_contract}}/TemplateApp/g' ./**/*.rs
