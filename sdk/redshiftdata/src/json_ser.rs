// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_cancel_statement_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CancelStatementInput,
) {
    if let Some(var_1) = &input.id {
        object.key("Id").string(var_1);
    }
}

pub fn serialize_structure_describe_statement_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeStatementInput,
) {
    if let Some(var_2) = &input.id {
        object.key("Id").string(var_2);
    }
}

pub fn serialize_structure_describe_table_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeTableInput,
) {
    if let Some(var_3) = &input.cluster_identifier {
        object.key("ClusterIdentifier").string(var_3);
    }
    if let Some(var_4) = &input.secret_arn {
        object.key("SecretArn").string(var_4);
    }
    if let Some(var_5) = &input.db_user {
        object.key("DbUser").string(var_5);
    }
    if let Some(var_6) = &input.database {
        object.key("Database").string(var_6);
    }
    if let Some(var_7) = &input.connected_database {
        object.key("ConnectedDatabase").string(var_7);
    }
    if let Some(var_8) = &input.schema {
        object.key("Schema").string(var_8);
    }
    if let Some(var_9) = &input.table {
        object.key("Table").string(var_9);
    }
    if let Some(var_10) = &input.next_token {
        object.key("NextToken").string(var_10);
    }
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
}

pub fn serialize_structure_execute_statement_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ExecuteStatementInput,
) {
    if let Some(var_11) = &input.sql {
        object.key("Sql").string(var_11);
    }
    if let Some(var_12) = &input.cluster_identifier {
        object.key("ClusterIdentifier").string(var_12);
    }
    if let Some(var_13) = &input.secret_arn {
        object.key("SecretArn").string(var_13);
    }
    if let Some(var_14) = &input.db_user {
        object.key("DbUser").string(var_14);
    }
    if let Some(var_15) = &input.database {
        object.key("Database").string(var_15);
    }
    if let Some(var_16) = &input.with_event {
        object.key("WithEvent").boolean(*var_16);
    }
    if let Some(var_17) = &input.statement_name {
        object.key("StatementName").string(var_17);
    }
    if let Some(var_18) = &input.parameters {
        let mut array_19 = object.key("Parameters").start_array();
        for item_20 in var_18 {
            {
                let mut object_21 = array_19.value().start_object();
                crate::json_ser::serialize_structure_sql_parameter(&mut object_21, item_20);
                object_21.finish();
            }
        }
        array_19.finish();
    }
}

pub fn serialize_structure_get_statement_result_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetStatementResultInput,
) {
    if let Some(var_22) = &input.id {
        object.key("Id").string(var_22);
    }
    if let Some(var_23) = &input.next_token {
        object.key("NextToken").string(var_23);
    }
}

pub fn serialize_structure_list_databases_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListDatabasesInput,
) {
    if let Some(var_24) = &input.cluster_identifier {
        object.key("ClusterIdentifier").string(var_24);
    }
    if let Some(var_25) = &input.database {
        object.key("Database").string(var_25);
    }
    if let Some(var_26) = &input.secret_arn {
        object.key("SecretArn").string(var_26);
    }
    if let Some(var_27) = &input.db_user {
        object.key("DbUser").string(var_27);
    }
    if let Some(var_28) = &input.next_token {
        object.key("NextToken").string(var_28);
    }
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
}

pub fn serialize_structure_list_schemas_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListSchemasInput,
) {
    if let Some(var_29) = &input.cluster_identifier {
        object.key("ClusterIdentifier").string(var_29);
    }
    if let Some(var_30) = &input.secret_arn {
        object.key("SecretArn").string(var_30);
    }
    if let Some(var_31) = &input.db_user {
        object.key("DbUser").string(var_31);
    }
    if let Some(var_32) = &input.database {
        object.key("Database").string(var_32);
    }
    if let Some(var_33) = &input.connected_database {
        object.key("ConnectedDatabase").string(var_33);
    }
    if let Some(var_34) = &input.schema_pattern {
        object.key("SchemaPattern").string(var_34);
    }
    if let Some(var_35) = &input.next_token {
        object.key("NextToken").string(var_35);
    }
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
}

pub fn serialize_structure_list_statements_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListStatementsInput,
) {
    if let Some(var_36) = &input.next_token {
        object.key("NextToken").string(var_36);
    }
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_37) = &input.statement_name {
        object.key("StatementName").string(var_37);
    }
    if let Some(var_38) = &input.status {
        object.key("Status").string(var_38.as_str());
    }
    if let Some(var_39) = &input.role_level {
        object.key("RoleLevel").boolean(*var_39);
    }
}

pub fn serialize_structure_list_tables_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTablesInput,
) {
    if let Some(var_40) = &input.cluster_identifier {
        object.key("ClusterIdentifier").string(var_40);
    }
    if let Some(var_41) = &input.secret_arn {
        object.key("SecretArn").string(var_41);
    }
    if let Some(var_42) = &input.db_user {
        object.key("DbUser").string(var_42);
    }
    if let Some(var_43) = &input.database {
        object.key("Database").string(var_43);
    }
    if let Some(var_44) = &input.connected_database {
        object.key("ConnectedDatabase").string(var_44);
    }
    if let Some(var_45) = &input.schema_pattern {
        object.key("SchemaPattern").string(var_45);
    }
    if let Some(var_46) = &input.table_pattern {
        object.key("TablePattern").string(var_46);
    }
    if let Some(var_47) = &input.next_token {
        object.key("NextToken").string(var_47);
    }
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
}

pub fn serialize_structure_sql_parameter(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SqlParameter,
) {
    if let Some(var_48) = &input.name {
        object.key("name").string(var_48);
    }
    if let Some(var_49) = &input.value {
        object.key("value").string(var_49);
    }
}