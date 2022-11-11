#!/usr/bin/env bash

sed -i '' -e 's/"{{ project-name }}"/"template-addon"/g' ./**/Cargo.toml
sed -i '' -e 's/"{{ project_name }}"/"template_addon"/g' ./**/Cargo.toml
sed -i '' -e 's/{{ addon_migrate_msg }}/TemplateMigrateMsg/g' ./**/*.rs
sed -i '' -e 's/{{ addon_execute_msg }}/TemplateExecuteMsg/g' ./**/*.rs
sed -i '' -e 's/{{ addon_query_msg }}/TemplateQueryMsg/g' ./**/*.rs
sed -i '' -e 's/{{ addon_instantiate_msg }}/TemplateInstantiateMsg/g' ./**/*.rs
sed -i '' -e 's/{{ addon_result }}/TemplateResult/g' ./**/*.rs
sed -i '' -e 's/{{ addon_contract }}/TemplateAddOn/g' ./**/*.rs
sed -i '' -e 's/{{ addon_error }}/TemplateError/g' ./**/*.rs
