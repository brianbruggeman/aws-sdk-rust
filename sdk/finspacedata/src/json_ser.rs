// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_changeset_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateChangesetInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.change_type {
        object.key("changeType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.client_token {
        object.key("clientToken").string(var_2.as_str());
    }
    if let Some(var_3) = &input.format_params {
        let mut object_4 = object.key("formatParams").start_object();
        for (key_5, value_6) in var_3 {
            {
                object_4.key(key_5).string(value_6.as_str());
            }
        }
        object_4.finish();
    }
    if let Some(var_7) = &input.source_params {
        let mut object_8 = object.key("sourceParams").start_object();
        for (key_9, value_10) in var_7 {
            {
                object_8.key(key_9).string(value_10.as_str());
            }
        }
        object_8.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_dataset_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDatasetInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_11) = &input.alias {
        object.key("alias").string(var_11.as_str());
    }
    if let Some(var_12) = &input.client_token {
        object.key("clientToken").string(var_12.as_str());
    }
    if let Some(var_13) = &input.dataset_description {
        object.key("datasetDescription").string(var_13.as_str());
    }
    if let Some(var_14) = &input.dataset_title {
        object.key("datasetTitle").string(var_14.as_str());
    }
    if let Some(var_15) = &input.kind {
        object.key("kind").string(var_15.as_str());
    }
    if let Some(var_16) = &input.owner_info {
        let mut object_17 = object.key("ownerInfo").start_object();
        crate::json_ser::serialize_structure_crate_model_dataset_owner_info(
            &mut object_17,
            var_16,
        )?;
        object_17.finish();
    }
    if let Some(var_18) = &input.permission_group_params {
        let mut object_19 = object.key("permissionGroupParams").start_object();
        crate::json_ser::serialize_structure_crate_model_permission_group_params(
            &mut object_19,
            var_18,
        )?;
        object_19.finish();
    }
    if let Some(var_20) = &input.schema_definition {
        let mut object_21 = object.key("schemaDefinition").start_object();
        crate::json_ser::serialize_structure_crate_model_schema_union(&mut object_21, var_20)?;
        object_21.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_data_view_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDataViewInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_22) = &input.as_of_timestamp {
        object.key("asOfTimestamp").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_22).into()),
        );
    }
    if input.auto_update {
        object.key("autoUpdate").boolean(input.auto_update);
    }
    if let Some(var_23) = &input.client_token {
        object.key("clientToken").string(var_23.as_str());
    }
    if let Some(var_24) = &input.destination_type_params {
        let mut object_25 = object.key("destinationTypeParams").start_object();
        crate::json_ser::serialize_structure_crate_model_data_view_destination_type_params(
            &mut object_25,
            var_24,
        )?;
        object_25.finish();
    }
    if let Some(var_26) = &input.partition_columns {
        let mut array_27 = object.key("partitionColumns").start_array();
        for item_28 in var_26 {
            {
                array_27.value().string(item_28.as_str());
            }
        }
        array_27.finish();
    }
    if let Some(var_29) = &input.sort_columns {
        let mut array_30 = object.key("sortColumns").start_array();
        for item_31 in var_29 {
            {
                array_30.value().string(item_31.as_str());
            }
        }
        array_30.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_permission_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreatePermissionGroupInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_32) = &input.application_permissions {
        let mut array_33 = object.key("applicationPermissions").start_array();
        for item_34 in var_32 {
            {
                array_33.value().string(item_34.as_str());
            }
        }
        array_33.finish();
    }
    if let Some(var_35) = &input.client_token {
        object.key("clientToken").string(var_35.as_str());
    }
    if let Some(var_36) = &input.description {
        object.key("description").string(var_36.as_str());
    }
    if let Some(var_37) = &input.name {
        object.key("name").string(var_37.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_user_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateUserInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_38) = &input.api_access {
        object.key("ApiAccess").string(var_38.as_str());
    }
    if let Some(var_39) = &input.api_access_principal_arn {
        object.key("apiAccessPrincipalArn").string(var_39.as_str());
    }
    if let Some(var_40) = &input.client_token {
        object.key("clientToken").string(var_40.as_str());
    }
    if let Some(var_41) = &input.email_address {
        object.key("emailAddress").string(var_41.as_str());
    }
    if let Some(var_42) = &input.first_name {
        object.key("firstName").string(var_42.as_str());
    }
    if let Some(var_43) = &input.last_name {
        object.key("lastName").string(var_43.as_str());
    }
    if let Some(var_44) = &input.r#type {
        object.key("type").string(var_44.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_disable_user_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DisableUserInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_45) = &input.client_token {
        object.key("clientToken").string(var_45.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_enable_user_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::EnableUserInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_46) = &input.client_token {
        object.key("clientToken").string(var_46.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_working_location_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetWorkingLocationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_47) = &input.location_type {
        object.key("locationType").string(var_47.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_reset_user_password_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ResetUserPasswordInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_48) = &input.client_token {
        object.key("clientToken").string(var_48.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_changeset_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateChangesetInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_49) = &input.client_token {
        object.key("clientToken").string(var_49.as_str());
    }
    if let Some(var_50) = &input.format_params {
        let mut object_51 = object.key("formatParams").start_object();
        for (key_52, value_53) in var_50 {
            {
                object_51.key(key_52).string(value_53.as_str());
            }
        }
        object_51.finish();
    }
    if let Some(var_54) = &input.source_params {
        let mut object_55 = object.key("sourceParams").start_object();
        for (key_56, value_57) in var_54 {
            {
                object_55.key(key_56).string(value_57.as_str());
            }
        }
        object_55.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_dataset_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDatasetInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_58) = &input.alias {
        object.key("alias").string(var_58.as_str());
    }
    if let Some(var_59) = &input.client_token {
        object.key("clientToken").string(var_59.as_str());
    }
    if let Some(var_60) = &input.dataset_description {
        object.key("datasetDescription").string(var_60.as_str());
    }
    if let Some(var_61) = &input.dataset_title {
        object.key("datasetTitle").string(var_61.as_str());
    }
    if let Some(var_62) = &input.kind {
        object.key("kind").string(var_62.as_str());
    }
    if let Some(var_63) = &input.schema_definition {
        let mut object_64 = object.key("schemaDefinition").start_object();
        crate::json_ser::serialize_structure_crate_model_schema_union(&mut object_64, var_63)?;
        object_64.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_permission_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdatePermissionGroupInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_65) = &input.application_permissions {
        let mut array_66 = object.key("applicationPermissions").start_array();
        for item_67 in var_65 {
            {
                array_66.value().string(item_67.as_str());
            }
        }
        array_66.finish();
    }
    if let Some(var_68) = &input.client_token {
        object.key("clientToken").string(var_68.as_str());
    }
    if let Some(var_69) = &input.description {
        object.key("description").string(var_69.as_str());
    }
    if let Some(var_70) = &input.name {
        object.key("name").string(var_70.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_user_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateUserInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_71) = &input.api_access {
        object.key("apiAccess").string(var_71.as_str());
    }
    if let Some(var_72) = &input.api_access_principal_arn {
        object.key("apiAccessPrincipalArn").string(var_72.as_str());
    }
    if let Some(var_73) = &input.client_token {
        object.key("clientToken").string(var_73.as_str());
    }
    if let Some(var_74) = &input.first_name {
        object.key("firstName").string(var_74.as_str());
    }
    if let Some(var_75) = &input.last_name {
        object.key("lastName").string(var_75.as_str());
    }
    if let Some(var_76) = &input.r#type {
        object.key("type").string(var_76.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_dataset_owner_info(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DatasetOwnerInfo,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_77) = &input.name {
        object.key("name").string(var_77.as_str());
    }
    if let Some(var_78) = &input.phone_number {
        object.key("phoneNumber").string(var_78.as_str());
    }
    if let Some(var_79) = &input.email {
        object.key("email").string(var_79.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_permission_group_params(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PermissionGroupParams,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_80) = &input.permission_group_id {
        object.key("permissionGroupId").string(var_80.as_str());
    }
    if let Some(var_81) = &input.dataset_permissions {
        let mut array_82 = object.key("datasetPermissions").start_array();
        for item_83 in var_81 {
            {
                let mut object_84 = array_82.value().start_object();
                crate::json_ser::serialize_structure_crate_model_resource_permission(
                    &mut object_84,
                    item_83,
                )?;
                object_84.finish();
            }
        }
        array_82.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_schema_union(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SchemaUnion,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_85) = &input.tabular_schema_config {
        let mut object_86 = object.key("tabularSchemaConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_schema_definition(&mut object_86, var_85)?;
        object_86.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_data_view_destination_type_params(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DataViewDestinationTypeParams,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_87) = &input.destination_type {
        object.key("destinationType").string(var_87.as_str());
    }
    if let Some(var_88) = &input.s3_destination_export_file_format {
        object
            .key("s3DestinationExportFileFormat")
            .string(var_88.as_str());
    }
    if let Some(var_89) = &input.s3_destination_export_file_format_options {
        let mut object_90 = object
            .key("s3DestinationExportFileFormatOptions")
            .start_object();
        for (key_91, value_92) in var_89 {
            {
                object_90.key(key_91).string(value_92.as_str());
            }
        }
        object_90.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_resource_permission(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ResourcePermission,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_93) = &input.permission {
        object.key("permission").string(var_93.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_schema_definition(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SchemaDefinition,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_94) = &input.columns {
        let mut array_95 = object.key("columns").start_array();
        for item_96 in var_94 {
            {
                let mut object_97 = array_95.value().start_object();
                crate::json_ser::serialize_structure_crate_model_column_definition(
                    &mut object_97,
                    item_96,
                )?;
                object_97.finish();
            }
        }
        array_95.finish();
    }
    if let Some(var_98) = &input.primary_key_columns {
        let mut array_99 = object.key("primaryKeyColumns").start_array();
        for item_100 in var_98 {
            {
                array_99.value().string(item_100.as_str());
            }
        }
        array_99.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_column_definition(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ColumnDefinition,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_101) = &input.data_type {
        object.key("dataType").string(var_101.as_str());
    }
    if let Some(var_102) = &input.column_name {
        object.key("columnName").string(var_102.as_str());
    }
    if let Some(var_103) = &input.column_description {
        object.key("columnDescription").string(var_103.as_str());
    }
    Ok(())
}
