#!/usr/bin/env bash

# replace all occurrences of the templatized variables with their templates
sed -i '' -e 's/"template-app-name"/"{{app-name}}"/g' ./**/Cargo.toml
sed -i '' -e 's/"template_app_name"/"{{app_name}}"/g' ./**/Cargo.toml
sed -i '' -e 's/"template_app_name"/"{{app_name}}"/g' ./**/*.rs
sed -i '' -e 's/"template_namespace"/"{{username}}"/g' ./**/*.rs
sed -i '' -e 's/TemplateMigrateMsg/ {{app_migrate_msg}}/g' ./**/*.rs
sed -i '' -e 's/TemplateExecuteMsg/ {{app_execute_msg}}/g' ./**/*.rs
sed -i '' -e 's/TemplateQueryMsg/ {{app_query_msg}}/g' ./**/*.rs
sed -i '' -e 's/TemplateInstantiateMsg/ {{app_instantiate_msg}}/g' ./**/*.rs
sed -i '' -e 's/TemplateResult/ {{app_result}}/g' ./**/*.rs
sed -i '' -e 's/TemplateError/ {{app_error}}/g' ./**/*.rs
sed -i '' -e 's/TemplateApp/ {{app_contract}}/g' ./**/*.rs
