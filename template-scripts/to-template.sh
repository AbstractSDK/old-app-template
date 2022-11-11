#!/usr/bin/env bash

# replace all occurrences of the templatized variables with their templates
sed -i '' -e 's/"template-addon-name"/"{{addon-name}}"/g' ./**/Cargo.toml
sed -i '' -e 's/"template_addon_name"/"{{addon_name}}"/g' ./**/Cargo.toml
sed -i '' -e 's/"template_addon_name"/"{{addon_name}}"/g' ./**/*.rs
sed -i '' -e 's/"template_namespace"/"{{username}}"/g' ./**/*.rs
sed -i '' -e 's/TemplateMigrateMsg/{{addon_migrate_msg}}/g' ./**/*.rs
sed -i '' -e 's/TemplateExecuteMsg/{{addon_execute_msg}}/g' ./**/*.rs
sed -i '' -e 's/TemplateQueryMsg/{{addon_query_msg}}/g' ./**/*.rs
sed -i '' -e 's/TemplateInstantiateMsg/{{addon_instantiate_msg}}/g' ./**/*.rs
sed -i '' -e 's/TemplateResult/{{addon_result}}/g' ./**/*.rs
sed -i '' -e 's/TemplateError/{{addon_error}}/g' ./**/*.rs
sed -i '' -e 's/TemplateAddOn/{{addon_contract}}/g' ./**/*.rs
