use prost_build::Config;

fn main() {
    let mut config = Config::new();

    add_serde_attributes(&mut config);
    add_deprecation_attributes(&mut config);
    add_derive_copy_attributes(&mut config);
    add_clippy_attributes(&mut config);

    config.compile_protos(&["messages.proto"], &["./"]).unwrap();
}

#[cfg(not(feature = "ndjson"))]
fn add_serde_attributes(_config: &mut Config) {}

#[cfg(feature = "ndjson")]
fn add_serde_attributes(config: &mut Config) {
    config.type_attribute(".", "#[derive(serde::Serialize)]");
    config.type_attribute(".", "#[serde(rename_all = \"camelCase\")]");

    flatten_enums(
        config,
        &[
            "Envelope.message",
            "FeatureChild.value",
            "RuleChild.value",
            "Step.argument",
            "Attachment.body",
            "PickleStepArgument.message",
            "CommandActionComplete.result",
            "SourceReference.reference",
        ],
    );
    skip_serializing_empty_options(
        config,
        &[
            "feature",
            "location",
            "argument",
            "timestamp",
            "source_location",
            "action_location",
            "platform",
            "duration",
            "sources_config",
            "runtime_config",
            "support_code_config",
            "TestStepFinished.test_result",
            "Attachment.source",
            "FeatureChild.value",
            "RuleChild.value",
            "Examples.table_header",
            "PickleStepArgument.message",
            "SourcesConfig.filters",
            "SourcesConfig.order",
            "source_reference",
            "StepDefinition.pattern",
            "CommandActionComplete.result",
            "CommandInitializeTestCase.pickle",
            "StepMatchArgument.group",
            "CommandGenerateSnippet.pickle_step_argument",
            "Envelope.message",
        ],
    );
    skip_serializing_empty_vecs(
        config,
        &[
            "comments",
            "children",
            "tags",
            "steps",
            "examples",
            "rows",
            "cells",
            "ast_node_ids",
            "test_steps",
            "step_definition_ids",
            "step_match_arguments_lists",
            "step_match_arguments",
            "absolute_paths",
            "name_regular_expressions",
            "uri_to_lines_mapping",
            "lines",
            "before_test_case_hooks",
            "after_test_case_hooks",
            "step_definitions",
            "parameter_types",
            "Examples.table_body",
            "regular_expressions",
            "generated_expressions",
            "parameter_type_names",
        ],
    );
    skip_serializing_empty_strings(
        config,
        &[
            "language",
            "keyword",
            "name",
            "description",
            "id",
            "media_type",
            "content",
            "delimiter",
            "ast_node_id",
            "test_step_id",
            "test_case_started_id",
            "pickle_id",
            "pickle_step_id",
            "hook_id",
            "step_definition_id",
            "parameter_type_name",
            "test_case_id",
            "action_id",
            "tag_expression",
            "completed_id",
            "absolute_path",
            "data",
            "Attachment.body",
            "Group.value",
            "Platform.implementation",
            "Platform.version",
            "Platform.os",
            "Platform.cpu",
            "Comment.text",
            "Step.text",
            "PickleStep.text",
            "GeneratedExpression.text",
            "TableCell.value",
            "PickleTableCell.value",
            "TestResult.message",
        ],
    );
    skip_serializing_zero_numbers(
        config,
        &[
            "Location.line",
            "Location.column",
            "TestCaseStarted.attempt",
            "Group.start",
        ],
    );
}

#[cfg(feature = "ndjson")]
fn flatten_enums(config: &mut Config, fields: &[&'static str]) {
    for field_path in fields {
        config.field_attribute(field_path, "#[serde(flatten)]");
    }
}

#[cfg(feature = "ndjson")]
fn skip_serializing_empty_options(config: &mut Config, fields: &[&'static str]) {
    for field_path in fields {
        config.field_attribute(
            field_path,
            "#[serde(skip_serializing_if = \"Option::is_none\")]",
        );
    }
}

#[cfg(feature = "ndjson")]
fn skip_serializing_empty_vecs(config: &mut Config, fields: &[&'static str]) {
    for field_path in fields {
        config.field_attribute(
            field_path,
            "#[serde(skip_serializing_if = \"Vec::is_empty\")]",
        );
    }
}

#[cfg(feature = "ndjson")]
fn skip_serializing_empty_strings(config: &mut Config, fields: &[&'static str]) {
    for field_path in fields {
        config.field_attribute(
            field_path,
            "#[serde(skip_serializing_if = \"String::is_empty\")]",
        );
    }
}

#[cfg(feature = "ndjson")]
fn skip_serializing_zero_numbers(config: &mut Config, fields: &[&'static str]) {
    for field_path in fields {
        config.field_attribute(
            field_path,
            "#[serde(skip_serializing_if = \"crate::proto::serde::is_zero\")]",
        );
    }
}

fn add_deprecation_attributes(config: &mut Config) {
    config.type_attribute(
        "TestCasePrepared",
        "#[deprecated(note = \"Please use TestCase instead\")]",
    );
    config.type_attribute(
        "TestCasePreparedStep",
        "#[deprecated(note = \"Please use TestStep instead\")]",
    );
}

fn add_derive_copy_attributes(config: &mut Config) {
    config.type_attribute("Location", "#[derive(Copy)]");
}

fn add_clippy_attributes(config: &mut Config) {
    config.type_attribute("message", "// Cannot box variants in code generation until");
    config.type_attribute(
        "message",
        "// https://github.com/danburkert/prost/issues/13",
    );
    config.type_attribute("message", "// is fixed.");
    config.type_attribute("message", "#[allow(clippy::large_enum_variant)]");
}
